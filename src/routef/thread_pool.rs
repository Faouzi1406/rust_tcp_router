use std::sync::mpsc;
use std::thread;
use std::sync::Arc;
use std::sync::Mutex;


pub struct ThreadPool {
    threads: Vec<Workers>,
    sender: mpsc::Sender<Job>,
}

impl ThreadPool {
    pub fn new(size:usize) -> ThreadPool  {
        assert!(size >0);
        let (sender, receiver) = mpsc::channel();
        let mut threads_workers = Vec::with_capacity(size);


        let receiver = Arc::new(Mutex::new(receiver));

        for id in 0..size {
            threads_workers.push(Workers::new(id, Arc::clone(&receiver)))
        }

        ThreadPool {
            threads:threads_workers,
            sender
        }
    }
    pub fn execute<F>(&self, f: F)
        where
            F: FnOnce() + Send + 'static
    {
        let job = Box::new(f);

        self.sender.send(job).unwrap();
    }
}

trait FnBox {
    fn call_box(self: Box<Self>);
}

impl<F: FnOnce()> FnBox for F {
    fn call_box(self: Box<F>) {
        (*self)()
    }
}

struct Workers {
    id:usize,
    thread:thread::JoinHandle<()>
}


type Job = Box<dyn FnBox + Send + 'static>;

impl Workers {
     fn new(id: usize, receiver: Arc<Mutex<mpsc::Receiver<Job>>>) -> Workers {
        let thread = thread::spawn(move || {
            loop {
                let job = receiver.lock().unwrap().recv().unwrap();
                job.call_box();
            }
        });

       Workers {
            id,
            thread,
        }
    } 
}
