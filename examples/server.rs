use std::fs;
use std::thread;
use std::io::prelude::*;
use std::net::{TcpListener, TcpStream};
use std::num::NonZeroU32;
use std::sync::{mpsc, Arc, Mutex};
use crate::Message::{NewJob, Terminate};


type Job = Box<dyn FnOnce() + Send + 'static>;

pub enum Message {
    NewJob(Job),
    Terminate
}


pub struct ThreadPool {
    workers: Vec<Worker>,
    sender: mpsc::Sender<Message>,
}

impl ThreadPool {
    pub fn new(size: NonZeroU32) -> ThreadPool {
        let size = size.get() as usize;
        let (sender, receiver) = mpsc::channel();

        let mut workers = Vec::with_capacity(size);
        let receiver = Arc::new(Mutex::new(receiver));

        for id in 0..size {
            workers.push(Worker::new(id, receiver.clone()));
        }

        ThreadPool { workers, sender }
    }

    pub fn execute<F>(&self, f: F)
    where
        F: FnOnce() + Send + 'static,
    {
        let job = Box::new(f);
        self.sender.send(NewJob(job)).unwrap();
    }
}

impl Drop for ThreadPool {
    fn drop(&mut self) {
        for _ in &self.workers {
            self.sender.send(Terminate);
        }

        for worker in &mut self.workers {
            println!("Shutting down worker {}", worker.id);
            if let Some(thread) = worker.thread.take() {
                thread.join().unwrap();
            }
        }
    }
}

struct Worker {
    id: usize,
    thread: Option<thread::JoinHandle<()>>,
}

impl Worker {
    fn new(id: usize, receiver: Arc<Mutex<mpsc::Receiver<Message>>>) -> Worker {
        let thread = thread::spawn(move || loop {
            let message = receiver.lock().unwrap().recv().unwrap();

            match message {
                NewJob(job) => {
                    println!("Worker {} got a job; executing.", id);
                    job();
                }
                Message::Terminate => {
                    return;
                }
            }
        });

        Worker { id, thread: Some(thread) }
    }
}


fn main() -> std::io::Result<()> {
    let listener    = TcpListener::bind("[::1]:8080")?;
    let thread_pool = ThreadPool::new(NonZeroU32::new(4).unwrap());

    for stream in listener.incoming() {
        let stream = stream?;
        thread_pool.execute(|| {
            handle_connection(stream);
        });
    }

    Ok(())
}

// TcpStream needs needs to be mut because its internal state might change, as it keeps track of
// what data it returns to us.
fn handle_connection(mut stream: TcpStream) {
    let mut buffer = [0; 1024];

    stream.read(&mut buffer).unwrap();

    // The “lossy” part of the name indicates the behavior of this function when it sees an
    // invalid UTF-8 sequence: it will replace the invalid sequence with �.
    println!("Request: {}", String::from_utf8_lossy(&buffer[..]));

    let get = b"GET / HTTP/1.1\r\n";
    let (status, filename) = if buffer.starts_with(get) {
        ("200 OK", "static/html/index.html")
    } else {
        ("404 NOT FOUND", "static/html/errors/404.html")
    };

    let contents = fs::read_to_string(filename).unwrap();
    let response = format!("HTTP/1.1 {}\r\n\r\n{}", status, contents);

    stream.write(response.as_bytes()).unwrap();
    stream.flush().unwrap();
}
