use std::{collections::HashMap, path::PathBuf};

use serde::Deserialize;
use ts_rs::TS;

#[derive(Debug, Deserialize, Clone, TS)]
#[ts(export, export_to = "types.d.ts")]
#[serde(rename_all = "camelCase")]
pub struct Config {
    #[serde(default)]
    pub base_url: PathBuf,
    pub paths: HashMap<String, Vec<String>>,
}
