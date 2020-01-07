import enum
import uuid

from .graph_description_pb2 import NetworkConnection


class NetworkConnectionState(enum.IntEnum):
    CREATED = 0
    TERMINATED = 1
    EXISTING = 2


class NetworkConnectionBuilder(object):
    def __init__(self):
        self.node_key = str(uuid.uuid4())
        self.src_ip_address = ""
        self.dst_ip_address = ""
        self.protocol = ""
        self.src_port = 0
        self.dst_port = 0
        self.created_timestamp = 0
        self.terminated_timestamp = 0
        self.last_seen_timestamp = 0
        self.state = None

    def with_src_ip_address(self, src_ip_address: str) -> 'NetworkConnectionBuilder':
        self.src_ip_address = src_ip_address
        return self

    def with_src_port(self, src_port: int) -> 'NetworkConnectionBuilder':
        self.src_port = src_port
        return self

    def with_dst_ip_address(self, dst_ip_address: str) -> 'NetworkConnectionBuilder':
        self.dst_ip_address = dst_ip_address
        return self

    def with_dst_port(self, dst_port: int) -> 'NetworkConnectionBuilder':
        self.dst_port = dst_port
        return self

    def with_protocol(self, protocol: str) -> 'NetworkConnectionBuilder':
        self.protocol = protocol
        return self

    def build(self) -> NetworkConnection:
        assert self.protocol
        assert self.src_ip_address
        assert self.src_port
        assert self.dst_ip_address
        assert self.dst_port
        assert self.state

        return NetworkConnection(
            node_key=self.node_key,
            src_ip_address=self.src_ip_address,
            dst_ip_address=self.dst_ip_address,
            protocol=self.protocol,
            src_port=self.src_port,
            dst_port=self.dst_port,
            created_timestamp=self.created_timestamp,
            terminated_timestamp=self.terminated_timestamp,
            last_seen_timestamp=self.last_seen_timestamp,
            state=self.state,

        )


