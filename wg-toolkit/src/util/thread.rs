//! Thread polling utilities.

use std::sync::atomic::{AtomicBool, Ordering};
use std::sync::Arc;
use std::thread;

use crossbeam_channel::{Receiver, Sender};


/// This structure is made to block on multiple thread at the same time and repeatedly
/// in order to aggregate the value they are returning.
#[derive(Debug)]
pub struct ThreadPoll<T> {
    tx: Sender<T>,
    rx: Receiver<T>,
    alive: Arc<AtomicBool>,
}

impl<T: Send + 'static> ThreadPoll<T> {

    pub fn new() -> Self {
        let (tx, rx) = crossbeam_channel::bounded(2);
        Self {
            tx, rx,
            alive: Arc::new(AtomicBool::new(true)),
        }
    }

    /// Spawn a new value producer that will be continuously polled and its result will
    /// be added to the internal queue that can be retrieved with [`Self::poll`], this
    /// producer's thread terminates when this aggregator is dropped. In order for this
    /// to properly work you should be using so kind of timeout on the producer.
    pub fn spawn<F>(&self, mut producer: F) -> ThreadPollHandle
    where 
        F: FnMut() -> T,
        F: Send + 'static,
    {

        let tx = self.tx.clone();
        let alive = Arc::clone(&self.alive);
        let handle_alive = Arc::new(AtomicBool::new(true));
        let handle = ThreadPollHandle {
            alive: Arc::clone(&handle_alive)
        };

        thread::Builder::new()
            .name(format!("Thread Poll Worker"))
            .spawn(move || {
                while alive.load(Ordering::Relaxed) && handle_alive.load(Ordering::Relaxed) {
                    // Deliberately ignoring potential error if the channel has been closed
                    // since we .
                    let _ = tx.send(producer());
                }
            })
            .unwrap();
        
        handle

    }

    /// Block until a new value is available.
    pub fn poll(&self) -> T {
        // Unwrap because we own both ends so it should not disconnect.
        self.rx.recv().unwrap()
    }

    /// Non-blocking poll.
    pub fn try_poll(&self) -> Option<T> {
        // Don't care of the "disconnected" error because it should not happen.
        self.rx.try_recv().ok()
    }

}

impl<T> Drop for ThreadPoll<T> {
    fn drop(&mut self) {
        self.alive.store(false, Ordering::Relaxed);
    }
}

#[derive(Debug, Clone)]
pub struct ThreadPollHandle {
    alive: Arc<AtomicBool>,
}

impl ThreadPollHandle {
    
    pub fn terminate(&self) {
        self.alive.store(false, Ordering::Relaxed);
    }

}
