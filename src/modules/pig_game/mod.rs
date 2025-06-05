use crate::database::{Database, Pig};
use crate::modules::BotModule;
use async_trait::async_trait;
use rand::prelude::*;
use teloxide::{prelude::*, types::Message};

pub struct PigGameModule;

impl PigGameModule {
    pub fn new() -> Self {
        Self
    }

    fn generate_default_pig_name(&self) -> String {
        let names = vec![
            "Хрякоблядь",
            "Свинопидор",
            "Ебаный Кабан",
            "Бекон ебучий",
            "Хрюкало Сраное",
            "Матьегохряк",
            "Пиздохрюк",
            "Свинья в говне",
            "Блядобекон",
            "Хрякотрах",
        ];

        let mut rng = rand::rng();
        let random_name = names[rng.random_range(0..names.len())];
        format!("{}", random_name)
    }

    async fn create_new_pig(
        &self,
        chat_id: i64,
        user_id: i64,
        owner_name: &str,
        pig_name: &str,
        db: &Database,
    ) -> Result<Pig, sqlx::Error> {
        let new_pig = Pig {
            id: 0, // Will be set by database
            chat_id,
            user_id,
            weight: 10, // Starting weight
            name: pig_name.to_string(),
            last_feed: 0.0,
            last_salo: 0.0,
            owner_name: owner_name.to_string(),
            salo: 0,
            poisoned: false,
            barn: 0,
            pigsty: 0,
            vetclinic: 0,
            vet_last_pickup: 0.0,
            last_weight: 10,
            avatar_url: None,
            biolab: 0,
            butchery: 0,
            pills: 0,
            factory: 0,
            warehouse: 0,
            institute: 0,
        };

        db.create_pig(&new_pig).await
    }

    async fn feed_pig(&self, pig: &mut Pig, db: &Database) -> Result<String, sqlx::Error> {
        let current_time = std::time::SystemTime::now()
            .duration_since(std::time::UNIX_EPOCH)
            .unwrap()
            .as_secs_f64();

        pig.last_feed = current_time;
        pig.weight += 5; // Simple feeding logic

        db.update_pig(pig).await?;
        Ok(format!("🐷 {} съел корм! Вес: {}", pig.name, pig.weight))
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
            ("feed", "Покормить свинью"),
            ("mypig", "Посмотреть информацию о своей свинье"),
            ("pigstats", "Посмотреть статистику свиней"),
        ]
    }

    async fn handle_command(
        &self,
        bot: Bot,
        msg: Message,
        command: &str,
        args: Vec<&str>,
        db: &Database,
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

            "feed" => match db.get_pig(chat_id, user_id).await {
                Ok(Some(mut pig)) => match self.feed_pig(&mut pig, db).await {
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
            },

            "mypig" => match db.get_pig(chat_id, user_id).await {
                Ok(Some(pig)) => {
                    let status = if pig.poisoned {
                        "🤢 Отравлена"
                    } else {
                        "😊 Здорова"
                    };
                    let message = format!(
                        "🐷 Ваша свинья: {}\n\
                             💪 Вес: {}\n\
                             🏠 Сарай: {}\n\
                             🐖 Свинарник: {}\n\
                             🏥 Ветклиника: {}\n\
                             🧪 Таблетки: {}\n\
                             📊 Статус: {}",
                        pig.name,
                        pig.weight,
                        pig.barn,
                        pig.pigsty,
                        pig.vetclinic,
                        pig.pills,
                        status
                    );
                    bot.send_message(msg.chat.id, message).await?;
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
            },

            "pigstats" => {
                // Find pig by name if args provided, otherwise show user's pig
                if !args.is_empty() {
                    let search_name = args.join(" ");
                    match db.find_pig_by_name(chat_id, &search_name).await {
                        Ok(pigs) if !pigs.is_empty() => {
                            let pig = &pigs[0]; // Take first match
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
                    // Show user's own pig
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

            _ => {
                bot.send_message(msg.chat.id, "Неизвестная команда свиньи")
                    .await?;
            }
        }

        Ok(())
    }
}
