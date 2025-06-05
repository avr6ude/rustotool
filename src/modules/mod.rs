use crate::database::Database;
use async_trait::async_trait;
use std::collections::HashMap;
use teloxide::{prelude::*, types::Message};
use crate::config::Config;

#[async_trait]
pub trait BotModule: Send + Sync {
    fn name(&self) -> &'static str;
    fn commands(&self) -> Vec<(&'static str, &'static str)>;
    async fn handle_command(
        &self,
        bot: Bot,
        msg: Message,
        command: &str,
        args: Vec<&str>,
        db: &Database,
        config: &Config,
    ) -> ResponseResult<()>;
    async fn handle_message(
        &self,
        _bot: Bot,
        _msg: Message,
        _db: &Database,
        _config: &Config
    ) -> ResponseResult<bool> {
        Ok(false)
    }
}

pub struct ModuleManager {
    modules: HashMap<String, Box<dyn BotModule>>,
}

impl ModuleManager {
    pub fn new() -> Self {
        Self {
            modules: HashMap::new(),
        }
    }

    pub fn register_module(&mut self, module: Box<dyn BotModule>) {
        let name = module.name().to_string();
        self.modules.insert(name, module);
    }

    pub async fn handle_command(
        &self,
        bot: Bot,
        msg: Message,
        command: &str,
        args: Vec<&str>,
        db: &Database,
        config: &Config
    ) -> ResponseResult<bool> {
        for module in self.modules.values() {
            if module.commands().iter().any(|(cmd, _)| *cmd == command) {
                module
                    .handle_command(bot.clone(), msg.clone(), command, args.clone(), db, config)
                    .await?;
                return Ok(true);
            }
        }
        Ok(false)
    }

    pub async fn handle_message(
        &self,
        bot: Bot,
        msg: Message,
        db: &Database,
        config: &Config
    ) -> ResponseResult<()> {
        for module in self.modules.values() {
            if module.handle_message(bot.clone(), msg.clone(), db, config).await? {
                break; // Stop at first module that handles the message
            }
        }
        Ok(())
    }

    pub fn get_all_commands(&self) -> Vec<String> {
        let mut result = Vec::new();
        for module in self.modules.values() {
            result.push(format!("{}:", module.name()));
            for (cmd, desc) in module.commands() {
                result.push(format!("/{} - {}", cmd, desc));
            }
            result.push(String::new())
        }
        result
    }
}

pub mod pig_game;
pub mod powerful_nahruk;
pub mod reactions;
