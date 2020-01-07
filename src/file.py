from typing import *

import enum

import uuid
from .graph_description_pb2 import File


class FileState(enum.IntEnum):
    CREATED = 0
    DELETED = 1
    EXISTING = 2


class FileBuilder(object):
    def __init__(self):
        self.node_key = str(uuid.uuid4())
        self.asset_id = None  # type: Optional[str]
        self.hostname = None  # type: Optional[str]
        self.state = None
        self.created_timestamp = 0  # type: int
        self.deleted_timestamp = 0  # type: int
        self.last_seen_timestamp = 0  # type: int
        self.file_name = ""  # type: str
        self.file_path = ""  # type: str
        self.file_extension = ""  # type: str
        self.file_mime_type = ""  # type: str
        self.file_size = 0  # type: int
        self.file_version = ""  # type: str
        self.file_description = ""  # type: str
        self.file_product = ""  # type: str
        self.file_company = ""  # type: str
        self.file_directory = ""  # type: str
        self.file_inode = 0  # type: int
        self.file_hard_links = 0  # type: int
        self.md5_hash = ""  # type: str
        self.sha1_hash = ""  # type: str
        self.sha256_hash = ""  # type: str

    def with_asset_id(self, asset_id: str):
        self.asset_id = asset_id
        return self

    def with_hostname(self, hostname: str):
        self.hostname = hostname
        return self

    def with_state(self, state: FileState):
        self.state = state
        return self

    def with_created_timestamp(self, created_timestamp: int):
        self.created_timestamp = created_timestamp
        return self

    def with_deleted_timestamp(self, deleted_timestamp: int):
        self.deleted_timestamp = deleted_timestamp
        return self

    def with_last_seen_timestamp(self, last_seen_timestamp: int):
        self.last_seen_timestamp = last_seen_timestamp
        return self

    def with_file_name(self, file_name: str):
        self.file_name = file_name
        return self

    def with_file_path(self, file_path: str):
        self.file_path = file_path
        return self

    def with_file_extension(self, file_extension: str):
        self.file_extension = file_extension
        return self

    def with_file_mime_type(self, file_mime_type: str):
        self.file_mime_type = file_mime_type
        return self

    def with_file_size(self, file_size: int):
        self.file_size = file_size
        return self

    def with_file_version(self, file_version: str):
        self.file_version = file_version
        return self

    def with_file_description(self, file_description: str):
        self.file_description = file_description
        return self

    def with_file_product(self, file_product: str):
        self.file_product = file_product
        return self

    def with_file_company(self, file_company: str):
        self.file_company = file_company
        return self

    def with_file_directory(self, file_directory: str):
        self.file_directory = file_directory
        return self

    def with_file_inode(self, file_inode: str):
        self.file_inode = file_inode
        return self

    def with_file_hard_links(self, file_hard_links: int):
        self.file_hard_links = file_hard_links
        return self

    def with_md5_hash(self, md5_hash: str):
        self.md5_hash = md5_hash
        return self

    def with_sha1_hash(self, sha1_hash: str):
        self.sha1_hash = sha1_hash
        return self

    def with_sha256_hash(self, sha256_hash: str):
        self.sha256_hash = sha256_hash
        return self

    def build(self) -> File:
        return File(
            node_key=self.node_key,
            asset_id=self.asset_id,
            hostname=self.hostname,
            state=self.state,
            created_timestamp=self.created_timestamp,
            deleted_timestamp=self.deleted_timestamp,
            last_seen_timestamp=self.last_seen_timestamp,
            file_name=self.file_name,
            file_path=self.file_path,
            file_extension=self.file_extension,
            file_mime_type=self.file_mime_type,
            file_size=self.file_size,
            file_version=self.file_version,
            file_description=self.file_description,
            file_product=self.file_product,
            file_company=self.file_company,
            file_directory=self.file_directory,
            file_inode=self.file_inode,
            file_hard_links=self.file_hard_links,
            md5_hash=self.md5_hash,
            sha1_hash=self.sha1_hash,
            sha256_hash=self.sha256_hash,
        )
