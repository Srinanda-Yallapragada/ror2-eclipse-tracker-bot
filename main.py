# requires the 'message_content' intent.
import json  # needed to read secrets for discord bot token
from enum import Enum

import discord
from discord.ext import commands

from pymongo import MongoClient

client = MongoClient("mongodb://localhost:27017/")

database = client.ror2_eclipse_tracker_db
collection = database.eclipse_levels


class Survivor(Enum):
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


with open("Secrets.json") as file:
    secrets = json.load(file)


intents = discord.Intents.default()
intents.message_content = True  # needed to see message content

bot = commands.Bot(command_prefix="!", intents=intents)


@bot.event
async def on_ready():
    print(f"Logged in as {bot.user}\n")


@bot.command()
async def whoami(ctx):
    await ctx.send("You are: " + str(ctx.author))


@bot.command()
async def insert_fake_data(ctx):
    collection.insert_one(
        {"name": "user1", "levels": [0, 0, 4, 2, 0, 0, 0, 0, 0, 0, 0, 0, 0]}
    )

    collection.insert_one(
        {"name": "user2", "levels": [0, 0, 6, 9, 0, 0, 0, 0, 0, 0, 0, 0, 0]}
    )


@bot.command()
async def get_data(ctx):
    await ctx.send(str(list(collection.find())))


bot.run(secrets["token"])
