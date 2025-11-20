use std::usize;

use crate::db;
use crate::survivor_enum;
use crate::survivor_enum::NUM_SURVIVORS;
use crate::Data;

type Error = Box<dyn std::error::Error + Send + Sync>;
type Context<'a> = poise::Context<'a, Data, Error>;

/// Gets all survivor eclipse levels. The levels are the number inside the [] bracket.
#[poise::command(slash_command)]
pub async fn get_survivor_lvls(ctx: Context<'_>) -> Result<(), Error> {
    let user_name = ctx.author().name.clone();

    let exists = db::check_user_name_exists(&ctx.data().database, &user_name).await?;

    if !exists {
        ctx.say("You are not in the database. Run set_survivor_eclipse_lvl first and update your survivor levels")
            .await?;
        return Ok(());
    }

    let levels = db::fetch_all_lvls(&ctx.data().database, &ctx.author().name).await?;

    // Format the levels into a readable string
    let mut response = String::new();
    for (index, level) in levels.iter().enumerate() {
        let survivor = survivor_enum::Survivors::index_to_survivor(index);
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
    ctx.say(response).await?;
    Ok(())
}

/// Update one survivor's eclipse level.  The levels are the number inside the [] bracket.
#[poise::command(slash_command)]
pub async fn set_survivor_eclipse_lvl(
    ctx: Context<'_>,
    #[description = "Which survivor do you want to update?"]
    //Poise only supports choice types that can be constructed from a literal
    // (https://doc.rust-lang.org/reference/expressions/literal-expr.html).

    //on new survivor update here
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
        "VoidFiend",
        "Seeker",
        "FalseSon",
        "Chef",
        "Operator",
        "Drifter"
    )]
    selection: &'static str,
    #[description = "What level to update to? Set level to 9 if you have completed all levels."]
    new_lvl: i32,
) -> Result<(), Error> {
    if new_lvl < 1 || new_lvl > 9 {
        ctx.say("Invalid level provided. Valid levels are in the range 1 <= level <= 9")
            .await?;
        return Ok(());
    }

    let survivor = match survivor_enum::Survivors::name_to_survivor(selection) {
        Some(selection) => selection,
        None => {
            ctx.say("Invalid survivor name provided.").await?;
            return Ok(());
        }
    };

    let response = format!(
        "Updated survivor {} to eclipse level [{}].",
        survivor, new_lvl
    );

    let user_name = ctx.author().name.clone();
    let exists = db::check_user_name_exists(&ctx.data().database, &user_name).await?;

    if exists == true {
        let mut levels = db::fetch_all_lvls(&ctx.data().database, &ctx.author().name).await?;
        levels[survivor as usize] = new_lvl;
        db::update(&ctx.data().database, &user_name, &levels).await?;
    } else {
        let mut levels: [i32; survivor_enum::NUM_SURVIVORS] = [1; survivor_enum::NUM_SURVIVORS];
        levels[survivor as usize] = new_lvl;
        db::add(&ctx.data().database, &user_name, &levels).await?;
    }
    ctx.say(response).await?;
    Ok(())
}
/// Returns eclipse class combinations for selected players
#[poise::command(slash_command)]
pub async fn eclipse_class_selector(
    ctx: Context<'_>,
    #[description = "Username of user 1."]
    #[choices("obama5542", "nanx4", "romir.k", "ehnanda", "gamerunicorn.", "prnvs", "thatprofessor", "None")]
    user0: &str,
    #[description = "Username of user 2."]
    #[choices("obama5542", "nanx4", "romir.k", "ehnanda", "gamerunicorn.", "prnvs", "thatprofessor", "None")]
    user1: &str,
    #[description = "Username of user 3."]
    #[choices("obama5542", "nanx4", "romir.k", "ehnanda", "gamerunicorn.", "prnvs", "thatprofessor", "None")]
    user2: &str,
    #[description = "Username of user 4."]
    #[choices("obama5542", "nanx4", "romir.k", "ehnanda", "gamerunicorn.", "prnvs", "thatprofessor", "None")]
    user3: &str,
) -> Result<(), Error> {
    let users = vec![user0, user1, user2, user3]
        .into_iter()
        .filter(|user| *user != "None")
        .map(|user| user.to_owned())
        .collect::<Vec<String>>();

    let mut lvls_matrix = vec![[0; NUM_SURVIVORS]; 4];

    for (index, user) in users.iter().enumerate() {
        if !db::check_user_name_exists(&ctx.data().database, user).await? {
            ctx.say(format!(
                "User {} does not exist in the database.  Run set_survivor_eclipse_lvl first",
                user
            ))
            .await?;
            return Ok(());
        } else {
            let user_lvls = db::fetch_all_lvls(&ctx.data().database, user).await?;
            lvls_matrix[index] = user_lvls;
        }
    }

    let response = build_response(&users, &lvls_matrix);
    ctx.say(response).await?;
    Ok(())
}

///  Increment survivor's eclipse level by one.
#[poise::command(slash_command)]
pub async fn survivor_lvl_up(
    ctx: Context<'_>,
    #[description = "Which survivor do you want to level up by 1?"]
    //Poise only supports choice types that can be constructed from a literal
    // (https://doc.rust-lang.org/reference/expressions/literal-expr.html).

    //on new survivor update here
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
        "VoidFiend",
        "Seeker",
        "FalseSon",
        "Chef",
        "Operator",
        "Drifter"
    )]
    selection: &'static str,
) -> Result<(), Error> {
    let user_name = ctx.author().name.clone();
    let exists = db::check_user_name_exists(&ctx.data().database, &user_name).await?;

    if exists == true {
        let mut levels = db::fetch_all_lvls(&ctx.data().database, &ctx.author().name).await?;

        let survivor = match survivor_enum::Survivors::name_to_survivor(selection) {
            Some(selection) => selection,
            None => {
                ctx.say("Invalid survivor name provided.").await?;
                return Ok(());
            }
        };

        let survivor_index = survivor_enum::Survivors::survivor_to_index(&survivor);

        if levels[survivor_index] < 9 {
            levels[survivor_index] += 1;
            ctx.say(format!(
                "{} level updated to {}",
                survivor, levels[survivor_index]
            ))
            .await?;
        } else if levels[survivor_index] == 9 {
            ctx.say(format!(
                "{} has already completed all eclipse levels.",
                survivor
            ))
            .await?;
            return Ok(());
        }

        db::update(&ctx.data().database, &user_name, &levels).await?;
    } else {
        ctx.say(format!(
            "User {} does not exist in the database.  Run set_survivor_eclipse_lvl first",
            user_name
        ))
        .await?;
        return Ok(());
    }
    Ok(())
}

fn build_response(users: &[String], lvls_matrix: &[[i32; NUM_SURVIVORS]]) -> String {
    let mut response = String::new();
    for i in 1..=9 {
        let mut level_has_survivors = false;

        for (user_index, _user) in users.iter().enumerate() {
            if lvls_matrix[user_index].iter().any(|&lvl| lvl == i) {
                level_has_survivors = true;
                break;
            }
        }

        if !level_has_survivors {
            continue;
        }

        if i == 9 {
            response.push_str("Eclipse completed\n");
        } else {
            response.push_str(&format!("Eclipse {}:\n", i));
        }

        for (user_index, user) in users.iter().enumerate() {
            let mut user_has_survivors = false;
            let mut user_response = String::new();

            for (index, lvl) in lvls_matrix[user_index].iter().enumerate() {
                if *lvl == i {
                    user_response.push_str(&format!(
                        "{}, ",
                        survivor_enum::Survivors::index_to_survivor(index)
                    ));
                    user_has_survivors = true;
                }
            }

            if user_has_survivors {
                response.push_str(&format!("    {}: {}", user, user_response));
                response.push_str("\n");
            }
        }
        response.push_str("\n");
    }
    return response;
}

// // Responds with "world!"
// #[poise::command(slash_command)]
// pub async fn hello(ctx: Context<'_>) -> Result<(), Error> {
//     ctx.say("world!").await?;
//     Ok(())
// }

// /// Echoes the message you type
// #[poise::command(slash_command)]
// pub async fn echo(ctx: Context<'_>, message: String) -> Result<(), Error> {
//     ctx.say(message).await?;
//     Ok(())
// }

// /// Responds with your name
// #[poise::command(slash_command)]
// pub async fn whoami(ctx: Context<'_>) -> Result<(), Error> {
//     let response = format!(
//         "{} Your user_name is {}",
//         ctx.author().to_string(),
//         ctx.author().name
//     );
//     ctx.say(response).await?;
//     Ok(())
// }
