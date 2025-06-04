use teloxide::{prelude::*, utils::command::BotCommands};

mod config;
mod database;
use config::Config;
use database::Database;


#[tokio::main]
async fn main() {
    pretty_env_logger::init();
    log::info!("Starting bot...");

    let config = Config::load_or_default();
    log::info!("Config loaded: {:?}", config);

    // Connect to database
    let database_url = config.database_url
        .or_else(|| std::env::var("DATABASE_URL").ok())
        .expect("DATABASE_URL not set in config or environment");
    
    let db = Database::connect(&database_url).await
        .expect("Failed to connect to database");
    
    // Run migrations
    log::info!("Running database migrations...");
    db.migrate().await.expect("Failed to run migrations");
    log::info!("Database migrations completed");

    let token = std::env::var("TELOXIDE_TOKEN").expect("TELOXIDE_TOKEN not set");
    let bot = Bot::new(token);

    Command::repl(bot, answer).await;
}


#[derive(BotCommands, Clone)]
#[command(rename_rule = "lowercase", description = "These commands are supported:")]
enum Command {
    #[command(description = "Display this message")]
    Help,
    #[command(description = "Display the current time")]
    Username(String),
    #[command(description = "Display the current date", parse_with = "split")]
    UsernameAndAge {username: String, age: u8},
}

async fn answer(bot: Bot, msg: Message, cmd: Command) -> ResponseResult<()> {
    match cmd {
        Command::Help => {
            bot.send_message(msg.chat.id, Command::descriptions().to_string()).await?;
        }
        Command::Username(username) => {
            bot.send_message(msg.chat.id, format!("Your username is @{username}.")).await?;
        }
        Command::UsernameAndAge { username, age} => {
            bot.send_message(msg.chat.id, format!("Your username is @{username} and age is {age}.")).await?;
        }
    };

    Ok(())
}
