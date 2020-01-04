
use grapl_graph_descriptions::graph_description::File as InnerFile;
use grapl_graph_descriptions::graph_description::FileBuilder;
use pyo3::create_exception;
use pyo3::prelude::*;

create_exception!(file_node, FileBuilderError, pyo3::exceptions::ValueError);

#[pyclass]
#[derive(Clone)]
pub struct FileNode {
    pub(crate) inner_node: InnerFile,
}

impl<'source> pyo3::FromPyObject<'source> for FileNode {
    fn extract(ob: &'source pyo3::types::PyAny) -> pyo3::PyResult<Self> {
        Ok(
            pyo3::PyTryFrom::try_from(ob).map(|x: &Self| x.clone())?
        )
    }
}

#[pyclass]
#[derive(Clone, Default)]
pub struct FileNodeBuilder {
    builder: FileBuilder,
}

#[pymethods]
impl FileNodeBuilder {
    #[new]
    fn new(
        obj: &PyRawObject,
    ) {
        obj.init(
            Self::default()
        )
    }

    pub fn asset_id(&mut self, asset_id: Option<String>) -> Self {
        self.builder.asset_id(asset_id);
        self.clone()
    }

    pub fn hostname(&mut self, hostname: Option<String>) -> Self {
        self.builder.hostname(hostname);
        self.clone()
    }

    pub fn state(&mut self, state: u32) -> Self {
        self.builder.state(state);
        self.clone()
    }

    pub fn with_created_timestamp_ms(&mut self, timestamp: u64) -> Self {
        self.builder.created_timestamp(timestamp);
        self.clone()
    }

    pub fn with_last_seen_timestamp_ms(&mut self, timestamp: u64) -> Self {
        self.builder.last_seen_timestamp(timestamp);
        self.clone()
    }

    pub fn with_deleted_timestamp_ms(&mut self, timestamp: u64) -> Self {
        self.builder.deleted_timestamp(timestamp);
        self.clone()
    }

    pub fn file_name(&mut self, file_name: String) -> Self {
        self.builder.file_name(file_name);
        self.clone()
    }

    pub fn file_path(&mut self, file_path: String) -> Self {
        self.builder.file_path(file_path);
        self.clone()
    }

    pub fn file_extension(&mut self, file_extension: String) -> Self {
        self.builder.file_extension(file_extension);
        self.clone()
    }

    pub fn file_mime_type(&mut self, file_mime_type: String) -> Self {
        self.builder.file_mime_type(file_mime_type);
        self.clone()
    }

    pub fn file_size(&mut self, file_size: u64) -> Self {
        self.builder.file_size(file_size);
        self.clone()
    }

    pub fn file_version(&mut self, file_version: String) -> Self {
        self.builder.file_version(file_version);
        self.clone()
    }

    pub fn file_description(&mut self, file_description: String) -> Self {
        self.builder.file_description(file_description);
        self.clone()
    }

    pub fn file_product(&mut self, file_product: String) -> Self {
        self.builder.file_product(file_product);
        self.clone()
    }

    pub fn file_company(&mut self, file_company: String) -> Self {
        self.builder.file_company(file_company);
        self.clone()
    }

    pub fn file_directory(&mut self, file_directory: String) -> Self {
        self.builder.file_directory(file_directory);
        self.clone()
    }

    pub fn file_inode(&mut self, file_inode: u64) -> Self {
        self.builder.file_inode(file_inode);
        self.clone()
    }

    pub fn file_hard_links(&mut self, file_hard_links: u64) -> Self {
        self.builder.file_hard_links(file_hard_links);
        self.clone()
    }

    pub fn md5_hash(&mut self, md5_hash: String) -> Self {
        self.builder.md5_hash(md5_hash);
        self.clone()
    }

    pub fn sha1_hash(&mut self, sha1_hash: String) -> Self {
        self.builder.sha1_hash(sha1_hash);
        self.clone()
    }

    pub fn sha256_hash(&mut self, sha256_hash: String) -> Self {
        self.builder.sha256_hash(sha256_hash);
        self.clone()
    }



    pub fn build(&self) -> PyResult<FileNode> {

        let built_node = match self.builder.build() {
            Ok(built_node) => built_node,
            Err(e) => {
                return Err(
                    PyErr::new::<FileBuilderError, _>(format!("{}", e))
                )
            }
        };

        Ok(
            FileNode {
                inner_node: built_node
            }
        )
    }
}
