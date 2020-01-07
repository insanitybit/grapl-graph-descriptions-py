import uuid

from typing import *

from .graph_description_pb2 import DynamicNode, NodeProperty


class DynamicNodeBuilder(object):
    def __init__(self, node_type: str):
        self.node_type = node_type
        self.node_key = str(uuid.uuid4())

        self.asset_id = None
        self.hostname = None

        self.seen_at = 0
        self.id_strategy = None

        self.str_props = {}  # type: Dict[str, str]
        self.int_props = {}  # type: Dict[str, int]
        self.uint_props = {}  # type: Dict[str, int]

    def with_str_prop(self, name: str, value: str):
        self.str_props[name] = value

    def with_int_prop(self, name: str, value: int):
        self.int_props[name] = value

    def with_uint_prop(self, name: str, value: int):
        assert value >= 0
        self.uint_props[name] = value

    def build(self) -> DynamicNode:
        properties = {}

        for prop_name, prop in self.str_props:
            properties[prop_name] = NodeProperty(
                strprop=prop
            )

        for prop_name, prop in self.int_props:
            properties[prop_name] = NodeProperty(
                intprop=prop
            )

        for prop_name, prop in self.uint_props:
            properties[prop_name] = NodeProperty(
                uintprop=prop
            )

        return DynamicNode(
            node_key=self.node_key,
            properties=properties,
            node_type=self.node_type,
            seen_at=self.seen_at,
            asset_id=self.asset_id,
            hostname=self.hostname,
            host_ip=None,  # deprecated
            id_strategy=self.id_strategy,
        )