use serde::Deserialize;
use std::collections::BTreeMap;
use std::path::Path;

#[derive(Clone, Debug, Deserialize)]
#[serde(default)]
pub struct Weights {
    #[serde(rename = "S")]
    pub small: u32,
    #[serde(rename = "M")]
    pub medium: u32,
    #[serde(rename = "L")]
    pub large: u32,
    #[serde(rename = "XL")]
    pub extra_large: u32,
}

impl Default for Weights {
    fn default() -> Self {
        Self {
            small: 1,
            medium: 3,
            large: 8,
            extra_large: 20,
        }
    }
}

impl Weights {
    pub fn table(&self) -> BTreeMap<&'static str, u32> {
        BTreeMap::from([
            ("S", self.small),
            ("M", self.medium),
            ("L", self.large),
            ("XL", self.extra_large),
        ])
    }
}

#[derive(Clone, Debug, Deserialize)]
#[serde(default)]
pub struct Policy {
    pub require_independent_verification: bool,
    pub verify_freezes_and_adr_always: bool,
    pub verify_gate_tasks: bool,
    pub fan_in_warning: usize,
    pub workstream_lines_warning: usize,
    pub task_lines_warning: usize,
}

impl Default for Policy {
    fn default() -> Self {
        Self {
            require_independent_verification: false,
            verify_freezes_and_adr_always: true,
            verify_gate_tasks: true,
            fan_in_warning: 60,
            workstream_lines_warning: 6000,
            task_lines_warning: 100,
        }
    }
}

#[derive(Clone, Debug, Default, Deserialize)]
#[serde(default)]
pub struct Config {
    pub weights: Weights,
    pub policy: Policy,
}

impl Config {
    pub fn load(root: &Path) -> Self {
        let path = root.join("roadmap.toml");
        let Ok(text) = std::fs::read_to_string(&path) else {
            return Self::default();
        };
        toml::from_str::<Self>(&text).unwrap_or_default()
    }

    pub fn weight_of(&self, size: &str) -> u32 {
        match size {
            "S" => self.weights.small,
            "M" => self.weights.medium,
            "L" => self.weights.large,
            "XL" => self.weights.extra_large,
            _ => 0,
        }
    }
}
