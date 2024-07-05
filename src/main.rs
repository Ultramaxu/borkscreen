use banscreen::fs::image_module_file_system_adapter::ImageModuleFileSystemAdapter;
use banscreen::take_screen_shot_usecase::TakeScreenShotUseCase;
use banscreen::window_system::x11_dl_window_system_adapter::X11DLWindowSystemAdapter;

fn main() {
    let mut usecase = TakeScreenShotUseCase::new(
        Box::new(X11DLWindowSystemAdapter::new().expect("Unable to create X11DLWindowSystemAdapter.")),
        Box::new(ImageModuleFileSystemAdapter::new()),
    );

    usecase.take_screenshot(
        "~ : zsh — Konsole".to_string(),
        "screenshot.png".to_string(),
    ).expect("Unable to take screenshot.");
}
