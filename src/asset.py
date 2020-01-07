import time

from typing import *

from .graph_description_pb2 import Graph as _Graph, Node, Edge, EdgeList
from .graph_description_pb2 import (
    DynamicNode,
    File,
    IpAddress,
    IpConnection,
    IpPort,
    NetworkConnection,
    Process,
    ProcessInboundConnection,
    ProcessOutboundConnection,
)

NodeT = Union[
    File,
    IpAddress,
    IpConnection,
    IpPort,
    NetworkConnection,
    Process,
    ProcessInboundConnection,
    ProcessOutboundConnection,
    DynamicNode,
]

class Graph(object):
    def __init__(self):
        self._graph = _Graph(
            nodes={},
            edges={},
            timestamp=int(time.time())
        )

    def add_node(self, node: NodeT):
        node_key = node.node_key
        node_wrapper = into_node(node)
        self._graph.nodes[node_key] = node_wrapper

    def add_edge(self, edge_name: str, from_key: str, to_key: str):
        edge = Edge(
            to=to_key,
            edgeName=edge_name,
        )
        # 'from' is a reserved keyword :[

        setattr(edge, 'from', from_key)
        edge_list = self._graph.edges.get(from_key)
        if edge_list:
            edge_list.edges.append(edge)
        else:
            edge_list = EdgeList(edges=[edge])
            self._graph.edges[from_key] = edge_list


def into_node(node: NodeT) -> Node:
    if isinstance(node, Node):
        return node

    if isinstance(node, File):
        node_wrapper = Node(
            file_node=node,
        )
    elif isinstance(node, IpAddress):
        node_wrapper = Node(
            ip_address_node=node,
        )
    elif isinstance(node, IpConnection):
        node_wrapper = Node(
            ip_connection_node=node,
        )
    elif isinstance(node, IpPort):
        node_wrapper = Node(
            ip_port_node=node,
        )
    elif isinstance(node, NetworkConnection):
        node_wrapper = Node(
            network_connection_node=node,
        )
    elif isinstance(node, Process):
        node_wrapper = Node(
            process_node=node,
        )
    elif isinstance(node, ProcessInboundConnection):
        node_wrapper = Node(
            process_inbound_connection_node=node,
        )
    elif isinstance(node, ProcessOutboundConnection):
        node_wrapper = Node(
            process_outbound_connection_node=node,
        )
    elif isinstance(node, DynamicNode):
        node_wrapper = Node(
            dynamic_node=node,
        )
    else:
        raise TypeError(f"{type(node)} is not a valid Node")

    return node_wrapper