use std::sync::mpsc::channel;
use std::thread::spawn;
use std::sync::mpsc::{Sender, Receiver};

use crate::exercise::Exercise;

struct Watcher {
    exercise: Exercise,
}

impl Watcher {
    pub fn new(exercise: Exercise) -> Watcher {
        Watcher {
            exercise: exercise,
        }
    }
    pub fn watch(&mut self) -> () {
        let (tx, rx): (Sender<Exercise>, Receiver<Exercise>) = channel();
        self.exercise.has_changed();
        let thread_changes:Sender<Exercise> = tx.clone();
        let handler_changes = spawn(move || {
            // self.exercise.has_changed()
        });
    }
}