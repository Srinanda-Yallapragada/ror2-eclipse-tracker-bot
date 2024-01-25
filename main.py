# requires the 'message_content' intent
import json  # needed to read secrets for discord bot token
from enum import Enum

import discord
from discord.ext import commands

from pymongo import MongoClient

client = MongoClient("mongodb://localhost:27017/")
# discord userid are now unique, meaning in the database,
# "userid" column will be a unique identifier
database = client.ror2_eclipse_tracker_db
col = database.eclipse_levels

NUM_SURVIVORS = 13


class Survivors(Enum):
    ACRID = 0
    ARTIFICER = 1
    BANDIT = 2
    CAPTAIN = 3
    COMMANDO = 4
    ENGINEER = 5
    HUNTRESS = 6
    LOADER = 7
    MUL_T = 8
    MERCENARY = 9
    REX = 10
    RAILGUNNER = 11
    VOID_FIEND = 12

    @classmethod
    def has_key(cls, name):
        return name in cls.__members__


intents = discord.Intents.default()
intents.message_content = True  # needed to see message content

bot = commands.Bot(command_prefix="!", intents=intents)


@bot.event
async def on_ready():
    print(f"Logged in as {bot.user}\n")


@bot.command()
async def whoami(ctx):
    await ctx.send("You are: " + str(ctx.author))


# Expected one time usecase for uploading all current levels in one command
@bot.command()
async def insert_all_levels(ctx):
    res = col.find_one({"userid": str(ctx.author)})
    if res:
        await ctx.send(
            "You already used this command once! Use !update_survivor instead"
        )
        return

    user_levels = [1] * NUM_SURVIVORS

    def check(m):
        return int(m.content) > 0 and int(m.content) < 10

    for survivor in Survivors:
        await ctx.send("Input " + survivor.name + " eclipse level.")
        msg = await bot.wait_for("message", check=check, timeout=30)
        user_levels[survivor.value] = int(msg.content)

    col.insert_one({"userid": str(ctx.author), "levels": user_levels})
    await ctx.send("Levels for user " + str(ctx.author) + " had been added")


@bot.command()
async def get_my_levels(ctx):
    res = col.find_one({"userid": str(ctx.author)})
    if res:
        user_levels = res["levels"]
        # formmating return data
        ret_string = "Eclipse levels for " + str(ctx.author) + "\n"
        for survivor in Survivors:
            ret_string += survivor.name
            ret_string += " - "
            ret_string += (
                "completed"
                if (user_levels[survivor.value]) == 9
                else str(user_levels[survivor.value])
            )
            ret_string += "\n"

        await ctx.send(ret_string)
    else:
        await ctx.send("You do not have an entry in the database")


async def valid_input_for_update(ctx, survivor, level):
    # convert to uppercase for Survivor Enum
    survivor = survivor.upper()
    # Check if survivor exsists
    if not Survivors.has_key(survivor):
        await ctx.send("This survivor does not exsist. \n")
        return False

    if not level.isdigit():
        await ctx.send(str(level) + " is an invalid value for level. \n")
        return False

    elif not (int(level) < 10) and (int(level) > 0):
        await ctx.send(str(level) + " must be between 1 and 9 inclusive. \n")
        return False

    return True


@bot.command()
async def update_survivor(ctx, survivor, level):
    if await valid_input_for_update(ctx, survivor, level):
        survivor = survivor.upper()
        level = int(level)
    else:
        await ctx.send("Invalid command. Please try again!")
        return

    res = col.find_one({"userid": str(ctx.author)})

    if res:
        user_levels = res["levels"]
        user_levels[Survivors[survivor].value] = level

        # update in database
        col.update_one({"userid": str(ctx.author)}, {"$set": {"levels": user_levels}})

        await ctx.send(
            "Updated "
            + str(ctx.author)
            + "'s survivor "
            + survivor
            + " to level "
            + str(level)
        )

    else:
        print("First use !insert_all_data to initilize your eclipse levels")


# gets discord bot token from Secrets.json
with open("Secrets.json") as file:
    secrets = json.load(file)

bot.run(secrets["token"])
