use notify::{RecursiveMode};
use notify_debouncer_mini::new_debouncer;
use notify_debouncer_mini::DebouncedEventKind;
use std::{path::Path, time::Duration};

use std::error::Error;

use std::collections::HashSet;

use log;

pub fn start_watcher(root_path_str: &str) -> std::result::Result<(), Box<dyn Error>> {
    // setup debouncer
    let (tx, rx) = std::sync::mpsc::channel();

    // No specific tickrate, max debounce time 2 seconds
    let mut debouncer = new_debouncer(Duration::from_secs(2), tx).unwrap();

    let watcher = debouncer
        .watcher();

    watcher.watch(Path::new(root_path_str), RecursiveMode::Recursive)?;

    // print all events, non returning
    for result in rx {
        match result {
            Ok(events) => {
                let mut paths = HashSet::new();
                events
                    .iter()
                    .for_each(|event| {
                        //log::info!("Event {event:?}")),
                        //println!("Event: {:?}", event);
                        log::info!("Event: {event:?}");
                        if event.kind == DebouncedEventKind::AnyContinuous {
                            log::info!("Continuous Event, Ignoring");
                        } else {
                            let path = event.path.to_str().unwrap_or_default();
                            log::info!("Rescan will be triggered: {event:?}");
                            paths.insert(path);
                        }
                        // TODO: eventually optimize to rescan only the changed path
                        // for path in paths.iter() {
                        //     processor::process_album(path);
                        // }
                    });
                // Exit the watcher to do another full scan
                if paths.len() > 0 {
                    drop(debouncer);
                    break;
                }
            }
            Err(error) => log::info!("Error {error:?}"),
        }
    }

    // let mut watcher = notify::recommended_watcher(|res| {
    //     match res {
    //         Ok(event) => println!("event: {:?}", event),
    //         Err(e) => println!("watch error: {:?}", e),
    //     }
    // })?;
    //watcher.watch(Path::new(root_path_str), RecursiveMode::Recursive)?;

    Ok(())
}