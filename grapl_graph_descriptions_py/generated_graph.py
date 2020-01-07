import time

from typing import *

from .graph_description_pb2 import (
    GeneratedSubgraphs as _GeneratedSubgraphs,
    Graph as _Graph,
)
from .graph import Graph

class Generatedsubgraphs(object):
    def __init__(self, graphs: Iterable[Graph]):
        self._generated_subgraphs = _GeneratedSubgraphs(
            subgraphs=[g._graph for g in graphs]
        )