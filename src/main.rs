use notify::{
    event::{self, CreateKind},
    Event, EventKind,
};
use std::path::PathBuf;
mod path_watcher;

fn main() -> Result<(), Box<dyn std::error::Error>> {
    let path_to_watch = PathBuf::from("./tests/from");
    println!("SOURCE path: {:?}", path_to_watch);

    let (_watcher, event_rx) = path_watcher::watch_path(path_to_watch)?;

    for event in event_rx {
        match event {
            Ok(event) => handle_event(event),
            Err(e) => eprintln!("watch error: {:?}", e),
        }
    }

    Ok(())
}

fn handle_event(event: Event) {
    match event.kind {
        EventKind::Create(event_kind) => handle_create_event(event_kind, event.paths),
        EventKind::Modify(_) => {
            println!("MODIFY {:?}", event);
        }
        EventKind::Remove(_) => {
            println!("REMOVE {:?}", event);
        }
        _ => {
            println!("UNHANDLED{:?}", event);
        }
    }
}

fn handle_create_event(event_kind: event::CreateKind, paths: Vec<PathBuf>) {
    for path in paths {
        match event_kind {
            CreateKind::File => {
                if let Some(file_path) = path.to_str() {
                    // cpy file to dest?
                    copy_file_to_dest(file_path);
                }
            }
            _ => {
                println!("UNHANDLED create event {:?}", event_kind);
            }
        }
    }
}

fn copy_file_to_dest(file_path: &str) {
    println!("COPYING file {:?}", file_path);
    let dest_path = PathBuf::from("./tests/to");

    if let Some(file_name) = PathBuf::from(file_path).file_name() {
        let dest = dest_path.join(file_name);
        match std::fs::copy(file_path, dest) {
            Ok(_) => {
                println!("COPIED file to dest: {:?}", dest_path);
            }
            Err(e) => {
                eprintln!("error copying file to dest: {}", e);
            }
        }
    }
}
