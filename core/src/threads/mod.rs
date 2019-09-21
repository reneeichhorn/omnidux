use std::thread;
use std::sync::mpsc::{channel, Receiver, Sender};

use crate::task::Task;
use crate::config::Thread as ThreadConfig;
use crate::config::ThreadType;
use crate::scheduler::context::Context;

pub struct Thread {
  /// Task uuid.
  uuid: usize,
  /// Task sender.
  sender: Sender<Task>,
  /// Task fulfillment receiver.
  receiver: Option<Receiver<Task>>,
  /// Thread configuration.
  config: ThreadConfig,
}

#[derive(Clone)]
pub struct ThreadSender {
  pub sender: Sender<Task>,
}

impl ThreadSender {
  // Sends task to paired thread.
  pub fn send_task(&self, task: Task) {
    self.sender.send(task).unwrap();
  }
}

impl Thread {
  pub fn new(uuid: usize, config: ThreadConfig) -> Self {
    let (sender, receiver) = channel();

    Thread { 
      uuid: uuid,
      sender: sender,
      receiver: Some(receiver),
      config: config,
    }
  }

  pub fn create_sender(&self) -> ThreadSender {
    ThreadSender {
      sender: self.sender.clone(),
    }
  }

  /// Spawns the thread and executes the handler inside.
  pub fn spawn(&mut self, context: &Context) {
    let builder = thread::Builder::new()
      .name(self.config.name.clone());
    
    match self.config.thread_type {
      ThreadType::Main => {},
      ThreadType::WebWorker => {},
      ThreadType::Thread => {
        let thread_name = self.config.name.clone();
        let receiver = self.receiver.take().unwrap();
        let thread_context = context.clone();
        let uuid = self.uuid;

        builder.spawn(move || {
          loop {
            let result = receiver.recv();
            match result {
              Ok(received_task) => {
                thread_context.handle_schedule(uuid, &received_task);
              },
              Err(err) => {
                println!("[{n}] Error while receiving task {:?}", err, n = thread_name);
              }
            }
          }
        }).unwrap();
     }
    }
  }

  /// Blocks optionally the main thread to handle task.
  pub fn block(&mut self, context: &Context) {
    if let ThreadType::Main = self.config.thread_type {
    } else {
      return;
    }

    let receiver = self.receiver.take().unwrap();
    loop {
      let result =  receiver.recv();
      match result {
        Ok(received_task) => {
          context.handle_schedule(self.uuid, &received_task);
        },
        Err(err) => {
          println!("Error while receiving task {:?}", err);
        }
      }
    }
  }
}