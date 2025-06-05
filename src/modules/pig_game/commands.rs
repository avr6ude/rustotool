use unicode_segmentation::UnicodeSegmentation;
use teloxide::{
    prelude::*,
    sugar::request::RequestReplyExt,
    types::Message,
};
use crate::database::Database;
use crate::config::Config;


impl super::PigGameModule {
    pub async fn handle_pig_command(
        &self,
        bot: Bot,
        msg: Message,
        command: &str,
        args: Vec<&str>,
        db: &Database,
        config: &Config,
    ) -> ResponseResult<()> {
        let chat_id = msg.chat.id.0;
        let user_id = msg.from.as_ref().map(|u| u.id.0 as i64).unwrap_or(0);
        let username = msg
            .from
            .as_ref()
            .and_then(|u| u.username.as_ref())
            .map(|s| s.as_str())
            .unwrap_or("Unknown");

        match command {
            "pig" => {
                let pig_name = if args.is_empty() {
                    self.generate_default_pig_name()
                } else {
                    args.join(" ")
                };

                match db.get_pig(chat_id, user_id).await {
                    Ok(Some(existing_pig)) => {
                        bot.send_message(
                            msg.chat.id,
                            format!(
                                "У вас уже есть свинья: {} (вес: {})",
                                existing_pig.name, existing_pig.weight
                            ),
                        )
                        .await?;
                    }
                    Ok(None) => {
                        match self
                            .create_new_pig(chat_id, user_id, username, &pig_name, db)
                            .await
                        {
                            Ok(pig) => {
                                bot.send_message(
                                    msg.chat.id,
                                    format!(
                                        "🐷 Поздравляем! {} создал свинью: {} (вес: {})",
                                        username, pig.name, pig.weight
                                    ),
                                )
                                .await?;
                            }
                            Err(e) => {
                                log::error!("Failed to create pig: {}", e);
                                bot.send_message(msg.chat.id, "Ошибка при создании свиньи")
                                    .await?;
                            }
                        }
                    }
                    Err(e) => {
                        log::error!("Database error: {}", e);
                        bot.send_message(msg.chat.id, "Ошибка базы данных").await?;
                    }
                }
            }

            "grow" | "гров" => {
                let pig_name = if args.is_empty() {
                    self.generate_default_pig_name()
                } else {
                    args.join(" ")
                };

                match db.get_pig(chat_id, user_id).await {
                    Ok(Some(mut pig)) => match self.feed_pig(&mut pig, db, config).await {
                        Ok(message) => {
                            bot.send_message(msg.chat.id, message).await?;
                        }
                        Err(e) => {
                            log::error!("Failed to feed pig: {}", e);
                            bot.send_message(msg.chat.id, "Ошибка при кормлении свиньи")
                                .await?;
                        }
                    },
                    Ok(None) => {
                        match self.create_new_pig(chat_id, user_id, username, &pig_name, db).await {
                            Ok(mut pig) => {
                                match self.feed_pig(&mut pig, db, config).await {
                                    Ok(message) => {
                                        bot.send_message(msg.chat.id, message).await?;
                                    }
                                    Err(e) => {
                                        log::error!("Failed to feed pig: {}", e);
                                        bot.send_message(msg.chat.id, "Ошибка при кормлении свиньи")
                                            .await?;
                                    }
                                };
                            }
                            Err(e) => {
                                log::error!("Failed to create pig: {}", e);
                                bot.send_message(msg.chat.id, "Ошибка при создании свиньи")
                                    .await?;
                            }
                        }
                    }
                    Err(e) => {
                        log::error!("Database error: {}", e);
                        bot.send_message(msg.chat.id, "Ошибка базы данных").await?;
                    }
                }
            }

            "my" => match db.get_pig(chat_id, user_id).await {
                Ok(Some(pig)) => {
                    let message = self.format_pig_info(&pig, db).await;
                    bot.send_message(msg.chat.id, message)
                        .reply_markup(self.create_pig_keyboard(user_id, msg.id.0))
                        .await?;
                }
                Ok(None) => {
                    bot.send_message(
                        msg.chat.id,
                        "У вас нет свиньи! Создайте её командой /pig <имя>",
                    )
                    .await?;
                }
                Err(e) => {
                    log::error!("Database error: {}", e);
                    bot.send_message(msg.chat.id, "Ошибка базы данных").await?;
                }
            }

            "pigstats" => {
                if !args.is_empty() {
                    let search_name = args.join(" ");
                    match db.find_pig_by_name(chat_id, &search_name).await {
                        Ok(pigs) if !pigs.is_empty() => {
                            let pig = &pigs[0];
                            let message = format!(
                                "🐷 {}\n\
                                 👤 Владелец: {}\n\
                                 💪 Вес: {}\n\
                                 🏠 Сарай: {}",
                                pig.name, pig.owner_name, pig.weight, pig.barn
                            );
                            bot.send_message(msg.chat.id, message).await?;
                        }
                        Ok(_) => {
                            bot.send_message(
                                msg.chat.id,
                                format!("Свинья с именем '{}' не найдена", search_name),
                            )
                            .await?;
                        }
                        Err(e) => {
                            log::error!("Database error: {}", e);
                            bot.send_message(msg.chat.id, "Ошибка базы данных").await?;
                        }
                    }
                } else {
                    match db.get_pig(chat_id, user_id).await {
                        Ok(Some(pig)) => {
                            let message = format!(
                                "🐷 Ваша свинья: {}\n\
                                 💪 Вес: {}\n\
                                 🏠 Сарай: {}",
                                pig.name, pig.weight, pig.barn
                            );
                            bot.send_message(msg.chat.id, message).await?;
                        }
                        Ok(None) => {
                            bot.send_message(msg.chat.id, "У вас нет свиньи!").await?;
                        }
                        Err(e) => {
                            log::error!("Database error: {}", e);
                            bot.send_message(msg.chat.id, "Ошибка базы данных").await?;
                        }
                    }
                }
            }

            "top" => {
                match db.get_chat_pigs_ranked(chat_id).await {
                    Ok(pigs) => {
                        if pigs.is_empty() {
                            bot.send_message(msg.chat.id, "В этом чате пока нет свиней 🐖").await?;
                        } else {
                            let top_pigs: Vec<String> = pigs
                                .iter()
                                .take(5)
                                .enumerate()
                                .map(|(i, pig)| {
                                    let position = i + 1;
                                    let medal = match position {
                                        1 => "🥇",
                                        2 => "🥈",
                                        3 => "🥉",
                                        _ => "🏅",
                                    };
                                    format!("{} {}. {} - {} кг (владелец: {}) 🐖", medal, position, pig.name, pig.weight, pig.owner_name)
                                })
                                .collect();

                            let message = format!("🏆 Топ 5 свиней в чате:\n{}", top_pigs.join("\n"));
                            bot.send_message(msg.chat.id, message).await?;
                        }
                    }
                    Err(e) => {
                        log::error!("Database error: {}", e);
                        bot.send_message(msg.chat.id, "Ошибка базы данных").await?;
                    }
                }
            }

            "name" => {
                if args.is_empty() {
                    bot.send_message(msg.chat.id, "Введи имя, еблан").reply_to(msg.id).await?;
                } else {
                    let new_name = args.join(" ");
                    if new_name.as_str().graphemes(true).count() >= 32 {
                        bot.send_message(msg.chat.id, "У тебя хряк весит меньше, чем твое имя. Придумай что-то короче 32 буковок, блядина.")
                            .reply_to(msg.id)
                            .await?;
                    } else {
                        match db.get_pig(chat_id, user_id).await {
                            Ok(Some(_)) => {
                                match db.update_pig_name(chat_id, user_id, &new_name).await {
                                    Ok(_) => {
                                        bot.send_message(msg.chat.id,
                                            format!("Теперь вашего хряка зовут {}", new_name))
                                        .reply_to(msg.id)
                                        .await?;
                                    }
                                    Err(e) => {
                                        log::error!("Database error: {}", e);
                                        bot.send_message(msg.chat.id, "Какая-то хуйня случилась. Пиши админу, блять").await?;
                                    }
                                }
                            }
                            Ok(None) => {
                                let owner_name = msg.from.map(|u| u.full_name()).unwrap_or_else(|| "Unknown".to_string());
                                match self.create_new_pig(chat_id, user_id, &owner_name, &new_name, db).await {
                                    Ok(_) => {
                                        bot.send_message(msg.chat.id,
                                            format!("Создана новая свинья с именем '{}'! 🐷", new_name))
                                            .reply_to(msg.id)
                                            .await?;
                                    }
                                    Err(e) => {
                                        log::error!("Failed to create pig: {}", e);
                                        bot.send_message(msg.chat.id, "Ошибка при создании свиньи")
                                            .reply_to(msg.id)
                                            .await?;
                                    }
                                }
                            }
                            Err(e) => {
                                log::error!("Database error: {}", e);
                                bot.send_message(msg.chat.id, "Какая-то хуйня случилась. Пиши админу, блять").await?;
                            }
                        }
                    }
                }
            }

            _ => {
                bot.send_message(msg.chat.id, "Неизвестная команда свиньи")
                    .await?;
            }
        }

        Ok(())
    }

    pub async fn format_pig_info(&self, pig: &crate::database::Pig, db: &Database) -> String {
        let position = match db.get_pig_rank(pig.chat_id, pig.user_id).await {
            Ok(rank) => rank,
            Err(e) => {
                log::error!("Database error: {}", e);
                None
            }
        }.unwrap_or(0);


        format!(
            "🐖 Ваш {} весит {} кг\n\
            📊 Место в топе: {}\n\
            ",
            pig.name, pig.weight, position,
        )
    }


    pub async fn feed_pig(&self, pig: &mut crate::database::Pig, db: &Database, config: &Config) -> Result<String, sqlx::Error> {
        let current_time = std::time::SystemTime::now()
            .duration_since(std::time::UNIX_EPOCH)
            .unwrap()
            .as_secs_f64();

        pig.last_feed = current_time;

        let total_players = db.get_chat_total_players(pig.chat_id).await?;
        let current_rank = db.get_pig_rank(pig.chat_id, pig.user_id).await?.unwrap_or(1);
        let score = pig.weight as f64;
        let (min_grow, max_grow) = self.calculate_grow_range(score, current_rank, total_players, &config.game);

        let growth = if min_grow == max_grow {
            min_grow
        } else {
            let range = (max_grow - min_grow + 1) as u32;
            let random_offset = rand::random::<u32>() % range;
            min_grow + random_offset as i32
        };

        pig.weight = (pig.weight + growth).max(1);

        db.update_pig(pig).await?;

        let growth_text = if growth > 0 {
            format!("поправился на {} кг", growth)
        } else if growth < 0 {
            format!("уменьшился на {} кг", -growth)
        } else {
            format!("обосрался и нихуя не прибавил")
        };
        Ok(format!("🐖 Ваш {} {} \n\
                    💪 Теперь он весит {} кг.\n
                    ",
                pig.name, growth_text, pig.weight))
    }
}
