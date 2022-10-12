use std::{collections::HashMap, fs, path};

/// The name of the config file to use.
pub(crate) static FILE_NAME: &str = "mksite.toml";

/// TODO: doc
#[derive(Debug, PartialEq, Default, serde::Deserialize, serde::Serialize)]
pub(crate) struct Config {
    pub metadata: HashMap<String, toml::Value>,
    pub processors: HashMap<String, HashMap<String, Processor>>,
}

/// Loads the `mksite.toml` config file from the current directory.
pub(crate) fn load() -> anyhow::Result<Config> {
    let res = toml::from_str(&fs::read_to_string(FILE_NAME)?)?;
    Ok(res)
}

/// TODO: doc
#[derive(Debug, PartialEq, serde::Deserialize, serde::Serialize)]
#[serde(untagged)]
pub(crate) enum Processor {
    Single(String),
    Chain(Vec<String>),
}

/// Returns true if the `mksite.toml` config file exists in the current directory.
pub fn exists() -> bool {
    path::Path::new(FILE_NAME).exists()
}

/// Generates the `mksite.toml` config file in the specified directory.
/// `path` must be a directory.
pub(crate) fn generate_config_file(path: &impl AsRef<path::Path>) -> anyhow::Result<()> {
    anyhow::ensure!(
        fs::metadata(path)?.is_dir(),
        "{:?} is not a directory",
        path.as_ref()
    );

    anyhow::ensure!(
        !path.as_ref().join(FILE_NAME).exists(),
        "Config file {FILE_NAME} already exists"
    );

    fs::write(
        path.as_ref().join(FILE_NAME),
        toml::to_string(&Config::default())?,
    )?;

    Ok(())
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn serialize_config() {
        let cfg: Config = toml::from_str(
            r#"
            [metadata]
            foo = "bar"
            year = 2022
            bar = false

            [processors]
            md.html = "md2html"
            scd.html = [ "scdoc", "pandoc -f man -t html" ]
            "#,
        )
        .unwrap();

        assert_eq!(
            cfg,
            Config {
                metadata: maplit::hashmap! {
                    "foo".into() => toml::Value::String("bar".into()),
                    "year".into() => toml::Value::Integer(2022),
                    "bar".into() => toml::Value::Boolean(false),
                },
                processors: maplit::hashmap! {
                    "md".into() => maplit::hashmap! {
                        "html".into() => Processor::Single("md2html".into()),
                    },
                    "scd".into() => maplit::hashmap! {
                        "html".into() => Processor::Chain(vec!["scdoc".into(), "pandoc -f man -t html".into()])
                    }
                }
            }
        )
    }
}
