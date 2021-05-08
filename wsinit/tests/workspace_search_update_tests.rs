mod test_utils;

#[cfg(test)]
mod tests {
    use std::fs::File;
    use std::path::{Path, PathBuf};
    use std::{env, fs};

    use cargo_wsinit::*;

    use crate::test_utils::*;

    #[test]
    fn update_paths() {
        let test_root = ThreadTestPath::new_removed();
        let root_path = test_root.to_str().unwrap();

        make_lib(&test_root, "lib1");
        make_lib(&test_root, "lib2");
        make_lib(&test_root, "lib3/sub");

        let options = Options::new(root_path, FileExistsBehaviour::Halt);
        let toml_file = Workspace::new(options)
            .update_toml()
            .expect("Expect new file to be made without issue");

        assert!(toml_file.exists());

        let file_contents = read_file(&toml_file);

        assert_eq!(
            format!(
                "\n[workspace]\nmembers = [\n    \"{}/lib1\",\n    \"{}/lib2\",\n    \"{}/lib3/sub\",\n]\n",
                root_path,
                root_path,
                root_path
            ),
            file_contents
        );
    }

    #[test]
    fn update_paths_in_cd() {
        let test_root = ThreadTestPath::new_removed();
        let mut root_path = test_root.to_str().unwrap();

        make_lib(&test_root, "lib1");
        make_lib(&test_root, "lib2");
        make_lib(&test_root, "lib3/sub");

        env::set_current_dir(root_path).unwrap();
        root_path = ".";

        let options = Options::new(root_path, FileExistsBehaviour::Halt);
        let toml_file = Workspace::new(options)
            .update_toml()
            .expect("Expect new file to be made without issue");

        assert!(toml_file.exists());

        let file_contents = read_file(&toml_file);

        assert_eq!(
            "\n[workspace]\nmembers = [\n    \"lib1\",\n    \"lib2\",\n    \"lib3/sub\",\n]\n",
            file_contents
        );
    }

    fn make_lib(path: &Path, lib_name: &str) {
        let path = path.join(PathBuf::from(lib_name));
        fs::create_dir_all(&path).unwrap();

        let toml = path.join(PathBuf::from("Cargo.toml"));

        File::create(toml).expect("");
    }
}
