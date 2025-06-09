use serde::{Deserialize, Serialize};
use url::Url;

#[derive(Clone, Debug, Deserialize, Serialize, Default, PartialEq, Eq)]
#[serde(rename_all = "kebab-case")]
pub struct ProxyConfig {
    /// https proxy.
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub https: Option<Url>,
    /// http proxy.
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
    pub fn merge(&self, other: Self) -> Self {
        Self {
            https: other.https.as_ref().or(self.https.as_ref()).cloned(),
            http: other.http.as_ref().or(self.http.as_ref()).cloned(),
            non_proxy_hosts: if other.is_default() {
                self.non_proxy_hosts.clone()
            } else {
                other.non_proxy_hosts.clone()
            },
        }
    }
}
