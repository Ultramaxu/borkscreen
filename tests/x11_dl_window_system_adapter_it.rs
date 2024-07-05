use std::{env, thread};
use std::io::BufRead;
use std::time::Duration;
use testcontainers::{Container, core::WaitFor, GenericImage, ImageExt, runners::SyncRunner};
use testcontainers::core::{ExecCommand, IntoContainerPort, Mount};

use borkscreen::gateways::{ListWindowsWindowSystemGateway, ScreenShotWindowSystemGateway};
use borkscreen::window_system::x11_dl_window_system_adapter::X11DLWindowSystemAdapter;

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
    let actual = sut.take_screen_shot(window_id).expect("Failed to list windows");

    // Then
    let expected_image_path = format!("{}/tests/test_images/2.png", env::current_dir().unwrap().display());
    let expected = image::open(expected_image_path).expect("Could not find test-image").into_rgb8();
    let result = image_compare::rgb_hybrid_compare(&actual, &expected).expect("Images had different dimensions");
    assert!(result.score >= 0.9, "similarity score = {}", result.score);
}

#[test]
fn test_should_return_none_if_window_cannot_be_found() {
    // Given
    let _container = run_xvfb_container();
    let sut = X11DLWindowSystemAdapter::new()
        .expect("Unable to create the system under test");
    
    // When
    let window_id = sut.find_window(&"bbbb".to_string()).unwrap();
    
    // Then
    assert_eq!(window_id, None);
}

fn run_xvfb_container() -> Container<GenericImage> {
    env::set_var("DISPLAY", "127.0.0.1:99.0");
    let image_mount_dir = format!("{}/tests/test_images", env::current_dir().unwrap().display());
    let container = GenericImage::new("ultramaxu/ultramaxu-homelab-xvfb-alpine", "0.0.0")
        .with_wait_for(WaitFor::message_on_stdout("Openbox-Debug: Moving to desktop 1"))
        .with_mapped_port(6099, 6099.tcp())
        .with_mount(Mount::bind_mount(image_mount_dir, "/images"))
        .start()
        .expect("Unable to start xvfb container");
    container
}

fn start_feh_process(container: &Container<GenericImage>, title: &str, image_number: u32) {
    let command = format!("feh --title {} /images/{}.png &", title, image_number);
    let mut result = container.exec(ExecCommand::new(
        vec!["sh", "-c", command.as_str()]))
        .expect("Unable to run the feh command");
    for line in result.stdout().lines() {
        println!("[STD OUT] {}", line.unwrap_or("[EMPTY LINE]".to_string()));
    }
    for line in result.stderr().lines() {
        println!("[STD ERR] {}", line.unwrap_or("[EMPTY LINE]".to_string()));
    }
    // Hey, As Long As It Works.
    // More seriously, one has to await a bit for the feh process to actually register the window
    // in xvfb. Even awaiting (polling) with tools like xdotool was a fruitless endeavour.
    // Until one thinks of a better solution, this will have to do.
    thread::sleep(Duration::from_millis(100));
}
