use std::collections::HashMap;

use serde::{Deserialize, Serialize};
use url::Url;

#[derive(Clone, Default, Debug, Deserialize, Serialize, PartialEq, Eq)]
#[serde(deny_unknown_fields, rename_all = "kebab-case")]
pub struct RepodataChannelConfig {
    /// Disable JLAP compression for repodata.
    #[serde(alias = "disable_jlap")] // BREAK: remove to stop supporting snake_case alias
    #[serde(skip_serializing_if = "Option::is_none")]
    pub disable_jlap: Option<bool>,

    /// Disable bzip2 compression for repodata.
    #[serde(alias = "disable_bzip2")] // BREAK: remove to stop supporting snake_case alias
    #[serde(skip_serializing_if = "Option::is_none")]
    pub disable_bzip2: Option<bool>,

    /// Disable zstd compression for repodata.
    #[serde(alias = "disable_zstd")] // BREAK: remove to stop supporting snake_case alias
    #[serde(skip_serializing_if = "Option::is_none")]
    pub disable_zstd: Option<bool>,

    /// Disable the use of sharded repodata
    #[serde(skip_serializing_if = "Option::is_none")]
    pub disable_sharded: Option<bool>,
}

impl RepodataChannelConfig {
    pub fn is_empty(&self) -> bool {
        self.disable_jlap.is_none()
            && self.disable_bzip2.is_none()
            && self.disable_zstd.is_none()
            && self.disable_sharded.is_none()
    }

    pub fn merge(&self, other: Self) -> Self {
        Self {
            disable_jlap: self.disable_jlap.or(other.disable_jlap),
            disable_zstd: self.disable_zstd.or(other.disable_zstd),
            disable_bzip2: self.disable_bzip2.or(other.disable_bzip2),
            disable_sharded: self.disable_sharded.or(other.disable_sharded),
        }
    }
}

// impl From<RepodataChannelConfig> for SourceConfig {
//     fn from(value: RepodataChannelConfig) -> Self {
//         SourceConfig {
//             jlap_enabled: !value.disable_jlap.unwrap_or(false),
//             zstd_enabled: !value.disable_zstd.unwrap_or(false),
//             bz2_enabled: !value.disable_bzip2.unwrap_or(false),
//             sharded_enabled: !value.disable_sharded.unwrap_or(false),
//             cache_action: Default::default(),
//         }
//     }
// }

#[derive(Clone, Default, Debug, Serialize, Deserialize, PartialEq, Eq)]
#[serde(rename_all = "kebab-case")]
pub struct RepodataConfig {
    /// Default configuration for all channels.
    #[serde(flatten)]
    pub default: RepodataChannelConfig,

    /// Per-channel configuration for repodata.
    #[serde(flatten)]
    pub per_channel: HashMap<Url, RepodataChannelConfig>,
}

impl RepodataConfig {
    pub fn is_empty(&self) -> bool {
        self.default.is_empty() && self.per_channel.is_empty()
    }

    /// Merge the given RepodataConfig into the current one.
    /// `other` is mutable to allow for moving the values out of it.
    /// The given config will have higher priority
    pub fn merge(&self, mut other: Self) -> Self {
        let mut per_channel: HashMap<_, _> = self
            .per_channel
            .clone()
            .into_iter()
            .map(|(url, config)| {
                let other_config = other.per_channel.remove(&url).unwrap_or_default();
                (url, config.merge(other_config))
            })
            .collect();

        per_channel.extend(other.per_channel);

        Self {
            default: self.default.merge(other.default),
            per_channel,
        }
    }
}