use notify::{Event, EventKind, RecommendedWatcher, RecursiveMode, Watcher};
use std::fs::create_dir;
use std::ops::Sub;
use std::path::PathBuf;
use std::sync::{Arc, Condvar, Mutex};
use std::thread;
use std::time::{Duration, Instant};

pub struct FileMonitor {
    path: String,
    dir_path: String,
    config_pair: Arc<(Mutex<String>, Condvar)>,
}

impl FileMonitor {
    pub fn new(path: String, config_pair: Arc<(Mutex<String>, Condvar)>) -> Self {
        let mut dir_path = PathBuf::from(&path);
        dir_path.pop();
        FileMonitor {
            path,
            dir_path: dir_path.to_str().unwrap().to_string(),
            config_pair,
        }
    }

    pub fn monitor(&self) {
        create_dir(&self.dir_path).unwrap_or_default();

        let (tx, rx) = std::sync::mpsc::channel();
        let mut watcher = RecommendedWatcher::new(tx, notify::Config::default()).unwrap();
        watcher
            .watch(self.dir_path.as_ref(), RecursiveMode::Recursive)
            .unwrap();

        let mut last_update_time = Instant::now().sub(Duration::from_secs(60));

        loop {
            // only update config if the event is related to a file change
            if let Ok(Ok(Event {
                kind: EventKind::Modify(_),
                ..
            })) = rx.recv()
            {
                // debounce duplicated events
                if last_update_time.elapsed().as_millis() > 100 {
                    // ensure file changes are propagated
                    thread::sleep(Duration::from_millis(100));

                    *self.config_pair.0.lock().unwrap() =
                        std::fs::read_to_string(&self.path).unwrap();
                    println!("File update received");
                    self.config_pair.1.notify_all();

                    last_update_time = Instant::now();
                }
            }
        }
    }
}
