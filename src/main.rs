use anyhow::Context;
use clap::{Parser, Subcommand};
use mpdf::{PagePdfConfig, Pdf, PdfConfig, PlannerPdfConfig};

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
        #[arg(short, long, default_value_t = PagePdfConfig::default().to_px_size_string())]
        dimensions: String,

        /// DPI to use for the created PDF.
        #[arg(long, default_value_t = PagePdfConfig::default().dpi)]
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
        /// `mpdf:{NAME}` where the name is prefixed with `mpdf:`.
        #[arg(short, long, default_value_t = PdfConfig::default().script)]
        script: String,

        /// Year to associate when running the PDF generation script.
        #[arg(long, default_value_t = PlannerPdfConfig::default().year)]
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
            //Planner::build(PlannerConfig {
            //    year,
            //    dimensions: PlannerDimensions::from_str(&dimensions, dpi)?,
            //    dpi,
            //    font,
            //    script,
            //})
            //.context("Failed to build PDF")?
            //.save(&output)
            //.context("Failed to save planner to file")?;

            // If indicated, we try to open the PDF automatically
            if open {
                opener::open(&output).with_context(|| format!("Failed to open {output}"))?;
            }

            Ok(())
        }
    }
}
