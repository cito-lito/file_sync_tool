use file_sync_tool::path_watcher;
use std::fs::{self, File};
use std::io::{self};
use std::path::PathBuf;
use std::time::Duration;

struct TestFile {
    path: PathBuf,
}

impl Drop for TestFile {
    fn drop(&mut self) {
        let _ = fs::remove_file(&self.path);
    }
}

#[test]
fn test_file_create_event() -> io::Result<()> {
    let test_dir = PathBuf::from("./tests");
    let test_file = test_dir.join("test_file.txt");

    let (_watcher, rx) =
        path_watcher::watch_path(test_dir.clone()).expect("Failed to start watch path");

    // trigger event by creating a file
    let mut _file: File = File::create(&test_file).expect("Failed to create test file");

    println!("total events: {:?}", rx);

    let timeout = Duration::from_secs(3);
    let received_event = rx.recv_timeout(timeout).expect("No event received");

    println!("Event: {:?}", received_event);

    //ensure deleting the test file
    let _ = fs::remove_file(&test_file);

    // check that an event was received
    assert!(received_event.is_ok(), "Failed to receive file event");

    Ok(())
}
