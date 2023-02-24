use std::sync::{Arc, Mutex};
use std::sync::mpsc::{Receiver, Sender};
use std::sync::mpsc::channel;
use std::thread::{sleep, spawn};
use std::time::Duration;

use crate::exercise::{Exercise, Mode};

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
            let exercise_guard = exercise_arc_mtx.lock().unwrap();
            let compilation_result = exercise_guard.compile();
            match compilation_result.unwrap().success()
            {
                true => {
                    println!("Code is compiling");
                    if let Mode::Test = exercise_guard.mode {
                        println!("Launching tests");
                        exercise_guard.run_tests();
                    }
                    // sleep(Duration::from_secs(300u64))
                }
                false => {
                    println!("Error while compiling");
                    // sleep(Duration::from_secs(300u64))
                }
            }
        }
    }
}