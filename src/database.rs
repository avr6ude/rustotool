use serde::{Deserialize, Serialize};
use sqlx::PgPool;
use uuid::Uuid;

#[derive(Debug, Clone, Serialize, Deserialize, sqlx::FromRow)]
pub struct Pig {
    pub id: i32,
    pub chat_id: i64,
    pub user_id: i64,
    pub weight: i32,
    pub name: String,
    pub last_feed: f64,
    pub last_salo: f64,
    pub owner_name: String,
    pub salo: i32,
    pub poisoned: bool,
    pub barn: i32,
    pub pigsty: i32,
    pub vetclinic: i32,
    pub vet_last_pickup: f64,
    pub last_weight: i32,
    pub avatar_url: Option<String>,
    pub biolab: i32,
    pub butchery: i32,
    pub pills: i32,
    pub factory: i32,
    pub warehouse: i32,
    pub institute: i32,
}

#[derive(Debug, Clone, Serialize, Deserialize, sqlx::FromRow)]
pub struct Loot {
    pub id: i32,
    pub chat_id: i64,
    pub owner: i64,
    pub name: String,
    pub icon: String,
    pub description: Option<String>,
    pub class_name: String,
    pub class_icon: String,
    pub weight: f64,
    pub base_stats: serde_json::Value,
    pub rarity: serde_json::Value,
    pub uuid: Uuid,
}

pub struct Database {
    pool: PgPool,
}

impl Database {
    pub async fn connect(database_url: &str) -> Result<Self, sqlx::Error> {
        let pool = PgPool::connect(database_url).await?;
        Ok(Database { pool })
    }

    pub async fn migrate(&self) -> Result<(), sqlx::migrate::MigrateError> {
        sqlx::migrate!("./migrations").run(&self.pool).await
    }
    pub async fn get_chat_pigs_ranked(&self, chat_id: i64) -> Result<Vec<Pig>, sqlx::Error> {
        sqlx::query_as::<_, Pig>(
            "SELECT id, chat_id, user_id, weight, name, last_feed, last_salo, owner_name,
             salo, poisoned, barn, pigsty, vetclinic, vet_last_pickup, last_weight,
             avatar_url, biolab, butchery, pills, factory, warehouse, institute
             FROM pigs WHERE chat_id = $1 ORDER BY weight DESC"
        )
        .bind(chat_id)
        .fetch_all(&self.pool)
        .await
    }

    pub async fn get_chat_total_players(&self, chat_id: i64) -> Result<i32, sqlx::Error> {
        let count: (i64,) = sqlx::query_as(
            "SELECT COUNT(*) FROM pigs WHERE chat_id = $1",
        )
        .bind(chat_id)
        .fetch_one(&self.pool)
        .await?;

        Ok(count.0 as i32)
    }

    pub async fn get_pig_rank(&self, chat_id: i64, user_id: i64) -> Result<Option<i32>, sqlx::Error> {
        let result: Option<(i64,)> = sqlx::query_as(
            "SELECT rank FROM (
            SELECT user_id, ROW_NUMBER() OVER (ORDER BY weight DESC) as rank
            FROM pigs WHERE chat_id = $1
            ) ranked WHERE user_id = $2"
        )
        .bind(chat_id)
        .bind(user_id)
        .fetch_optional(&self.pool)
        .await?;

        Ok(result.map(|r| r.0 as i32))
    }
    pub async fn get_pig(&self, chat_id: i64, user_id: i64) -> Result<Option<Pig>, sqlx::Error> {
        sqlx::query_as::<_, Pig>(
            "SELECT id, chat_id, user_id, weight, name, last_feed, last_salo, owner_name,
             salo, poisoned, barn, pigsty, vetclinic, vet_last_pickup, last_weight,
             avatar_url, biolab, butchery, pills, factory, warehouse, institute
             FROM pigs WHERE chat_id = $1 AND user_id = $2",
        )
        .bind(chat_id)
        .bind(user_id)
        .fetch_optional(&self.pool)
        .await
    }

    pub async fn create_pig(&self, pig: &Pig) -> Result<Pig, sqlx::Error> {
        sqlx::query_as::<_, Pig>(
            "INSERT INTO pigs (chat_id, user_id, weight, name, last_feed, last_salo, owner_name,
                              salo, poisoned, barn, pigsty, vetclinic, vet_last_pickup, last_weight,
                              avatar_url, biolab, butchery, pills, factory, warehouse, institute)
             VALUES ($1, $2, $3, $4, $5, $6, $7, $8, $9, $10, $11, $12, $13, $14, $15, $16, $17, $18, $19, $20, $21)
             RETURNING id, chat_id, user_id, weight, name, last_feed, last_salo, owner_name,
                       salo, poisoned, barn, pigsty, vetclinic, vet_last_pickup, last_weight,
                       avatar_url, biolab, butchery, pills, factory, warehouse, institute"
        )
        .bind(pig.chat_id)
        .bind(pig.user_id)
        .bind(pig.weight)
        .bind(&pig.name)
        .bind(pig.last_feed)
        .bind(pig.last_salo)
        .bind(&pig.owner_name)
        .bind(pig.salo)
        .bind(pig.poisoned)
        .bind(pig.barn)
        .bind(pig.pigsty)
        .bind(pig.vetclinic)
        .bind(pig.vet_last_pickup)
        .bind(pig.last_weight)
        .bind(&pig.avatar_url)
        .bind(pig.biolab)
        .bind(pig.butchery)
        .bind(pig.pills)
        .bind(pig.factory)
        .bind(pig.warehouse)
        .bind(pig.institute)
        .fetch_one(&self.pool)
        .await
    }

    pub async fn update_pig(&self, pig: &Pig) -> Result<Pig, sqlx::Error> {
        sqlx::query_as::<_, Pig>(
            "UPDATE pigs SET weight = $3, name = $4, last_feed = $5, last_salo = $6, owner_name = $7,
                            salo = $8, poisoned = $9, barn = $10, pigsty = $11, vetclinic = $12,
                            vet_last_pickup = $13, last_weight = $14, avatar_url = $15, biolab = $16,
                            butchery = $17, pills = $18, factory = $19, warehouse = $20, institute = $21
             WHERE chat_id = $1 AND user_id = $2
             RETURNING id, chat_id, user_id, weight, name, last_feed, last_salo, owner_name,
                       salo, poisoned, barn, pigsty, vetclinic, vet_last_pickup, last_weight,
                       avatar_url, biolab, butchery, pills, factory, warehouse, institute"
        )
        .bind(pig.chat_id)
        .bind(pig.user_id)
        .bind(pig.weight)
        .bind(&pig.name)
        .bind(pig.last_feed)
        .bind(pig.last_salo)
        .bind(&pig.owner_name)
        .bind(pig.salo)
        .bind(pig.poisoned)
        .bind(pig.barn)
        .bind(pig.pigsty)
        .bind(pig.vetclinic)
        .bind(pig.vet_last_pickup)
        .bind(pig.last_weight)
        .bind(&pig.avatar_url)
        .bind(pig.biolab)
        .bind(pig.butchery)
        .bind(pig.pills)
        .bind(pig.factory)
        .bind(pig.warehouse)
        .bind(pig.institute)
        .fetch_one(&self.pool)
        .await
    }

    // Loot operations
    pub async fn get_user_loot(
        &self,
        chat_id: i64,
        user_id: i64,
    ) -> Result<Vec<Loot>, sqlx::Error> {
        sqlx::query_as::<_, Loot>(
            "SELECT id, chat_id, owner, name, icon, description, class_name, class_icon,
                    weight, base_stats, rarity, uuid
             FROM loot WHERE chat_id = $1 AND owner = $2",
        )
        .bind(chat_id)
        .bind(user_id)
        .fetch_all(&self.pool)
        .await
    }

    pub async fn add_loot(&self, loot: &Loot) -> Result<Loot, sqlx::Error> {
        sqlx::query_as::<_, Loot>(
            "INSERT INTO loot (chat_id, owner, name, icon, description, class_name, class_icon,
                              weight, base_stats, rarity, uuid)
             VALUES ($1, $2, $3, $4, $5, $6, $7, $8, $9, $10, $11)
             RETURNING id, chat_id, owner, name, icon, description, class_name, class_icon,
                       weight, base_stats, rarity, uuid",
        )
        .bind(loot.chat_id)
        .bind(loot.owner)
        .bind(&loot.name)
        .bind(&loot.icon)
        .bind(&loot.description)
        .bind(&loot.class_name)
        .bind(&loot.class_icon)
        .bind(loot.weight)
        .bind(&loot.base_stats)
        .bind(&loot.rarity)
        .bind(loot.uuid)
        .fetch_one(&self.pool)
        .await
    }

    pub async fn find_pig_by_name(
        &self,
        chat_id: i64,
        name: &str,
    ) -> Result<Vec<Pig>, sqlx::Error> {
        let search_pattern = format!("%{}%", name);
        sqlx::query_as::<_, Pig>(
            "SELECT id, chat_id, user_id, weight, name, last_feed, last_salo, owner_name,
                    salo, poisoned, barn, pigsty, vetclinic, vet_last_pickup, last_weight,
                    avatar_url, biolab, butchery, pills, factory, warehouse, institute
             FROM pigs WHERE chat_id = $1 AND name ILIKE $2",
        )
        .bind(chat_id)
        .bind(search_pattern)
        .fetch_all(&self.pool)
        .await
    }
}
