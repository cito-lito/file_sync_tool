use notify::{
    event::{self, CreateKind, ModifyKind},
    EventKind,
};
use notify_debouncer_full::DebouncedEvent;
use std::path::{Path, PathBuf};
mod path_watcher;

fn main() -> Result<(), Box<dyn std::error::Error>> {
    let path_to_watch = PathBuf::from("./tests/from");
    let dest_path = PathBuf::from("./tests/to");

    let (_debouncer, e_rx) =
        path_watcher::watch_path_with_debouncer(path_to_watch).expect("watch_path_with_debouncer");
    for debounced_result in e_rx {
        match debounced_result {
            Ok(events) => {
                for event in events {
                    println!("Watching  {:?}...", event);
                    handle_event(event, &dest_path);
                }
            }
            Err(e) => eprintln!("watch error: {:?}", e),
        }
    }

    Ok(())
}

fn handle_event(event: DebouncedEvent, dest_path: &PathBuf) {
    match event.kind {
        EventKind::Create(event_kind) => handle_create_event(event_kind, &event.paths, dest_path),
        EventKind::Modify(event_kind) => handle_modify_event(event_kind, &event.paths, dest_path),
        EventKind::Remove(event_kind) => handle_remove_event(event_kind, &event.paths, dest_path),
        _ => {
            println!("UNHANDLED{:?}", event);
        }
    }
}

fn handle_create_event(event_kind: event::CreateKind, paths: &Vec<PathBuf>, dest_path: &PathBuf) {
    for path in paths {
        match event_kind {
            CreateKind::File => {
                create_file(path, dest_path);
            }
            _ => {
                println!("UNHANDLED create event {:?}", event_kind);
            }
        }
    }
}

fn handle_modify_event(event_kind: event::ModifyKind, paths: &Vec<PathBuf>, dest_path: &PathBuf) {
    for path in paths {
        match event_kind {
            ModifyKind::Data(_) => copy_file(path, dest_path),
            ModifyKind::Name(_) => {}
            _ => {
                println!("UNHANDLED modify event {:?}", event_kind);
            }
        }
    }
}

fn handle_remove_event(event_kind: event::RemoveKind, paths: &Vec<PathBuf>, dest_path: &PathBuf) {
    for path in paths {
        match event_kind {
            event::RemoveKind::File => {
                println!("REMOVING file {:?}", dest_path);
                //TODO: remove debounce event for remove file looks like this: { kind: Modify(Name(Any))..
                remove_file(path, dest_path)
            }
            event::RemoveKind::Folder => {}
            _ => {
                println!("UNHANDLED remove event {:?}", event_kind);
            }
        }
    }
}

// for now leverage fs copy
fn create_file(file_path: &PathBuf, dest_path: &PathBuf) {
    // let dest_path = PathBuf::from("./tests/to");

    let file_name = file_path.file_name().expect("file name should exist");
    let dest_file = dest_path.join(file_name);

    if dest_file.exists() {
        println!("file already exists in dest: {:?}", dest_file);
    } else {
        match std::fs::copy(file_path, dest_file) {
            Ok(_) => {
                println!("COPIED file to dest: {:?}", dest_path);
            }
            Err(e) => {
                eprintln!("error copying file to dest: {}", e);
            }
        }
    }
}

fn remove_file(file_path: &Path, dest_path: &PathBuf) {
    let file_name = file_path.file_name().expect("file name should exist");
    let dest_file = dest_path.join(file_name);

    if dest_file.exists() {
        match std::fs::remove_file(dest_file) {
            Ok(_) => {
                println!("REMOVED file from dest: {:?}", dest_path);
            }
            Err(e) => {
                eprintln!("error removing file from dest: {}", e);
            }
        }
    } else {
        println!("file does not exist in dest: {:?}", dest_file);
    }
}

fn copy_file(file_path: &PathBuf, dest_path: &PathBuf) {
    let file_name = file_path.file_name().expect("file name should exist");
    let dest_file = dest_path.join(file_name);

    match std::fs::copy(file_path, dest_file) {
        Ok(_) => {
            println!("COPIED file to dest: {:?}", dest_path);
        }
        Err(e) => {
            eprintln!("error copying file to dest: {}", e);
        }
    }
}
