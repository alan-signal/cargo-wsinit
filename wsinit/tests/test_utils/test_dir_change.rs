use std::env;
use std::path::PathBuf;

pub struct TestDirChange {
    original_path: PathBuf,
}

impl TestDirChange {
    #[allow(dead_code)]
    pub fn cd(to: PathBuf) -> TestDirChange {
        let dir = env::current_dir().unwrap();
        env::set_current_dir(to).unwrap();

        TestDirChange { original_path: dir }
    }
}

impl Drop for TestDirChange {
    fn drop(&mut self) {
        env::set_current_dir(&self.original_path).unwrap();
    }
}
