use anyhow::Context;
use clap::{Parser, Subcommand};
use lpdf::{units, Planner, PlannerConfig};

#[derive(Debug, Parser)]
#[command(version, about, long_about = None)]
struct Cli {
    #[command(subcommand)]
    command: Commands,
}

#[derive(Debug, Subcommand)]
enum Commands {
    /// Construct a PDF using a Luau (https://luau.org/) script.
    Make {
        /// Dimensions (WIDTHxHEIGHT to use for the PDF output,
        /// defaulting to the Supernote A6 X2 Nomad.
        ///
        /// Can be in one of the following formats:
        ///
        /// 1. `{WIDTH}x{HEIGHT}in` for inches
        ///
        /// 2. `{WIDTH}x{HEIGHT}mm` for millimeters
        ///
        /// 3. `{WIDTH}x{HEIGHT}px` for pixels
        ///
        /// Note the the DPI will influence conversion rates from pixels to
        /// PDF millimeters.
        #[arg(short, long, default_value_t = String::from("1404x1872px"))]
        dimensions: String,

        /// DPI to use for the created PDF.
        #[arg(long, default_value_t = 300.0)]
        dpi: f32,

        /// Path to custom font to use in place of the default Jetbrains Mono font.
        #[arg(long)]
        font: Option<String>,

        /// If specified, will open the PDF after it is created using the system-default method.
        #[arg(long)]
        open: bool,

        /// Destination for the created PDF file.
        #[arg(short, long, default_value_t = String::from("planner.pdf"))]
        output: String,

        /// Path to the script to use to build the PDF.
        ///
        /// Internal scripts are referenced using a special syntax of
        /// `lpdf:{NAME}` where the name is prefixed with `lpdf:`.
        #[arg(short, long, default_value_t = String::from("lpdf:panda"))]
        script: String,

        /// Year to associate when running the PDF generation script.
        #[arg(long, default_value_t = 2024)]
        year: i32,
    },
}

fn main() -> anyhow::Result<()> {
    let cli = Cli::parse();

    match cli.command {
        Commands::Make {
            dimensions,
            dpi,
            font,
            open,
            output,
            script,
            year,
        } => {
            Planner::build(PlannerConfig {
                year,
                dimensions: dimensions_from_str(&dimensions, dpi)?,
                dpi,
                font,
                script,
            })
            .context("Failed to build PDF")?
            .save(&output)
            .context("Failed to save planner to file")?;

            // If indicated, we try to open the PDF automatically
            if open {
                opener::open(&output).with_context(|| format!("Failed to open {output}"))?;
            }

            Ok(())
        }
    }
}

/// Parse a string into dimensions `(width, height)`, supporting the following formats:
///
/// 1. `{WIDTH}x{HEIGHT}in` for inches
/// 2. `{WIDTH}x{HEIGHT}mm` for millimeters
/// 3. `{WIDTH}x{HEIGHT}px` for pixels
pub fn dimensions_from_str(s: &str, dpi: f32) -> anyhow::Result<(units::Mm, units::Mm)> {
    if s.len() < 2 {
        anyhow::bail!("Missing dimension units");
    }

    let s = s.to_lowercase();
    let (s, units) = s.split_at(s.len() - 2);
    let (width, height) = s.split_once('x').ok_or(anyhow::anyhow!(
        "Missing 'x' separator between dimension width & height"
    ))?;
    let width: f32 = width
        .trim()
        .parse()
        .map_err(|_| anyhow::anyhow!("Invalid dimension width! Must be numeric."))?;
    let height: f32 = height
        .trim()
        .parse()
        .map_err(|_| anyhow::anyhow!("Invalid dimension height! Must be numeric."))?;

    match units.trim() {
        // 1 in -> 25.4 mm
        "in" => Ok((units::Mm(width * 25.4), units::Mm(height * 25.4))),
        // mm is straight conversion
        "mm" => Ok((units::Mm(width), units::Mm(height))),
        // px -> pt (using DPI) -> mm
        "px" => Ok((
            units::Px(width as usize).into_pt(dpi).into(),
            units::Px(height as usize).into_pt(dpi).into(),
        )),
        // if we get a blank, still an error
        "" => Err(anyhow::anyhow!("Missing dimension units")),
        // otherwise, got something unexpected and should fail
        _ => Err(anyhow::anyhow!("Unknown dimension units")),
    }
}
