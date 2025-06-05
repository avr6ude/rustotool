use serde::{Deserialize, Serialize};
use std::fs;

#[derive(Debug, Deserialize, Serialize, Clone)]
pub struct GPTConfig {
    #[serde(rename = "LLM_API_URL")]
    pub llm_api_url: String,
    #[serde(rename = "LLM_API_TOKEN")]
    pub llm_api_token: String,
}

#[derive(Debug, Deserialize, Serialize, Clone)]
pub struct GameConfig {
    #[serde(rename = "FEED_DELAY")]
    pub feed_delay: u64,
    #[serde(rename = "BASE_GROWTH")]
    pub base_growth: f64,
    #[serde(rename = "RANK_FACTOR")]
    pub rank_factor: f64,
    #[serde(rename = "WEIGHT_FACTOR")]
    pub weight_factor: f64,
    #[serde(rename = "SALO_DELAY")]
    pub salo_delay: u64,
    #[serde(rename = "MAX_ITEMS")]
    pub max_items: u32,
    #[serde(rename = "BASE_PILLS_CHANCE")]
    pub base_pills_chance: f64,
    #[serde(rename = "BASE_PILLS_CHANCE_GROW")]
    pub base_pills_chance_grow: f64,
}

#[derive(Debug, Deserialize, Serialize, Clone)]
pub struct Config {
    pub gpt: GPTConfig,
    pub game: GameConfig,
    pub database_url: Option<String>,
}

impl Config {
    pub fn load() -> Result<Self, Box<dyn std::error::Error>> {
        let config_content =
            fs::read_to_string("config.yaml").or_else(|_| fs::read_to_string("config.yml"))?;

        let config: Config = serde_yaml::from_str(&config_content)?;
        Ok(config)
    }

    pub fn load_or_default() -> Self {
        Self::load().unwrap_or_else(|e| {
            log::warn!("Failed to load config: {}, using defaults", e);
            Config {
                gpt: GPTConfig {
                    llm_api_url: std::env::var("LLM_API_URL").unwrap_or_default(),
                    llm_api_token: std::env::var("LLM_API_TOKEN").unwrap_or_default(),
                },
                game: GameConfig {
                    base_growth: 0.5,
                    rank_factor: 0.5,
                    weight_factor: 0.5,
                    feed_delay: 4,
                    salo_delay: 8,
                    max_items: 15,
                    base_pills_chance: 0.33,
                    base_pills_chance_grow: 0.75,
                },
                database_url: std::env::var("dat").ok(),
            }
        })
    }
}
