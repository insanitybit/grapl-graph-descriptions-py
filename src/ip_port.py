import uuid

from .graph_description_pb2 import IpPort


class IpPortBuilder(object):
    def __init__(self):
        self.node_key = str(uuid.uuid4())
        self.ip_address = ""
        self.port = 0
        self.protocol = ""

    def with_ip_address(self, ip_address: str) -> 'IpPortBuilder':
        self.ip_address = ip_address
        return self

    def with_port(self, port: int) -> 'IpPortBuilder':
        self.port = port
        return self

    def with_protocol(self, protocol: str) -> 'IpPortBuilder':
        self.protocol = protocol
        return self

    def build(self) -> IpPort:
        assert self.ip_address
        assert self.protocol
        assert self.port

        return IpPort(
            node_key=self.node_key,
            ip_address=self.ip_address,
            port=self.port,
            protocol=self.ip_address,

        )


