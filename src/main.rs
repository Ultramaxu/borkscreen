use banscreen::fs::image_module_file_system_adapter::ImageModuleFileSystemAdapter;
use banscreen::take_screen_shot_usecase::TakeScreenShotUseCase;
use banscreen::window_system::x11_dl_window_system_adapter::X11DLWindowSystemAdapter;
use clap::{Parser, Subcommand};
use banscreen::list_windows_usecase::ListWindowsUseCase;

#[derive(Parser)]
#[command(name = "banscreen")]
#[command(version = "0.0.0")]
#[command(about = "Screenshots a window given its title")]
#[command(long_about = "Does a screenshot of the given window (finds it by the given title) and saves it to the given file.")]
struct Cli {
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

    match &cli.command {
        Commands::Capture { window_title, output_file } => {
            let mut usecase = TakeScreenShotUseCase::new(
                Box::new(X11DLWindowSystemAdapter::new().expect("Unable to create X11DLWindowSystemAdapter.")),
                Box::new(ImageModuleFileSystemAdapter::new()),
            );
            usecase.take_screenshot(
                window_title.to_string(),
                output_file.to_string(),
            ).expect("Unable to take screenshot.");
        },
        Commands::List => {
            let usecase = ListWindowsUseCase::new(
                Box::new(X11DLWindowSystemAdapter::new().expect("Unable to create X11DLWindowSystemAdapter.")),
            );
            let response = usecase.execute().expect("Unable to list winodws.");
            for window in response {
                println!("{}", window);
            }
        }
    }
}
