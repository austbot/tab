use std::sync::mpsc;
use std::thread;

pub trait FnBox {
    fn call_box(self: Box<Self>);
}

impl<F: FnOnce()> FnBox for F {
    fn call_box(self: Box<F>) {
        (*self)()
    }
}

pub type Swimmer = Box<FnBox + Send + 'static>;

pub struct Lane {
    pub id: String,
    sender: Option<mpsc::Sender<Swimmer>>,
    thread: Option<thread::JoinHandle<()>>
}

impl Lane {
    pub fn new(id: String) -> Lane {
        Lane {
            id,
            sender: None,
            thread: None
        }
    }

    pub fn is_setup(&self) -> bool {
        return self.thread.is_some() && self.sender.is_some();
    }

    pub fn setup(&mut self) {
        let (sender, receiver) = mpsc::channel();
        let id = self.id.clone();
        self.sender = Some(sender);
        self.thread = Some(thread::spawn(move || {
            loop {
                println!("Lane {}, executed a job", id);
                let job = receiver.recv().unwrap();
                job.call_box();
            }
        }));
    }

//    pub fn get_load(&self) -> usize {
//        self.load.into_inner().unwrap();
//    }

    pub fn act(&self, job: Swimmer) {
        assert!(self.is_setup(), true);
        self.sender.as_ref().unwrap().send(job);
    }
}


pub struct Pool {
    lanes: Vec<Lane>
}

impl Pool {
    pub fn new(size: usize) -> Pool {
        let mut lanes = Vec::with_capacity(size);
        for id in 0..size {
            let mut lane = Lane::new(id.to_string());
            lane.setup();
            lanes.push(lane);
        }

        Pool {
            lanes
        }
    }

    pub fn send<F>(&self, id: usize, f: F)
        where
            F: FnOnce() + Send + 'static
    {
        let job: Swimmer = Box::new(f);
        let lane = &self.lanes[id];
        lane.act(job)
    }

//    pub fn send_balanced<F>(&self, f: F)
//        where
//            F: FnOnce() + Send + 'static
//    {
//        let lane = &*self.lanes.iter().min_by(|l1, l2| l1.load.cmp(&l2.load)).unwrap();
//        let job: Swimmer = Box::new(f);
//        lane.act(job)
//    }
}

