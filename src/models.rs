use serde::{Deserialize, Serialize};
use std::collections::HashMap;

#[derive(Debug, Deserialize, Serialize, Default)]
pub struct Config {
    #[serde(default)]
    pub tools: Vec<Tool>,
    #[serde(default)]
    pub cheats: Vec<CheatSheet>,
}

#[derive(Debug, Deserialize, Serialize, Clone)]
pub struct Tool {
    pub id: String,
    pub name: String,
    pub command: String,
}

#[derive(Debug, Deserialize, Serialize, Clone)]
pub struct CheatSheet {
    pub name: String,
    pub content: String,
}

#[derive(Debug, Serialize, Deserialize, Default)]
pub struct History {
    pub usage: HashMap<String, u32>,
}
