use banscreen::fs::image_module_file_system_adapter::ImageModuleFileSystemAdapter;
use banscreen::take_screen_shot_usecase::TakeScreenShotUseCase;
use banscreen::window_system::x11_dl_window_system_adapter::X11DLWindowSystemAdapter;
use clap::Parser;

#[derive(Parser, Debug)]
#[command(name = "banscreen")]
#[command(version = "0.0.0")]
#[command(about = "Screenshots a window given its title")]
#[command(long_about = "Does a screenshot of the given window (finds it by the given title) and saves it to the given file.")]
struct Args {
    #[arg(short, long)]
    window_title: String,
    
    #[arg(short, long)]
    output_file: String,
}

fn main() {
    let args = Args::parse();
    
    let mut usecase = TakeScreenShotUseCase::new(
        Box::new(X11DLWindowSystemAdapter::new().expect("Unable to create X11DLWindowSystemAdapter.")),
        Box::new(ImageModuleFileSystemAdapter::new()),
    );

    usecase.take_screenshot(
        args.window_title,
        args.output_file,
    ).expect("Unable to take screenshot.");
}
