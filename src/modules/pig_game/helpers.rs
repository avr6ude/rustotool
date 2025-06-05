use crate::database::{Database, Pig};
use crate::config::GameConfig;
use rand::prelude::*;

impl super::PigGameModule {
    pub fn calculate_grow_range(&self, score: f64, rank: i32, total_players: i32, config: &GameConfig) -> (i32, i32) {
        let loss_base = config.base_growth;
        let gain_base = config.weight_factor;
        let loss_coefficient = config.rank_factor;
        let gain_coefficient = 1.0;

        let rank_percentage = rank as f64 / total_players as f64;

        let loss_modifier = 1.0 - rank_percentage;
        let gain_modifier = 1.0 / (1.0 - rank_percentage + 1.0);

        let max_loss = ((loss_base * score) * (loss_coefficient * loss_modifier)) + 15.0;
        let max_gain = (gain_base * score) * (2.0 * gain_coefficient * gain_modifier) + 35.0;

        (-max_loss.floor() as i32, max_gain.floor() as i32)
    }

    pub fn generate_default_pig_name(&self) -> String {
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
        let random_index = rng.random_range(0..names.len());
        format!("{}", names[random_index])
    }

    pub async fn create_new_pig(
        &self,
        chat_id: i64,
        user_id: i64,
        owner_name: &str,
        pig_name: &str,
        db: &Database,
    ) -> Result<Pig, sqlx::Error> {
        let new_pig = Pig {
            id: 0,
            chat_id,
            user_id,
            weight: 0,
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
            last_weight: 0,
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
}
