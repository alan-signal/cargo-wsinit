mod test_utils;

#[cfg(test)]
mod tests {
    use std::fs::{self, File};
    use std::path::{Path, PathBuf};

    use cargo_wsinit::*;

    use crate::test_utils::*;

    #[test]
    fn update_paths() {
        let test_root = ThreadTestPath::new_removed();
        let root_path = test_root.to_str().unwrap();

        make_lib(&test_root, "lib1");
        make_lib(&test_root, "lib2");
        make_non_lib(&test_root, "not_a_lib");
        make_lib(&test_root, "lib3/sub");

        let options = Options::new(root_path, FileExistsBehaviour::Halt);
        let toml_file = Workspace::new(options)
            .update_toml()
            .expect("Expect new file to be made without issue");

        assert!(toml_file.exists());

        let file_contents = read_file(&toml_file);

        assert_eq!(
            "[workspace]\n\nmembers = [\n    \"lib1\",\n    \"lib2\",\n    \"lib3/sub\",\n]\n",
            file_contents
        );
    }

    #[test]
    fn skip_target_paths() {
        let test_root = ThreadTestPath::new_removed();
        let root_path = test_root.to_str().unwrap();

        make_lib(&test_root, "lib1");
        make_lib(&test_root, "target/lib2");
        make_lib(&test_root, "lib3/sub");

        let options = Options::new(root_path, FileExistsBehaviour::Halt);
        let toml_file = Workspace::new(options)
            .update_toml()
            .expect("Expect new file to be made without issue");

        assert!(toml_file.exists());

        let file_contents = read_file(&toml_file);

        assert!(!file_contents.contains("target"));
    }

    #[test]
    fn update_paths_in_cd() {
        let test_root = ThreadTestPath::new_removed();
        let mut root_path = test_root.to_str().unwrap();

        make_lib(&test_root, "lib1");
        make_lib(&test_root, "lib2");
        make_lib(&test_root, "lib3/sub");

        let _dir = TestDirChange::cd(PathBuf::from(root_path));
        root_path = ".";

        let options = Options::new(root_path, FileExistsBehaviour::Halt);
        let toml_file = Workspace::new(options)
            .update_toml()
            .expect("Expect new file to be made without issue");

        assert!(toml_file.exists());

        let file_contents = read_file(&toml_file);

        assert_eq!(
            "[workspace]\n\nmembers = [\n    \"lib1\",\n    \"lib2\",\n    \"lib3/sub\",\n]\n",
            file_contents
        );
    }

    #[test]
    fn do_not_include_sub_directories_where_parent_has_a_toml() {
        let test_root = ThreadTestPath::new_removed();
        let root_path = test_root.to_str().unwrap();

        make_lib(&test_root, "lib1");
        make_lib(&test_root, "lib1/sub");
        make_lib(&test_root, "lib2");

        let options = Options::new(root_path, FileExistsBehaviour::Halt);
        let toml_file = Workspace::new(options)
            .update_toml()
            .expect("Expect new file to be made without issue");

        assert!(toml_file.exists());

        let file_contents = read_file(&toml_file);

        assert_eq!(
            "[workspace]\n\nmembers = [\n    \"lib1\",\n    \"lib2\",\n]\n",
            file_contents
        );
    }

    fn make_lib(path: &Path, lib_name: &str) {
        let path = create_path(path, lib_name);

        let toml = path.join(PathBuf::from("Cargo.toml"));

        File::create(toml).expect("");
    }

    fn make_non_lib(path: &Path, lib_name: &str) {
        let path = create_path(path, lib_name);

        let toml = path.join(PathBuf::from("SomeFile.txt"));

        File::create(toml).expect("");
    }

    fn create_path(path: &Path, lib_name: &str) -> PathBuf {
        let path = path.join(PathBuf::from(lib_name));
        fs::create_dir_all(&path).unwrap();
        path
    }
}
