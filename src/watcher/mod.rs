use std::sync::{Arc, Mutex};
use std::sync::mpsc::{Receiver, Sender};
use std::sync::mpsc::channel;
use std::thread::spawn;

use crate::exercise::Exercise;

pub struct Watcher {
    exercise: Exercise,
}

enum WatcherMessage {
    HasChanged
}

impl Watcher {
    pub fn new(exercise: Exercise) -> Watcher {
        Watcher {
            exercise: exercise,
        }
    }
    pub fn watch(self) -> () {
        let exercise_arc_mtx = Arc::new(Mutex::from(self.exercise));
        let exercise_arc_clone = Arc::clone(&exercise_arc_mtx);
        let (tx, rx): (Sender<WatcherMessage>, Receiver<WatcherMessage>) = channel();

        let thread_changes: Sender<WatcherMessage> = tx.clone();
        let handler_changes = spawn(move || {
            println!("Helloooo");
            let mut last_mod = 0u64;
            loop {
                let current_last_mod = exercise_arc_clone.lock().unwrap().get_last_mod();
                if current_last_mod.ne(&last_mod) {
                    println!("File has changed!");
                    last_mod = current_last_mod;
                    thread_changes.send(WatcherMessage::HasChanged).unwrap();
                }
            }
        });
        loop {
            let message = rx.recv().unwrap();
            println!("File changed, proceeding to compilation");
            let compilation_result = exercise_arc_mtx.lock().unwrap().compile();
            match compilation_result.unwrap().success()
            {
                true => {
                    println!("Code is compiling");
                    // break;
                }
                false => println!("Error while compiling")
            }
        }
    }
}