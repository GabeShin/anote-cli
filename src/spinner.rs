use std::io::Write;
use std::sync::{Arc, Mutex};
use std::thread::{self, JoinHandle};
use std::time::Duration;

pub struct Spinner {
    handle: Option<JoinHandle<()>>,
    control: Arc<Mutex<bool>>,
}

impl Spinner {
    pub fn start() -> Self {
        let control = Arc::new(Mutex::new(true));
        let handle = {
            let control = Arc::clone(&control);
            Some(thread::spawn(move || {
                let spinner_chars = vec!['|', '/', '-', '\\'];
                let mut i = 0;
                while *control.lock().unwrap() {
                    print!("\r{}", spinner_chars[i]);
                    i = (i + 1) % spinner_chars.len();
                    std::io::stdout().flush().unwrap();
                    thread::sleep(Duration::from_millis(100));
                }
                print!("\r \r"); // Erase the spinner character
            }))
        };

        Spinner { handle, control }
    }

    pub fn stop(&mut self) {
        *self.control.lock().unwrap() = false;
        if let Some(handle) = self.handle.take() {
            handle.join().unwrap();
        }
    }
}

impl Drop for Spinner {
    fn drop(&mut self) {
        self.stop();
    }
}
