use notify::FsEventWatcher;
use notify::{RecursiveMode, Watcher};
use notify_debouncer_full::{new_debouncer, DebounceEventResult, Debouncer, FileIdMap};
use std::path::PathBuf;
use std::sync::mpsc::{channel, Receiver};
use std::time::Duration;

pub fn watch_path_with_debouncer(
    path_to_watch: PathBuf,
) -> Result<
    (
        Debouncer<FsEventWatcher, FileIdMap>,
        Receiver<DebounceEventResult>,
    ),
    std::io::Error,
> {
    let (tx, rx) = channel();
    let mut debouncer = new_debouncer(Duration::from_secs(2), None, tx).expect("new_debouncer");

    debouncer
        .watcher()
        .watch(&path_to_watch, RecursiveMode::Recursive)
        .expect("watch");
    debouncer
        .cache()
        .add_root(&path_to_watch, RecursiveMode::Recursive);

    Ok((debouncer, rx))
}
