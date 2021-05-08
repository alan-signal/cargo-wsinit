mod test_utils;

#[cfg(test)]
mod tests {
    use crate::test_utils::*;
    use cargo_wsinit::*;

    #[test]
    fn create_a_new_toml_file() {
        let test_root = ThreadTestPath::new_removed();

        let options = Options::new(test_root.to_str().unwrap(), FileExistsBehaviour::Halt);
        let toml_file = Workspace::new(options)
            .update_toml()
            .expect("Expect new file to be made without issue");

        assert!(toml_file.exists());
    }

    #[test]
    fn create_a_new_toml_file_in_overwrite_mode() {
        let test_root = ThreadTestPath::new_removed();

        let options = Options::new(test_root.to_str().unwrap(), FileExistsBehaviour::Overwrite);
        let toml_file = Workspace::new(options)
            .update_toml()
            .expect("Expect new file to be made without issue");

        assert!(toml_file.exists());
    }

    #[test]
    fn create_a_new_toml_file_in_update_mode() {
        let test_root = ThreadTestPath::new_removed();

        let options = Options::new(test_root.to_str().unwrap(), FileExistsBehaviour::Update);
        let toml_file_error = Workspace::new(options)
            .update_toml()
            .expect_err("Expect new file to not be made in update mode");

        match toml_file_error {
            Error::GenericCreationError(_) => {}
            _ => assert!(false, "Wrong error enum value"),
        }
    }

    #[test]
    fn a_second_toml_file_in_the_same_location_is_not_allowed_and_doesnt_change_file_contents() {
        let test_root = ThreadTestPath::new_removed();
        let path = test_root.to_str().unwrap();

        let options = Options::new(path, FileExistsBehaviour::Halt);
        let toml_file = Workspace::new(options)
            .update_toml()
            .expect("Expect new file to be made without issue");
        assert!(toml_file.exists());

        overwrite_file(&toml_file, "Manually edited");

        let options = Options::new(path, FileExistsBehaviour::Halt);
        let second_toml_file_error = Workspace::new(options)
            .update_toml()
            .expect_err("Expect second file to not be created");

        match second_toml_file_error {
            Error::FileAlreadyExists => {}
            _ => assert!(false, "Wrong error enum value"),
        }

        assert!(toml_file.exists());

        let file_contents = read_file(&toml_file);

        assert_eq!("Manually edited", file_contents);
    }

    #[test]
    fn a_second_toml_file_in_the_same_location_is_allowed_with_overwrite_enabled() {
        let test_root = ThreadTestPath::new_removed();
        let path = test_root.to_str().unwrap();

        let options = Options::new(path, FileExistsBehaviour::Halt);
        let toml_file = Workspace::new(options)
            .update_toml()
            .expect("Expect new file to be made without issue");
        assert!(toml_file.exists());

        let options = Options::new(path, FileExistsBehaviour::Update);
        let second_toml_file = Workspace::new(options)
            .update_toml()
            .expect("Expect new file to be made without issue");

        assert!(second_toml_file.exists());
    }

    #[test]
    fn update_an_existing_toml() {
        let test_root = ThreadTestPath::new_removed();
        let path = test_root.to_str().unwrap();

        let options = Options::new(path, FileExistsBehaviour::Halt);
        let toml_file = Workspace::new(options)
            .update_toml()
            .expect("Expect new file to be made without issue");
        assert!(toml_file.exists());

        overwrite_file(&toml_file, "a = [\"d\", \"e\", \"f\"] # comment");

        let options = Options::new(path, FileExistsBehaviour::Update);
        let second_toml_file = Workspace::new(options)
            .update_toml()
            .expect("Expect second file to be updated");

        assert!(second_toml_file.exists());

        let file_contents = read_file(&second_toml_file);

        assert_eq!(
            r#"a = ["d", "e", "f"] # comment

[workspace]
members = [
    # List your crates here, e.g:
    # \"my-lib\",
]
"#,
            file_contents
        );
    }

    #[test]
    fn cant_update_an_existing_bad_toml() {
        let test_root = ThreadTestPath::new_removed();
        let path = test_root.to_str().unwrap();

        let options = Options::new(path, FileExistsBehaviour::Halt);
        let toml_file = Workspace::new(options)
            .update_toml()
            .expect("Expect new file to be made without issue");
        assert!(toml_file.exists());

        overwrite_file(&toml_file, "bad toml!");

        let options = Options::new(path, FileExistsBehaviour::Update);
        let second_toml_file_error = Workspace::new(options)
            .update_toml()
            .expect_err("Expect file to not parse");

        match second_toml_file_error {
            Error::ParseError => {}
            _ => assert!(false, "Wrong error enum value"),
        }

        let file_contents = read_file(&toml_file);

        assert_eq!("bad toml!", file_contents);
    }

    #[test]
    fn overwrite_an_existing_toml() {
        let test_root = ThreadTestPath::new_removed();
        let path = test_root.to_str().unwrap();

        let options = Options::new(path, FileExistsBehaviour::Halt);
        let toml_file = Workspace::new(options)
            .update_toml()
            .expect("Expect new file to be made without issue");
        assert!(toml_file.exists());

        overwrite_file(&toml_file, "a = [\"d\", \"e\", \"f\"] # comment");

        let options = Options::new(path, FileExistsBehaviour::Overwrite);
        let second_toml_file = Workspace::new(options)
            .update_toml()
            .expect("Expect second file to be updated");

        assert!(second_toml_file.exists());

        let file_contents = read_file(&second_toml_file);

        assert_eq!(
            r#"
[workspace]
members = [
    # List your crates here, e.g:
    # \"my-lib\",
]
"#,
            file_contents
        );
    }

    #[test]
    fn can_overwrite_an_existing_bad_toml() {
        let test_root = ThreadTestPath::new_removed();
        let path = test_root.to_str().unwrap();

        let options = Options::new(path, FileExistsBehaviour::Halt);
        let toml_file = Workspace::new(options)
            .update_toml()
            .expect("Expect new file to be made without issue");
        assert!(toml_file.exists());

        overwrite_file(&toml_file, "bad toml!");

        let options = Options::new(path, FileExistsBehaviour::Overwrite);
        let second_toml_file = Workspace::new(options)
            .update_toml()
            .expect("Expect second file to be updated");

        assert!(second_toml_file.exists());

        let file_contents = read_file(&second_toml_file);

        assert_eq!(
            r#"
[workspace]
members = [
    # List your crates here, e.g:
    # \"my-lib\",
]
"#,
            file_contents
        );
    }
}
