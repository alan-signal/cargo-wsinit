use toml_edit::*;

const EMPTY_FILE_TEMPLATE: &str = r#"[workspace]

members = [
]
"#;

const EMPTY_MEMBERS_COMMENT_BLOCK: &str = r#"commented_array = [
    # List your crates here, e.g:
    # \"my-lib\",
]
"#;

pub fn toml_update<T>(contents: &str, sub_projects: &[T]) -> Result<String, TomlError>
where
    T: AsRef<str> + Into<Value> + Clone,
{
    let contents = if contents.is_empty() {
        EMPTY_FILE_TEMPLATE
    } else {
        contents
    };

    let mut doc = contents.parse::<Document>()?;

    let mut array = Array::default();

    for (index, lib) in sub_projects.iter().enumerate() {
        if index == sub_projects.len() - 1 {
            array
                .push_formatted(decorated(lib.clone().into(), "\n    ", ",\n"))
                .unwrap();
        } else {
            array
                .push_formatted(decorated(lib.clone().into(), "\n    ", ""))
                .unwrap();
        }
    }

    if sub_projects.is_empty() {
        let doc = EMPTY_MEMBERS_COMMENT_BLOCK
            .parse::<Document>()
            .expect("invalid doc");
        array = doc["commented_array"].as_array().unwrap().clone();
    }

    if doc["workspace"].is_none() {
        doc["workspace"] = table();
    }
    doc["workspace"]["members"] = value(array);

    Ok(doc.to_string())
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn no_members() {
        let toml = "";

        let new_toml = toml_update::<String>(toml, &[]).unwrap();

        assert_eq!(
            new_toml,
            r#"[workspace]

members = [
    # List your crates here, e.g:
    # \"my-lib\",
]
"#
        );
    }

    #[test]
    fn one_member() {
        let toml = "";

        let new_toml = toml_update(toml, vec!["lib1"].as_slice()).unwrap();

        assert_eq!(
            new_toml,
            r#"[workspace]

members = [
    "lib1",
]
"#
        );
    }

    #[test]
    fn two_members() {
        let toml = "";

        let new_toml = toml_update(toml, vec!["lib1", "lib2"].as_slice()).unwrap();

        assert_eq!(
            new_toml,
            r#"[workspace]

members = [
    "lib1",
    "lib2",
]
"#
        );
    }

    #[test]
    fn modify_existing_doc() {
        let toml = r#"[table]
value = [1, 2, 3]
"#;

        let new_toml = toml_update(toml, vec!["lib1"].as_slice()).unwrap();

        assert_eq!(
            new_toml,
            r#"[table]
value = [1, 2, 3]

[workspace]
members = [
    "lib1",
]
"#
        );
    }

    #[test]
    fn modify_existing_doc_with_own_libs() {
        let toml = r#"[table]
value = [1, 2, 3]

[workspace]
members = ["libx",]
somekey = "hi" # some random comment
"#;

        let new_toml = toml_update(toml, vec!["liba"].as_slice()).unwrap();

        assert_eq!(
            new_toml,
            r#"[table]
value = [1, 2, 3]

[workspace]
members = [
    "liba",
]
somekey = "hi" # some random comment
"#
        );
    }
}
