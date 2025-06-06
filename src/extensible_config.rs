use std::path::Path;

use miette::IntoDiagnostic;
use serde::{de::DeserializeOwned, Deserialize, Serialize};

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct BuildConfig {
    pub version: Option<String>,
    pub author: Option<String>,
    pub description: Option<String>,
}

#[derive(Default, Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct ConfigBase<T> {
    #[serde(default)]
    pub build: Option<BuildConfig>,

    #[serde(flatten)]
    pub extensions: T,
}

pub trait Config: Serialize + for<'de> Deserialize<'de> + std::fmt::Debug + Clone + PartialEq + Eq + Default{
    /// Get the name of the extension.
    fn get_extension_name(&self) -> String;

    /// Merge another configuration (file) into this one.
    /// Note: the "other" configuration should take priority over the current one.
    fn merge_config(&mut self, other: &Self) -> Result<(), miette::Error>;

    /// Validate the configuration.
    fn validate(&self) -> Result<(), miette::Error>;

    /// Get the valid keys of the configuration.
    fn keys(&self) -> &[&str];
}

impl<T> ConfigBase<T>
where
    T: Config + DeserializeOwned,
{
    pub fn load_from_files<I, P>(paths: I) -> Result<Self, miette::Error>
    where
        I: IntoIterator<Item = P>,
        P: AsRef<Path>,
    {
        let mut config = ConfigBase::<T>::default();

        for path in paths {
            let content = std::fs::read_to_string(path.as_ref()).into_diagnostic()?;
            let other: ConfigBase<T> = toml::from_str(&content).into_diagnostic()?;
            config.merge_config(&other)?;
        }

        // config.validate().into_diagnostic()?;
        Ok(config)
    }
}


impl<T> Config for ConfigBase<T>
where
    T: Config + Default,
{
    fn get_extension_name(&self) -> String {
        "base".to_string()
    }

    fn merge_config(&mut self, other: &Self) -> Result<(), miette::Error> {
        if let Some(build) = &self.build {
            // TODO implement
        } else {
            self.build = other.build.clone();
        }
        
        return Ok(());
    }

    fn validate(&self) -> Result<(), miette::Error> {
        Ok(())
    }

    fn keys(&self) -> &[&str] {
        return &["build.version", "build.author", "build.description"];
    }
}

pub fn load_config<T: for<'de> Deserialize<'de>>(config_file: &str) -> Result<ConfigBase<T>, Box<dyn std::error::Error>> {
    let config_content = std::fs::read_to_string(config_file)?;
    let config: ConfigBase<T> = toml::from_str(&config_content)?;
    Ok(config)
}