use banscreen::fs::image_module_file_system_adapter::ImageModuleFileSystemAdapter;
use banscreen::take_screen_shot_usecase::TakeScreenShotUseCase;
use banscreen::window_system::x11_dl_window_system_adapter::X11DLWindowSystemAdapter;
use clap::{Parser, Subcommand, ValueEnum};
use banscreen::gateways::PresenterGateway;
use banscreen::list_windows_usecase::ListWindowsUseCase;
use banscreen::presenter::Presenter;
use banscreen::presenter_adapter::plain_text_presenter_adapter::PlainTextPresenterAdapter;
use banscreen::presenter_adapter::serde_presenter_adapter::SerdePresenterAdapter;

#[derive(ValueEnum, Copy, Clone, Debug, PartialEq, Eq)]
enum OutputFormat {
    PlainText,
    Json,
}

#[derive(Parser)]
#[command(name = "banscreen")]
#[command(version = "0.0.0")]
#[command(about = "Screenshots a window given its title")]
#[command(long_about = "Does a screenshot of the given window (finds it by the given title) and saves it to the given file.")]
struct Cli {
    #[clap(long, default_value_t = OutputFormat::PlainText, value_enum)]
    output_format: OutputFormat,
    
    #[command(subcommand)]
    command: Commands,
}

#[derive(Subcommand)]
enum Commands {
    /// Captures a screenshot of a window given its title
    Capture {
        #[arg(short, long)]
        window_title: String,

        #[arg(short, long)]
        output_file: String,
    },
    /// Lists all windows
    List,
}

fn main() {
    let cli = Cli::parse();
    let presenter_adapter: Box<dyn PresenterGateway> = match cli.output_format { 
        OutputFormat::PlainText => Box::new(PlainTextPresenterAdapter::new()),
        OutputFormat::Json => Box::new(SerdePresenterAdapter::new())
    };
    let presenter = Presenter::new(presenter_adapter);

    let command_result = match &cli.command {
        Commands::Capture { window_title, output_file } => {
            let mut usecase = TakeScreenShotUseCase::new(
                Box::new(X11DLWindowSystemAdapter::new().expect("Unable to create X11DLWindowSystemAdapter.")),
                Box::new(ImageModuleFileSystemAdapter::new()),
            );
            usecase.take_screenshot(
                window_title.to_string(),
                output_file.to_string(),
            )
        }
        Commands::List => {
            let usecase = ListWindowsUseCase::new(
                Box::new(X11DLWindowSystemAdapter::new().expect("Unable to create X11DLWindowSystemAdapter.")),
            );
            usecase.execute()
        }
    };
    presenter.present(&command_result).expect("Unable to present command result.");
    
    match command_result { 
        Ok(_) => std::process::exit(0),
        Err(_) => std::process::exit(1),
    }
}