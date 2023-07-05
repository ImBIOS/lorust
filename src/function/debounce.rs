use std::{
    pin::Pin,
    sync::{mpsc, Arc, Mutex},
    time::Duration,
};

/// Debounce function takes a closure and a delay, and returns a `Debounce` struct.
///
/// # Arguments
///
/// * `closure` - A closure which accepts a parameter of type `T` and returns `()`.
/// * `delay` - The duration after which the closure should be invoked.
///
/// # Returns
///
/// * `Debounce<T>` - An instance of Debounce struct which can be used to call the closure
///   or terminate the execution.
///
/// The function also spawns a new thread which waits for a message to arrive on the receiver
/// end of a channel. The first message that arrives sets the `current_param`.
/// After the first message arrives, the thread waits with a timeout equal to `delay`.
/// If another message arrives during this period, `current_param` is overwritten.
/// If the timeout occurs without a new message, the closure is called with `current_param` as the argument.
pub fn debounce<F, T>(closure: F, delay: Duration) -> Debounce<T>
where
    F: Fn(T) + Send + Sync + 'static,
    T: Send + Sync + 'static,
{
    let (sender, receiver) = mpsc::channel();
    let sender = Arc::new(Mutex::new(sender));
    let debounce_config = Arc::new(Mutex::new(DebounceConfig {
        closure: Box::pin(closure),
        delay,
    }));

    let dup_debounce_config = debounce_config.clone();

    Debounce {
        sender: Some(sender),
        thread: Some(std::thread::spawn(move || {
            let debounce_config = dup_debounce_config;
            let mut current_param = None; // finally saved as an argument to the execution
            loop {
                if current_param.is_none() {
                    let message = receiver.recv();
                    match message {
                        Ok(param) => current_param = param,
                        Err(_) => {
                            break;
                        }
                    }
                } else {
                    let message = receiver.recv_timeout(debounce_config.lock().unwrap().delay);
                    match message {
                        Ok(param) => current_param = param,
                        Err(err) => match err {
                            mpsc::RecvTimeoutError::Timeout => {
                                if let Some(param) = current_param.take() {
                                    (*debounce_config.lock().unwrap().closure)(param);
                                }
                            }
                            mpsc::RecvTimeoutError::Disconnected => {
                                break;
                            }
                        },
                    }
                }
            }
        })),
        debounce_config,
    }
}

/// `DebounceConfig` struct stores a closure and a delay duration.
/// This struct is used within `Debounce` struct to keep track of the configuration.
///
/// # Fields
///
/// * `closure` - A closure which accepts a parameter of type `T` and returns `()`.
/// * `delay` - The duration after which the closure should be invoked.
struct DebounceConfig<T> {
    closure: Pin<Box<dyn Fn(T) + Send + Sync + 'static>>,
    delay: Duration,
}

impl<T> Drop for DebounceConfig<T> {
    /// The `Drop` trait implementation for `DebounceConfig`.
    /// Logs a message with the memory address of the `DebounceConfig` instance when it's dropped.
    fn drop(&mut self) {
        log::trace!("drop DebounceConfig {:?}", format!("{:p}", self));
    }
}

/// `Debounce` struct enables calling the provided closure after a specified delay.
/// The delay is reset if another call is made before the delay duration has passed.
/// It holds an `Option` for a sender which is used to send messages to the worker thread.
/// It also holds an `Option` for a JoinHandle for the worker thread.
/// The `debounce_config` stores the closure and delay information.
///
/// # Fields
///
/// * `sender` - An `Option` for a sender which is used to send messages to the worker thread.
/// * `thread` - An `Option` for a JoinHandle for the worker thread.
/// * `debounce_config` - Stores the closure and delay information.
#[allow(dead_code)]
pub struct Debounce<T> {
    sender: Option<Arc<Mutex<mpsc::Sender<Option<T>>>>>,
    thread: Option<std::thread::JoinHandle<()>>,
    debounce_config: Arc<Mutex<DebounceConfig<T>>>,
}

impl<T> Debounce<T> {
    /// Method `call` sends a message with a parameter to the worker thread.
    /// This causes the delay timer to reset in the worker thread.
    ///
    /// # Arguments
    ///
    /// * `param` - The parameter to send to the worker thread.
    /// This will be used as the argument for the closure when the delay has passed.
    pub fn call(&self, param: T) {
        self.sender
            .as_ref()
            .unwrap()
            .lock()
            .unwrap()
            .send(Some(param))
            .unwrap();
    }

    /// Method `terminate` sends a `None` message to the worker thread,
    /// causing it to exit the loop and end the execution.
    pub fn terminate(&self) {
        self.sender
            .as_ref()
            .unwrap()
            .lock()
            .unwrap()
            .send(None)
            .unwrap();
    }
}

impl<T> Drop for Debounce<T> {
    /// The `Drop` trait implementation for `Debounce`.
    /// On drop, it terminates the worker thread and logs a message with the memory address of the `Debounce` instance.
    fn drop(&mut self) {
        self.terminate();
        log::trace!("drop Debounce {:?}", format!("{:p}", self));
    }
}
