use teloxide::types::MessageId;
use teloxide::{
    prelude::*,
    types::{ChatMemberStatus, CallbackQuery}
};
use crate::database::Database;
use crate::config::Config;

impl super::PigGameModule {
    pub async fn handle_callback_query(
        &self,
        bot: Bot,
        query: CallbackQuery,
        db: &Database,
        config: &Config,
    ) -> ResponseResult<()> {
        if let Some(data) = query.data {
            if let Some(message) = &query.message {
                let chat_id = message.chat().id.0;
                let user_id = query.from.id.0 as i64;

                match data.as_str() {
                    data if data.starts_with("grow:") => {
                        let original_user_id: i64 = data.strip_prefix("grow:").unwrap().parse().unwrap_or(0);

                        if user_id != original_user_id {
                            bot.answer_callback_query(&query.id)
                                .text("🖕🤣 Это не твой гров!")
                                .await?;
                            return Ok(());
                        }

                        match db.get_pig(chat_id, user_id).await {
                            Ok(Some(mut pig)) => {
                                match self.feed_pig(&mut pig, db, config).await {
                                    Ok(grow_message) => {
                                        bot.edit_message_text(
                                            message.chat().id,
                                            message.id(),
                                            grow_message,
                                        )
                                        .reply_markup(self.create_grow_keyboard(user_id))
                                        .await?;
                                    }
                                    Err(e) => {
                                        log::error!("Failed to feed pig: {}", e);
                                        bot.answer_callback_query(&query.id)
                                            .text("Ошибка при кормлении свиньи")
                                            .await?;
                                    }
                                }
                            }
                            Ok(None) => {
                                bot.answer_callback_query(&query.id)
                                    .text("У вас нет свиньи!")
                                    .await?;
                            }
                            Err(e) => {
                                log::error!("Database error: {}", e);
                                bot.answer_callback_query(&query.id)
                                    .text("Ошибка базы данных")
                                    .await?;
                            }
                        }
                    }

                    data if data.starts_with("remove:") => {
                        let chat_id = message.chat().id;
                        let callback_message_id = message.id();

                        let bot_user = bot.get_me().await?;
                        let bot_member = bot.get_chat_member(chat_id, bot_user.id).await;


                        let can_delete = match bot_member {
                            Ok(member) => match member.status() {
                                ChatMemberStatus::Owner => true,
                                ChatMemberStatus::Administrator { .. } => true, // assume admin can delete
                                _ => false,
                            },
                            Err(_) => false,
                        };

                        bot.delete_message(
                            message.chat().id,
                            callback_message_id,
                        ).await?;

                        if can_delete {
                            if let Some(original_msg_id_str) = data.strip_prefix("remove:") {
                                if let Ok(id) = original_msg_id_str.parse::<i32>() {
                                    let original_msg_id = MessageId(id);
                                    bot.delete_message(message.chat().id, original_msg_id).await.ok();
                                }
                            }
                        }
                    }

                    data if data.starts_with("back:") => {
                        let original_user_id: i64 = data.strip_prefix("back:").unwrap().parse().unwrap_or(0);
                        if user_id != original_user_id {
                            bot.answer_callback_query(&query.id)
                                .text("🖕🤣 Это не твой хряк!")
                                .await?;
                            return Ok(());
                        }

                        match db.get_pig(chat_id, user_id).await {
                            Ok(Some(pig)) => {
                                let response_text = self.format_pig_info(&pig, db).await;

                                bot.edit_message_text(
                                    message.chat().id,
                                    message.id(),
                                    response_text,
                                )
                                .reply_markup(self.create_pig_keyboard(user_id, message.id().0))
                                .await?;
                            }
                            Ok(None) => {
                                bot.answer_callback_query(&query.id)
                                    .text("У вас нет свиньи!")
                                    .await?;
                            }
                            Err(e) => {
                                log::error!("Database error: {}", e);
                                bot.answer_callback_query(&query.id)
                                    .text("Ошибка базы данных")
                                    .await?;
                            }
                        }
                    }

                    _ => {
                        bot.answer_callback_query(&query.id)
                            .text("Неизвестная команда")
                            .await?;
                    }
                }
            }
        }

        bot.answer_callback_query(&query.id).await?;
        Ok(())
    }
}
