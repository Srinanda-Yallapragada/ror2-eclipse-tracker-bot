use anyhow::Context as _;
use poise::serenity_prelude::{ClientBuilder, GatewayIntents};
use shuttle_runtime::SecretStore;
use shuttle_serenity::ShuttleSerenity;
use sqlx::{Executor, PgPool};

mod commands;
mod db;
mod survivor_enum;

pub struct Data {
    pub database: PgPool,
} // User data, which is stored and accessible in all command invocations
  // type Error = Box<dyn std::error::Error + Send + Sync>;
  // type Context<'a> = poise::Context<'a, Data, Error>;

// /// Responds with "world!"
// #[poise::command(slash_command)]
// async fn hello(ctx: Context<'_>) -> Result<(), Error> {
//     ctx.say("world!").await?;
//     Ok(())
// }

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
                commands::get_survivor_lvls(),
                commands::set_survivor_eclipse_lvl(),
                commands::eclipse_class_selector(),
                commands::survivor_lvl_up(),
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
