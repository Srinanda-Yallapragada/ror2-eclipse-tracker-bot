from enum import Enum


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
