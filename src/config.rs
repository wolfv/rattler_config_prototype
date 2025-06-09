use std::{collections::HashMap, path::{Path, PathBuf}};

use miette::IntoDiagnostic;
use rattler_conda_types::NamedChannelOrUrl;
use serde::{de::DeserializeOwned, Deserialize, Serialize};
use url::Url;

use crate::config::{build::BuildConfig, concurreny::ConcurrencyConfig, proxy::ProxyConfig, repodata_config::RepodataConfig};

mod s3;
mod concurreny;
mod repodata_config;
mod proxy;
mod build;


#[derive(Default, Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct ConfigBase<T> {
    #[serde(default)]
    #[serde(alias = "default_channels")] // BREAK: remove to stop supporting snake_case alias
    #[serde(skip_serializing_if = "Vec::is_empty")]
    pub default_channels: Vec<NamedChannelOrUrl>,

    /// Path to the file containing the authentication token.
    #[serde(default)]
    #[serde(alias = "authentication_override_file")] // BREAK: remove to stop supporting snake_case alias
    #[serde(skip_serializing_if = "Option::is_none")]
    pub authentication_override_file: Option<PathBuf>,

    /// If set to true, pixi will not verify the TLS certificate of the server.
    #[serde(default)]
    #[serde(alias = "tls_no_verify")] // BREAK: remove to stop supporting snake_case alias
    #[serde(skip_serializing_if = "Option::is_none")]
    pub tls_no_verify: Option<bool>,

    #[serde(default)]
    #[serde(skip_serializing_if = "HashMap::is_empty")]
    pub mirrors: HashMap<Url, Vec<Url>>,

    #[serde(default)]
    pub build: Option<BuildConfig>,

    /// Configuration for repodata fetching.
    #[serde(alias = "repodata_config")] // BREAK: remove to stop supporting snake_case alias
    #[serde(default, skip_serializing_if = "RepodataConfig::is_empty")]
    pub repodata_config: RepodataConfig,

    /// Configuration for the concurreny of rattler.
    #[serde(default)]
    #[serde(skip_serializing_if = "ConcurrencyConfig::is_default")]
    pub concurrency: ConcurrencyConfig,

    /// Https/Http proxy configuration for pixi
    #[serde(default)]
    #[serde(skip_serializing_if = "ProxyConfig::is_default")]
    pub proxy_config: ProxyConfig,

    #[serde(flatten)]
    pub extensions: T,

    #[serde(skip)]
    #[serde(alias = "loaded_from")] // BREAK: remove to stop supporting snake_case alias
    pub loaded_from: Vec<PathBuf>,
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

        self.repodata_config.merge(other.repodata_config.clone());
        
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