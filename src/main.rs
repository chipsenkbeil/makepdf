use anyhow::Context;
use clap::{Parser, Subcommand};
use log::*;
use makepdf::{PdfConfig, PdfConfigPage, Runtime};
use simplelog::*;
use std::fs::File;

#[derive(Debug, Parser)]
#[command(version, about, long_about = None)]
struct Cli {
    #[command(subcommand)]
    command: Commands,

    /// Path to the log file to create and populate
    #[arg(global = true, long, default_value_t = String::from("makepdf.log"))]
    log_file: String,

    /// If specified, suppresses all output
    #[arg(global = true, short, long)]
    quiet: bool,

    /// Level of verbosity with -v showing info, -vv showing debug, and -vvv showing trace
    #[arg(global = true, short, long, action = clap::ArgAction::Count)]
    verbose: u8,
}

#[derive(Debug, Subcommand)]
enum Commands {
    /// Construct a PDF using a Luau (https://luau.org/) script, which is also compatible with Lua
    /// 5.1.
    Make {
        /// Dimensions (WIDTHxHEIGHT) to use for the PDF output,
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
        /// Note that the DPI will influence conversion rates from pixels to PDF millimeters.
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
        ///
        /// When no output provided, will use the title as the filename.
        #[arg(short, long)]
        output: Option<String>,

        /// Path to the script to use to build the PDF.
        #[arg(short, long, default_value_t = PdfConfig::default().script)]
        script: String,

        /// Title of the PDF document.
        #[arg(long, default_value_t = PdfConfig::default().title)]
        title: String,
    },
}

fn main() -> anyhow::Result<()> {
    let cli = Cli::parse();
    init_logger(&cli)?;
    do_main(cli)
}

fn init_logger(cli: &Cli) -> anyhow::Result<()> {
    // Figure out log level for the terminal, defaulting to warn and above
    let term_log_level_filter = match (cli.quiet, cli.verbose) {
        (true, _) => LevelFilter::Off,
        (false, 0) => LevelFilter::Info,
        (false, 1) => LevelFilter::Debug,
        (false, _) => LevelFilter::Trace,
    };

    // Figure out log level for the file, defaulting to info and above
    let write_log_level_filter = match cli.verbose {
        0 => LevelFilter::Info,
        1 => LevelFilter::Debug,
        _ => LevelFilter::Trace,
    };

    CombinedLogger::init(vec![
        TermLogger::new(
            term_log_level_filter,
            Config::default(),
            TerminalMode::Mixed,
            ColorChoice::Auto,
        ),
        WriteLogger::new(
            write_log_level_filter,
            Config::default(),
            File::create(&cli.log_file).context("Failed to create log file")?,
        ),
    ])
    .context("Failed to initialize logger")
}

fn do_main(cli: Cli) -> anyhow::Result<()> {
    match cli.command {
        Commands::Make {
            dimensions,
            dpi,
            font,
            open,
            output,
            script,
            title,
        } => {
            // Translate our dimensions into a width and height we will use for the PDF pages
            let (width, height) = PdfConfigPage::parse_size(&dimensions, dpi)?;

            // If output is not specified, we will use the title with a .pdf extension
            let output = output.unwrap_or_else(|| {
                format!("{}.pdf", title.replace(|c: char| !c.is_alphanumeric(), "_"))
            });

            // Build our initial configuration based on the commandline arguments and defaults
            let config = PdfConfig {
                page: PdfConfigPage {
                    dpi,
                    font,
                    width,
                    height,
                    ..Default::default()
                },
                title,
                script,
            };

            // Do the actual process of
            //
            // 1. Creating a runtime for the given configuration
            // 2. Setup the configuration by running a Lua script to modify it
            // 3. Translate the internal pages & objects into the actual PDF
            // 4. Save the PDF to disk
            Runtime::new(config)
                .setup()
                .context("Failed to setup PDF")?
                .build()
                .context("Failed to build PDF")?
                .save(&output)
                .context("Failed to save PDF to file")?;

            // If indicated, we try to open the PDF automatically
            if open {
                info!("Opening {output}");
                opener::open(&output).with_context(|| format!("Failed to open {output}"))?;
            }

            Ok(())
        }
    }
}
