import uuid

from .graph_description_pb2 import IpAddress


class IpAddressBuilder(object):
    def __init__(self):
        self.ip_address = ""

    def with_ip_address(self, ip_address: str) -> 'IpAddressBuilder':
        self.ip_address = ip_address
        return self


    def build(self) -> IpAddress:
        assert self.ip_address
        return IpAddress(
            node_key=self.ip_address,
            ip_address=self.ip_address
        )


