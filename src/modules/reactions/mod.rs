use crate::config::Config;
use crate::database::Database;
use crate::modules::BotModule;
use async_trait::async_trait;
use rand::prelude::*;
use teloxide::types::ReactionType;
use teloxide::{prelude::*, types::Message};

pub struct ReactionsModule;

impl ReactionsModule {
  pub fn new() -> Self {
    Self
  }

  fn react_to_message(&self) -> &'static str {
    let mut rng = rand::rng();
    let reactions = vec!["🤡", "💩", "🤣", "💊", "😁", "😨"];
    if rand::random::<u8>() < 25 {
      reactions[rng.random_range(0..reactions.len())]
    } else {
      ""
    }
  }
}

#[async_trait]
impl BotModule for ReactionsModule {
  fn name(&self) -> &'static str {
    "Reactions Module"
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

  async fn handle_message(
    &self,
    bot: Bot,
    msg: Message,
    _db: &Database,
    _config: &Config,
  ) -> ResponseResult<bool> {
    let reaction = self.react_to_message();
    if !reaction.is_empty() {
      bot
        .set_message_reaction(msg.chat.id, msg.id)
        .reaction(vec![ReactionType::Emoji {
          emoji: reaction.to_string(),
        }])
        .await?;
      return Ok(true);
    } else {
      Ok(false)
    }
  }
}
