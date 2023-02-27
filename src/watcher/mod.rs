use std::sync::{Arc, Mutex, MutexGuard};
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
        let _handler_changes = spawn(move || {
            println!("Watching exercise");
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
            let _message = rx.recv().unwrap();
            println!("File changed, proceeding to evaluation");
            let exercise_guard = exercise_arc_mtx.lock().unwrap();
            let result = match exercise_guard.mode {
                Mode::Compile => {
                    Self::compile(exercise_guard)
                }
                Mode::Test => {
                    Self::run_tests(exercise_guard)
                }
                Mode::Clippy => { false }
            };
            if result {
                println!("Succeeded");
                break;
            }
        }
    }

    fn compile(exercise_guard: MutexGuard<Exercise>) -> bool {
        println!("Compiling");
        let compilation_result = exercise_guard.compile();
        compilation_result.unwrap().success()
    }
    fn run_tests(exercise_guard: MutexGuard<Exercise>) -> bool {
        println!("Testing");
        let test_result = exercise_guard.run_tests();
        test_result.unwrap().success()
    }
}