import enum
import uuid

from .graph_description_pb2 import ProcessInboundConnection


class ProcessInboundConnectionState(enum.IntEnum):
    CREATED = 0
    TERMINATED = 1
    EXISTING = 2


class ProcessInboundConnectionBuilder(object):
    def __init__(self):
        self.node_key = str(uuid.uuid4())
        self.state = None
        self.asset_id = None
        self.hostname = None
        self.created_timestamp = 0
        self.terminated_timestamp = 0
        self.last_seen_timestamp = 0
        self.port = 0
        self.ip_address = ""
        self.protocol = ""

    def build(self) -> ProcessInboundConnection:
        assert self.state
        assert self.asset_id
        assert self.hostname
        assert self.created_timestamp
        assert self.terminated_timestamp
        assert self.last_seen_timestamp
        assert self.port
        assert self.ip_address
        assert self.protocol

        return ProcessInboundConnection(
            node_key=self.node_key,
            state=self.state,
            asset_id=self.asset_id,
            hostname=self.hostname,
            created_timestamp=self.created_timestamp,
            terminated_timestamp=self.terminated_timestamp,
            last_seen_timestamp=self.last_seen_timestamp,
            port=self.port,
            ip_address=self.ip_address,
            protocol=self.protocol,
        )


