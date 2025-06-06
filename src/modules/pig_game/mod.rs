use crate::modules::BotModule;
use crate::config::Config;
use crate::database::Database;
use async_trait::async_trait;
use teloxide::{
    prelude::*,
    types::{Message, CallbackQuery}
};

mod callbacks;
mod commands;
mod keyboards;
mod helpers;

pub struct PigGameModule;

impl PigGameModule {
    pub fn new() -> Self {
        Self
    }
}

#[async_trait]
impl BotModule for PigGameModule {
    fn name(&self) -> &'static str {
        "Pig Game"
    }

    fn commands(&self) -> Vec<(&'static str, &'static str)> {
        vec![
            ("pig", "Создать новую свинью"),
            ("grow", "Покормить свинью"),
            ("my", "Посмотреть информацию о своей свинье"),
            ("pigstats", "Посмотреть статистику свиней"),
            ("top", "Посмотреть топ свиней по весу"),
            ("name", "Поменять имя")
        ]
    }

    async fn handle_command(
        &self,
        bot: Bot,
        msg: Message,
        command: &str,
        args: Vec<&str>,
        db: &Database,
        config: &Config,
    ) -> ResponseResult<()> {
        self.handle_pig_command(bot, msg, command, args, db, config).await
    }

    async fn handle_callback_query(
          &self,
          bot: Bot,
          query: CallbackQuery,
          db: &Database,
          config: &Config,
      ) -> ResponseResult<()> {
          self.handle_callback_query(bot, query, db, config).await
      }
}
