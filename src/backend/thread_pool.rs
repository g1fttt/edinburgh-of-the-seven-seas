use std::sync::mpsc::{self, Receiver, Sender};
use std::sync::{Arc, Mutex};

use std::thread::{self, JoinHandle};

type Job = Box<dyn FnOnce() + Send + 'static>;

#[derive(Debug)]
pub struct Pool {
  workers: Vec<Worker>,
  sender: Option<Sender<Job>>,
}

impl Pool {
  pub fn with_capacity(n: usize) -> Self {
    let (sender, receiver) = mpsc::channel();
    let receiver = Arc::new(Mutex::new(receiver));

    let workers =
      (0..n).map(|i| Worker::new(i, Arc::clone(&receiver))).collect();

    Self { workers, sender: Some(sender) }
  }

  pub fn execute<F>(&self, f: F)
  where
    F: FnOnce() + Send + 'static,
  {
    let sender = self.sender.as_ref().unwrap();

    let job = Box::new(f);

    if let Err(err) = sender.send(job) {
      eprintln!("Failed to send message to the threads: {err}");
    }
  }
}

impl Drop for Pool {
  fn drop(&mut self) {
    drop(self.sender.take());

    for worker in &mut self.workers {
      if let Some(thread) = worker.thread.take() {
        thread.join().expect("Failed to join thread");
      }
    }
  }
}

type ArcJobReceiver = Arc<Mutex<Receiver<Job>>>;

#[derive(Debug)]
struct Worker {
  #[expect(
    dead_code,
    reason = "Awaiting logger implementation for the further debugging"
  )]
  id: usize,
  thread: Option<JoinHandle<()>>,
}

impl Worker {
  fn new(id: usize, receiver: ArcJobReceiver) -> Self {
    let thread = thread::spawn(move || {
      loop {
        let guard = receiver.lock().unwrap();

        match guard.recv() {
          Ok(job) => job(),
          Err(_) => break,
        }
      }
    });

    Self { id, thread: Some(thread) }
  }
}
