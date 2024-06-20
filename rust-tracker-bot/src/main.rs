use std::usize;

use anyhow::Context as _;
use poise::serenity_prelude::{ClientBuilder, GatewayIntents};
use shuttle_runtime::SecretStore;
use shuttle_serenity::ShuttleSerenity;
use sqlx::{Executor, PgPool};

mod db;
mod survivor_enum;

struct Data {
    database: PgPool,
} // User data, which is stored and accessible in all command invocations
type Error = Box<dyn std::error::Error + Send + Sync>;
type Context<'a> = poise::Context<'a, Data, Error>;

/// Responds with "world!"
#[poise::command(slash_command)]
async fn hello(ctx: Context<'_>) -> Result<(), Error> {
    ctx.say("world!").await?;
    Ok(())
}

/// Echoes the message you type
#[poise::command(slash_command)]
async fn echo(ctx: Context<'_>, message: String) -> Result<(), Error> {
    ctx.say(message).await?;
    Ok(())
}

/// Responds with your name
#[poise::command(slash_command)]
async fn whoami(ctx: Context<'_>) -> Result<(), Error> {
    let response = format!(
        "{} Your user_name is {}",
        ctx.author().to_string(),
        ctx.author().name
    );
    ctx.say(response).await?;
    Ok(())
}

/// Gets all survivor eclipse levels
#[poise::command(slash_command)]
async fn get_survivor_lvls(ctx: Context<'_>) -> Result<(), Error> {
    let levels = db::fetch_all_lvls(&ctx.data().database, &ctx.author().name).await?;

    // Format the levels into a readable string
    let mut response = String::new();
    for (index, level) in levels.iter().enumerate() {
        if let Some(survivor) = survivor_enum::Survivors::index_to_survivor(index) {
            response.push_str(&format!(
                "{}: [{}]\n",
                survivor.survivor_to_name(),
                if *level == 9 {
                    "Completed".to_string()
                } else {
                    level.to_string()
                }
            ));
        }
    }
    ctx.say(response).await?;
    Ok(())
}

/// Update one survivor's eclipse level
#[poise::command(slash_command)]
async fn set_survivor_eclipse_lvl(
    ctx: Context<'_>,
    #[description = "Which survivor do you want to update?"]
    //Poise only supports choice types that can be constructed from a literal
    // (https://doc.rust-lang.org/reference/expressions/literal-expr.html).
    #[choices(
        "Acrid",
        "Artificer",
        "Bandit",
        "Captain",
        "Commando",
        "Engineer",
        "Huntress",
        "Loader",
        "MulT",
        "Mercenary",
        "Rex",
        "Railgunner",
        "VoidFiend"
    )]
    selection: &'static str,
    #[description = "What level to update to?"] new_lvl: i32,
) -> Result<(), Error> {
    let survivor = match survivor_enum::Survivors::name_to_survivor(selection) {
        Some(selection) => selection,
        None => {
            ctx.say("Invalid survivor name provided.").await?;
            return Ok(());
        }
    };

    let response = format!(
        "Updated survivor {} to eclipse level [{}]",
        survivor, new_lvl
    );

    let user_name = ctx.author().name.clone();
    let exists = db::check_user_name_exists(&ctx.data().database, &user_name).await?;
    if exists == true {
        let mut levels = db::fetch_all_lvls(&ctx.data().database, &ctx.author().name).await?;
        levels[survivor as usize] = new_lvl;
        db::update(&ctx.data().database, &user_name, &levels).await?;
    } else {
        let mut levels: [i32; survivor_enum::NUM_SURVIVORS] = [0; survivor_enum::NUM_SURVIVORS];
        levels[survivor as usize] = new_lvl;
        db::add(&ctx.data().database, &user_name, &levels).await?;
    }
    ctx.say(response).await?;
    Ok(())
}

#[shuttle_runtime::main]
async fn main(
    #[shuttle_shared_db::Postgres] pool: PgPool,
    #[shuttle_runtime::Secrets] secret_store: SecretStore,
) -> ShuttleSerenity {
    // Get the discord token set in `Secrets.toml`
    let discord_token = secret_store
        .get("DISCORD_TOKEN")
        .context("'DISCORD_TOKEN' was not found")?;

    // Run the schema migration
    pool.execute(include_str!("../schema.sql"))
        .await
        .context("failed to run migrations")?;

    let framework = poise::Framework::builder()
        .options(poise::FrameworkOptions {
            commands: vec![
                hello(),
                whoami(),
                echo(),
                get_survivor_lvls(),
                set_survivor_eclipse_lvl(),
            ],
            ..Default::default()
        })
        .setup(|ctx, _ready, framework| {
            Box::pin(async move {
                poise::builtins::register_globally(ctx, &framework.options().commands).await?;
                Ok(Data { database: pool })
            })
        })
        .build();
    let client = ClientBuilder::new(discord_token, GatewayIntents::non_privileged())
        .framework(framework)
        .await
        .map_err(shuttle_runtime::CustomError::new)?;

    Ok(client.into())
}
