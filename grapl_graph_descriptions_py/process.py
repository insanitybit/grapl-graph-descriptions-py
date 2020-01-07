from typing import *

import enum

import uuid
from .graph_description_pb2 import Process


class ProcessState(enum.IntEnum):
    CREATED = 0
    TERMINATED = 1
    EXISTING = 2


class ProcessBuilder(object):
    def __init__(self):
        self.node_key = str(uuid.uuid4())
        self.asset_id = None  # type: Optional[str]
        self.hostname = None  # type: Optional[str]
        self.process_id = 0
        self.process_guid = ""
        self.created_timestamp = 0
        self.terminated_timestamp = 0
        self.last_seen_timestamp = 0
        self.process_name = ""
        self.process_command_line = ""
        self.operating_system = ""

    def with_asset_id(self, asset_id: str) -> 'ProcessBuilder':
        self.asset_id = asset_id
        return self

    def with_process_id(self, process_id: int) -> 'ProcessBuilder':
        self.process_id = process_id
        return self

    def with_process_guid(self, process_guid: int) -> 'ProcessBuilder':
        self.process_guid = process_guid
        return self

    def with_created_timestamp(self, created_timestamp: int) -> 'ProcessBuilder':
        self.created_timestamp = created_timestamp
        return self

    def with_terminated_timestamp(self, terminated_timestamp: int) -> 'ProcessBuilder':
        self.terminated_timestamp = terminated_timestamp
        return self

    def with_last_seen_timestamp(self, last_seen_timestamp: int) -> 'ProcessBuilder':
        self.last_seen_timestamp = last_seen_timestamp
        return self

    def with_process_name(self, process_name: str) -> 'ProcessBuilder':
        self.process_name = process_name
        return self

    def with_process_command_line(self, process_command_line: str) -> 'ProcessBuilder':
        self.process_command_line = process_command_line
        return self

    def with_operating_system(self, operating_system: str) -> 'ProcessBuilder':
        self.operating_system = operating_system
        return self

    def build(self) -> Process:
        return Process(
            node_key=self.node_key,
            asset_id=self.asset_id,
            hostname=self.hostname,
            process_id=self.process_id,
            process_guid=self.process_guid,
            created_timestamp=self.created_timestamp,
            terminated_timestamp=self.terminated_timestamp,
            last_seen_timestamp=self.last_seen_timestamp,
            process_name=self.process_name,
            process_command_line=self.process_command_line,
            operating_system=self.operating_system,
        )


