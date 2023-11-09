use std::path::PathBuf;
mod path_watcher;

fn main() -> Result<(), Box<dyn std::error::Error>> {
    let path_to_watch = PathBuf::from("./");

    let (_watcher, event_rx) = path_watcher::watch_path(path_to_watch)?;

    for event in event_rx {
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