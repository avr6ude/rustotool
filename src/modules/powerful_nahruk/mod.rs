use crate::database::Database;
use crate::modules::BotModule;
use crate::config::Config;
use async_trait::async_trait;
use teloxide::{prelude::*, sugar::request::RequestReplyExt, types::Message};


pub struct PowerfulNahrukModule;

impl PowerfulNahrukModule {
  pub fn new() -> Self {
    Self
  }

  async fn check_nahruk(&self, msg: &str) -> String {
    let trigger_words = vec!["украина", "хохол", "хохл"];
    if trigger_words.iter().any(|&word| msg.contains(word)) {
      "Ваш нахрюк заблокирован ❌".to_string()
    } else {
      String::new()
    }
  }
}

#[async_trait]
impl BotModule for PowerfulNahrukModule {
  fn name(&self) -> &'static str {
    "Powerfull Nahruk"
  }

  fn commands(&self) -> Vec<(&'static str, &'static str)> {
    vec![]
  }

  async fn handle_command(
    &self,
    _bot: Bot,
    _msg: Message,
    _command: &str,
    _args: Vec<&str>,
    _db: &Database,
    _config: &Config,
  ) -> ResponseResult<()> {
    return Ok(());
  }

  async fn handle_message(&self, bot: Bot, msg: Message, _db: &Database, _config: &Config) -> ResponseResult<bool> {
    let nahruk = self.check_nahruk(&msg.text().unwrap_or("")).await;
    if !nahruk.is_empty() {
      bot
        .send_message(msg.chat.id, nahruk)
        .reply_to(msg.id)
        .await?;
      Ok(true)
    } else {
      Ok(false)
    }
  }
}
