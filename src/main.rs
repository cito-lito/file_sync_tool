use notify::{RecommendedWatcher, RecursiveMode, Result, Watcher, Config};
use std::path::PathBuf;
use std::sync::mpsc::channel;

fn main() -> Result<()> {

    // config  with debounce duration
    let config = Config::default(); // default polling 2s
    let path_to_watch = PathBuf::from("./");

    // channel to receive file events
    let (tx, rx) = channel();

    let mut watcher: RecommendedWatcher = Watcher::new(tx, config)?;

    // Add a path to be watched. All files and directories at that path, recursively.
    watcher.watch(&path_to_watch, RecursiveMode::Recursive)?;

    // log the file events for now
    for event in rx {
        println!("{:?}", event);
    }
    Ok(())
}
