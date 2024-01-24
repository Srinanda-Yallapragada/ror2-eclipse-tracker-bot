# requires the 'message_content' intent.
import json  # needed to read secrets for discord bot token
from enum import Enum

import discord
from discord.ext import commands

from pymongo import MongoClient

client = MongoClient("mongodb://localhost:27017/")

# discord userid are now unique, meaning in the database, "userid" column will be a unique identifier
database = client.ror2_eclipse_tracker_db
collection = database.eclipse_levels


class Survivors(Enum):
    ACRID = 0
    ARTIFICER = 1
    BANDIT = 2
    CAPTAIN = 3
    COMMANDO = 4
    ENGINEER = 5
    HERETIC = 6
    HUNTRESS = 7
    LOADER = 8
    MUL_T = 9
    MERCENARY = 10
    REX = 11
    RAILGUNNER = 12
    VOID_FIEND = 13

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
# this is to be updated to be more userfriendly, but this will do for now
@bot.command()
async def insert_all_levels(ctx, levels):
    res = collection.find_one({"userid": str(ctx.author)})
    if res:
        await ctx.send("You already have uploaded you eclipse levels!")
        return

    levels = [int(i) for i in levels]  # conevrting string to int
    collection.insert_one({"userid": str(ctx.author), "levels": levels})
    await ctx.send("Levels for user" + str(ctx.author) + " had been added")


# this is to be updated to be more user friendly, but this will do for now
@bot.command()
async def get_my_levels(ctx):
    res = collection.find_one({"userid": str(ctx.author)})
    if res:
        user_levels = res["levels"]
        print(user_levels)
        # formmating return data
        ret_string = "Eclipse levels for " + str(ctx.author) + "\n"
        for survivor in Survivors:
            ret_string += survivor.name
            ret_string += " - "
            ret_string += str(user_levels[survivor.value])
            ret_string += "\n"

        await ctx.send(ret_string)
    else:
        await ctx.send("You do not have an entry in the database")


async def valid_for_update(ctx, survivor, level):
    # convert to uppercase for Survivor Enum
    survivor = survivor.upper()
    # Check if survivor exsists
    if not Survivors.has_key(survivor):
        await ctx.send("This survivor does not exsist")
        return False

    if not level.isdigit():
        await ctx.send(str(level) + " is an invalid value for level")
        return False
    return True


@bot.command()
async def update_survivor(ctx, survivor, level):
    if valid_for_update(ctx, survivor, level):
        survivor = survivor.upper()
        level = int(level)
    else:
        ctx.send("Invalid command syntax. Please try again!")
        return

    res = collection.find_one({"userid": str(ctx.author)})

    if res:
        # isolate current levels from result
        user_levels = res["levels"]
        # update the particular survivors eclipse level
        user_levels[Survivors[survivor].value] = level
        # update in database
        collection.update_one(
            {"userid": str(ctx.author)}, {"$set": {"levels": user_levels}}
        )

        await ctx.send(
            "Updated "
            + str(ctx.author)
            + "'s survivor "
            + survivor
            + " to level "
            + str(level)
        )

    else:
        print("First user !insert_all_data to initilize your eclipse levels")


# gets discord bot token from Secrets.json
with open("Secrets.json") as file:
    secrets = json.load(file)

bot.run(secrets["token"])
