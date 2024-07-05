use std::{env, thread};
use std::time::Duration;
use testcontainers::{Container, core::WaitFor, GenericImage, ImageExt, runners::SyncRunner};
use testcontainers::core::{ExecCommand, Mount};

use banscreen::gateways::{ListWindowsWindowSystemGateway, ScreenShotWindowSystemGateway};
use banscreen::window_system::x11_dl_window_system_adapter::X11DLWindowSystemAdapter;

#[test]
fn test_should_list_multiple_windows() {
    // Given
    let container = run_xvfb_container();
    start_feh_process(&container, "window1", 1);
    start_feh_process(&container, "bbbb", 2);
    start_feh_process(&container, "window3", 3);
    let sut = X11DLWindowSystemAdapter::new()
        .expect("Unable to create the system under test");

    // When
    let windows = sut.list_windows().expect("Failed to list windows");

    // Then
    assert_eq!(windows, vec!["window1", "bbbb", "window3"]);
}

#[test]
fn test_should_take_a_screenshot_among_multiple_windows() {
    // Given
    let container = run_xvfb_container();
    start_feh_process(&container, "window1", 1);
    start_feh_process(&container, "bbbb", 2);
    start_feh_process(&container, "window3", 3);
    let sut = X11DLWindowSystemAdapter::new()
        .expect("Unable to create the system under test");

    // When
    let window_id = sut.find_window(&"bbbb".to_string()).unwrap().unwrap();
    let windows = sut.take_screen_shot(window_id).expect("Failed to list windows");
}

fn run_xvfb_container() -> Container<GenericImage> {
    env::set_var("DISPLAY", ":99");
    let container = GenericImage::new("xvfb-alpine", "latest")
        .with_wait_for(WaitFor::message_on_stdout("Openbox-Debug: Moving to desktop 1"))
        .with_mount(Mount::bind_mount("/tmp/.X11-unix", "/tmp/.X11-unix"))
        .with_mount(Mount::bind_mount("test_images", "/images"))
        .start()
        .expect("Unable to start xvfb container");
    container
}

fn start_feh_process(container: &Container<GenericImage>, title: &str, image_number: u32) {
    let command = format!("feh --title {} /images/{}.jpg &", title, image_number);
    container.exec(ExecCommand::new(
        vec!["sh", "-c", command.as_str()]))
        .expect("Unable to run the feh command");
    // Hey, As Long As It Works.
    thread::sleep(Duration::from_millis(100));
}
