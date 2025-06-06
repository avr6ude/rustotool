use teloxide::types::{InlineKeyboardButton, InlineKeyboardMarkup, MessageId};

impl super::PigGameModule {
    pub fn create_pig_keyboard(&self, user_id: i64, command_message_id: i32) -> InlineKeyboardMarkup {
        InlineKeyboardMarkup::new(vec![
            vec![
                InlineKeyboardButton::callback("🐷 ГРОВИМ!", &format!("grow:{}", user_id)),
            ],
            vec![
                InlineKeyboardButton::callback("🗑 Удалить", &format!("remove:{}", command_message_id)),
            ],
        ])

    }

    pub fn create_grow_keyboard(&self, user_id: i64) -> InlineKeyboardMarkup {
        InlineKeyboardMarkup::new(vec![
            vec![
                InlineKeyboardButton::callback("🔙 Назад", &format!("back:{}", user_id)),
            ]
        ])
    }
}
