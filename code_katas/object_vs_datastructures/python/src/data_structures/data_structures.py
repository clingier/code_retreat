""" Contains the original set of data structures for the code. """

from dataclasses import dataclass
from enum import Flag
from typing import List


class State(Flag):
    ALIVE = True
    DEAD = False


@dataclass
class Coordinates:
    x: int
    y: int


@dataclass
class Grid:
    cells: List[List[State]]
