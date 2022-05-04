use {
    futures::{
        future::{BoxFuture, FutureExt},
        task::{waker_ref, ArcWake},
    },
    std::{
        future::Future,
        sync::mpsc::{sync_channel, Receiver, SyncSender, TryRecvError},
        sync::{Arc, Mutex},
        task::{Context, Poll},
    },
};

pub struct Executor<TA> {
    ready_queue: Receiver<Arc<TA>>,
}

#[derive(Clone)]
pub struct Spawner<TA> {
    task_sender: SyncSender<Arc<TA>>,
}

pub struct Task<A> {
    future: Mutex<Option<BoxFuture<'static, A>>>,
    task_sender: SyncSender<Arc<Task<A>>>,
}

impl<A> ArcWake for Task<A> {
    fn wake_by_ref(arc_self: &Arc<Self>) {
        let cloned = arc_self.clone();
        arc_self.task_sender.send(cloned).expect("too many tasks queued B");
    }
}

pub struct TaskCB<A> {
    future: Mutex<Option<BoxFuture<'static, A>>>,
    task_sender: SyncSender<Arc<TaskCB<A>>>,
    cb: extern "C" fn(u64),
    data: u64,
}

impl<A> ArcWake for TaskCB<A> {
    fn wake_by_ref(arc_self: &Arc<Self>) {
        let cloned = arc_self.clone();
        let cb: extern "C" fn(u64) = arc_self.cb;
        cb(arc_self.data);
        arc_self.task_sender.send(cloned).expect("too many tasks queued B");
    }
}

pub fn new_executor_and_spawner<A>() -> (Executor<A>, Spawner<A>) {
    const MAX_QUEUED_TASKS: usize = 2;
    let (task_sender, ready_queue) = sync_channel(MAX_QUEUED_TASKS);
    (Executor { ready_queue }, Spawner { task_sender })
}

impl<A> Spawner<TaskCB<A>> {
    /// a spawner that accepts a C callback
    pub fn spawn_cb(&self, future: impl Future<Output = A> + 'static + Send, cb: extern "C" fn(u64), data: u64) {
        let future = future.boxed();
        let task = Arc::new(TaskCB {
            future: Mutex::new(Some(future)),
            task_sender: self.task_sender.clone(),
            cb,
            data,
        });
        self.task_sender.send(task).expect("too many tasks queued A");
    }
}

impl<A> Spawner<Task<A>> {
    pub fn spawn(&self, future: impl Future<Output = A> + 'static + Send) {
        let future = future.boxed();
        let task = Arc::new(Task {
            future: Mutex::new(Some(future)),
            task_sender: self.task_sender.clone(),
        });
        self.task_sender.send(task).expect("too many tasks queued A");
    }
}

#[derive(Clone, Debug, PartialEq, Eq)]
pub enum Progress<A> {
    Done(A),
    More,
    Error(String),
}

// TODO: deduplicate this code
impl<A> Executor<TaskCB<A>> {
    pub fn step(&self) -> Progress<A> {
        match self.ready_queue.try_recv() {
            Err(TryRecvError::Empty) => Progress::More,
            Err(TryRecvError::Disconnected) => Progress::Error("Disconnected worker".to_string()),
            Ok(task) => {
                let mut future_slot = task.future.lock().unwrap();
                if let Some(mut future) = future_slot.take() {
                    let waker = waker_ref(&task);
                    let context = &mut Context::from_waker(&*waker);
                    match future.as_mut().poll(context) {
                        Poll::Ready(r) => return Progress::Done(r),
                        Poll::Pending => *future_slot = Some(future),
                    }
                }
                Progress::More
            }
        }
    }
}

impl<A> Executor<Task<A>> {
    pub fn step(&self) -> Progress<A> {
        match self.ready_queue.try_recv() {
            Err(TryRecvError::Empty) => Progress::More,
            Err(TryRecvError::Disconnected) => Progress::Error("Disconnected worker".to_string()),
            Ok(task) => {
                let mut future_slot = task.future.lock().unwrap();
                if let Some(mut future) = future_slot.take() {
                    let waker = waker_ref(&task);
                    let context = &mut Context::from_waker(&*waker);
                    match future.as_mut().poll(context) {
                        Poll::Ready(r) => return Progress::Done(r),
                        Poll::Pending => *future_slot = Some(future),
                    }
                }
                Progress::More
            }
        }
    }
}

pub fn block_on<A>(future: impl Future<Output = A> + 'static + Send) -> A {
    let (executor, spawner) = new_executor_and_spawner();
    spawner.spawn(future);
    drop(spawner);
    loop {
        match executor.step() {
            Progress::More => continue,
            Progress::Error(rr) => panic!("{}", rr),
            Progress::Done(x) => return x,
        }
    }
}
