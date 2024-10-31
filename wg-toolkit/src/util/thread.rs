//! Thread polling utilities.

use std::sync::atomic::{AtomicBool, AtomicUsize, Ordering};
use std::sync::Arc;
use std::thread;

use crossbeam_channel::{Receiver, Sender};
use tracing::trace;


/// This structure is made to block on multiple thread at the same time and repeatedly
/// in order to aggregate the value they are returning.
#[derive(Debug)]
pub struct ThreadPoll<T> {
    tx: Sender<T>,
    rx: Receiver<T>,
    count: AtomicUsize,
}

impl<T: Send + 'static> ThreadPoll<T> {

    pub fn new() -> Self {
        let (tx, rx) = crossbeam_channel::bounded(2);
        Self {
            tx, rx, count: AtomicUsize::new(0),
        }
    }

    /// Spawn a new value producer that will be continuously polled and its result will
    /// be added to the internal queue that can be retrieved with [`Self::poll`], this
    /// producer's thread terminates when this aggregator is dropped. In order for this
    /// to properly work you should be using some kind of timeout on the producer.
    pub fn spawn<F>(&self, mut producer: F)
    where 
        F: FnMut() -> Option<T>,
        F: Send + 'static,
    {

        let tx = self.tx.clone();
        let num = self.count.fetch_add(1, Ordering::Relaxed);
        
        thread::Builder::new()
            .name(format!("poll-worker-{num}"))
            .spawn(move || {
                trace!("Spawned poll worker #{num}");
                while let Some(value) = producer() {
                    if tx.send(value).is_err() {
                        break;
                    }
                }
                trace!("Terminated poll worker #{num}")
            })
            .unwrap();
        
    }

    /// Same as [`Self::spawn`] but also returning a handle that, when dropped, will end
    /// the associated worker thread.
    pub fn spawn_with_handle<F>(&self, mut producer: F) -> ThreadPollHandle
    where 
        F: FnMut() -> Option<T>,
        F: Send + 'static,
    {
        let alive = Arc::new(AtomicBool::new(true));
        let thread_alive = Arc::clone(&alive);
        self.spawn(move || if thread_alive.load(Ordering::Relaxed) {
            producer()
        } else {
            None
        });
        ThreadPollHandle(alive)
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

/// Represent a handle to a thread poll worker, when all handles to 
#[derive(Debug, Clone)]
pub struct ThreadPollHandle(Arc<AtomicBool>);

impl Drop for ThreadPollHandle {
    fn drop(&mut self) {
        self.0.store(false, Ordering::Relaxed);
    }
}
