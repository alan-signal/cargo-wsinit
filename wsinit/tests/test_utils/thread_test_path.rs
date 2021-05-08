use std::io::ErrorKind;
use std::ops::Deref;
use std::path::{Path, PathBuf};
use std::{fs, thread};

pub struct ThreadTestPath {
    path: PathBuf,
    clear_path_on_drop: bool,
}

impl Deref for ThreadTestPath {
    type Target = Path;

    fn deref(&self) -> &Self::Target {
        self.path.as_path()
    }
}

impl ThreadTestPath {
    /// Create a new directory containing the threads name, removing anything that was there before.
    pub fn new_removed() -> ThreadTestPath {
        let string = format!("thread_{:?}", thread::current().id());
        let path = Path::new(string.as_str());
        fs::create_dir(path).unwrap();
        let path = fs::canonicalize(&path).unwrap(); // CD may change while test is running
        let test_path = ThreadTestPath {
            path,
            clear_path_on_drop: true,
        };
        test_path.clear_path();
        test_path
    }

    /// Delete contents of path and directory itself.
    pub fn clear_path(&self) {
        match fs::remove_dir_all(&self.path) {
            Ok(_) => {}
            Err(e) if e.kind() == ErrorKind::NotFound => {}
            Err(e) => {
                panic!("Failed to clean temporary path {}", e)
            }
        }
    }

    /// Useful for debugging tests.
    #[allow(dead_code)]
    pub fn do_not_delete_on_exit(&mut self) {
        self.clear_path_on_drop = false;
    }
}

/// Thread test path will automatically clean itself up when it goes out of scope.
impl Drop for ThreadTestPath {
    fn drop(&mut self) {
        if self.clear_path_on_drop {
            self.clear_path();
        }
    }
}
