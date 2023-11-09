use notify::{Config, Event, RecommendedWatcher, RecursiveMode, Result, Watcher};
use std::path::PathBuf;
use std::sync::mpsc::{channel, Receiver};

pub fn watch_path(path_to_watch: PathBuf) -> Result<(RecommendedWatcher, Receiver<Result<Event>>)> {
    let config = Config::default();
    let (tx, rx) = channel();

    let mut watcher = RecommendedWatcher::new(tx, config).map_err(|error| {
        eprintln!("Error creating watcher: {:?}", error);
        error
    })?;

    watcher
        .watch(&path_to_watch, RecursiveMode::Recursive)
        .map_err(|error| {
            eprintln!("Error watching path: {:?}", error);
            error
        })?;

    Ok((watcher, rx))
}
