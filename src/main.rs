use notify::{Config, RecommendedWatcher, RecursiveMode, Result, Watcher};
use std::path::PathBuf;
use std::sync::mpsc::channel;

fn main() -> Result<()> {
    let config = Config::default(); // default polling 2s

    let path_to_watch = PathBuf::from("./");

    let (tx, rx) = channel();

    let mut watcher = match RecommendedWatcher::new(tx, config) {
        Ok(watcher) => watcher,
        Err(e) => {
            eprintln!("Error creating watcher: {:?}", e);
            std::process::exit(1);
        }
    };

    // Add a path to be watched. All files and directories at that path, recursively.
   if let Err(e) =  watcher.watch(&path_to_watch, RecursiveMode::Recursive){
        eprintln!("Error watching path: {:?}", e);
        std::process::exit(1);
   }

    // log the file events for now
    for event in rx {
        match event {
            Ok(event) => handle_event(event),
            Err(e) => eprintln!("watch error: {:?}", e),
        }
    }
    Ok(())
}

fn handle_event(event: notify::Event){
    //do smth
    println!("{:?}", event)
}