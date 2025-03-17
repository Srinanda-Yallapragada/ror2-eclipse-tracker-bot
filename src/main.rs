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
}

#[shuttle_runtime::main]
async fn main(
    #[shuttle_shared_db::Postgres] pool: PgPool,
    #[shuttle_runtime::Secrets] secret_store: SecretStore,
) -> ShuttleSerenity {
    // Get the discord token set in `Secrets.toml`
    // you need to reset the token everytime you lose it from the 
    // developer portal
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
