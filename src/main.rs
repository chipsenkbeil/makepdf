use anyhow::Context;
use clap::{Parser, Subcommand};
use makepdf::{Engine, PdfConfig, PdfConfigPage, PdfConfigPlanner};

#[derive(Debug, Parser)]
#[command(version, about, long_about = None)]
struct Cli {
    #[command(subcommand)]
    command: Commands,
}

#[derive(Debug, Subcommand)]
enum Commands {
    /// Construct a PDF using a Luau (https://luau.org/) script, which is also compatible with Lua
    /// 5.1.
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
        #[arg(short, long, default_value_t = PdfConfigPage::default().to_px_size_string())]
        dimensions: String,

        /// DPI to use for the created PDF.
        #[arg(long, default_value_t = PdfConfigPage::default().dpi)]
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
        /// `makepdf:{NAME}` where the name is prefixed with `makepdf:`.
        #[arg(short, long, default_value_t = PdfConfig::default().script)]
        script: String,

        /// Year to associate when running the PDF generation script.
        #[arg(long, default_value_t = PdfConfig::default().title)]
        title: String,

        /// Year to associate when running the PDF generation script.
        #[arg(long, default_value_t = PdfConfigPlanner::default().year)]
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
            title,
            year,
        } => {
            // Translate our dimensions into a width and height we will use for the PDF pages
            let (width, height) = PdfConfigPage::parse_size(&dimensions, dpi)?;

            // Build our initial configuration based on the commandline arguments and defaults
            let config = PdfConfig {
                page: PdfConfigPage {
                    dpi,
                    font,
                    width,
                    height,
                    ..Default::default()
                },
                planner: PdfConfigPlanner {
                    year,
                    ..Default::default()
                },
                title,
                script,
            };

            // Do the actual process of
            //
            // 1. Creating an engine for the given configuration
            // 2. Setup the configuration by running a Lua script to modify it
            // 3. Run post-script hooks that will create internal pages & objects
            // 4. Translate the internal pages & objects into the actual PDF
            // 5. Save the PDF to disk
            Engine::new(config)
                .setup()
                .context("Failed to setup PDF engine")?
                .run_hooks()
                .context("Failed to run PDF hooks")?
                .build()
                .context("Failed to build PDF")?
                .save(&output)
                .context("Failed to save PDF to file")?;

            // If indicated, we try to open the PDF automatically
            if open {
                opener::open(&output).with_context(|| format!("Failed to open {output}"))?;
            }

            Ok(())
        }
    }
}
