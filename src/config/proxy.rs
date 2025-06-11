use serde::{Deserialize, Serialize};
use url::Url;

use crate::config::Config;

#[derive(Clone, Debug, Deserialize, Serialize, Default, PartialEq, Eq)]
#[serde(rename_all = "kebab-case")]
pub struct ProxyConfig {
    /// The HTTPS proxy to use
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub https: Option<Url>,

    /// The HTTP proxy to use
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub http: Option<Url>,

    /// A list of no proxy pattern
    #[serde(default)]
    #[serde(skip_serializing_if = "Vec::is_empty")]
    pub non_proxy_hosts: Vec<String>,
}

impl ProxyConfig {
    pub fn is_default(&self) -> bool {
        self.https.is_none() && self.https.is_none() && self.non_proxy_hosts.is_empty()
    }
}

impl Config for ProxyConfig {
    fn get_extension_name(&self) -> String {
        "proxy".to_string()
    }

    fn merge_config(self, other: &Self) -> Result<Self, miette::Error> {
        Ok(Self {
            https: other.https.as_ref().or(self.https.as_ref()).cloned(),
            http: other.http.as_ref().or(self.http.as_ref()).cloned(),
            non_proxy_hosts: if other.is_default() {
                self.non_proxy_hosts.clone()
            } else {
                other.non_proxy_hosts.clone()
            },
        })
    }

    fn validate(&self) -> Result<(), miette::Error> {
        if self.https.is_none() && self.http.is_none() {
            return Err(miette::miette!(
                "At least one of https or http proxy must be set"
            ));
        }
        Ok(())
    }

    fn keys(&self) -> Vec<String> {
        vec![
            "https".to_string(),
            "http".to_string(),
            "non-proxy-hosts".to_string(),
        ]
    }
}
