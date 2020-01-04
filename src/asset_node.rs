use grapl_graph_descriptions::graph_description::Asset as InnerAsset;
use grapl_graph_descriptions::graph_description::AssetBuilder;
use pyo3::create_exception;
use pyo3::prelude::*;

create_exception!(asset_node, AssetBuilderError, pyo3::exceptions::ValueError);

#[pyclass]
#[derive(Clone)]
pub struct AssetNode {
    pub(crate) inner_node: InnerAsset,
}

impl<'source> pyo3::FromPyObject<'source> for AssetNode {
    fn extract(ob: &'source pyo3::types::PyAny) -> pyo3::PyResult<Self> {
        Ok(
            pyo3::PyTryFrom::try_from(ob).map(|x: &Self| x.clone())?
        )
    }
}

#[pyclass]
#[derive(Clone, Default)]
pub struct AssetNodeBuilder {
    builder: AssetBuilder,
}

#[pymethods]
impl AssetNodeBuilder {
    #[new]
    fn new(
        obj: &PyRawObject,
    ) {
        obj.init(
            Self::default()
        )
    }

    pub fn with_asset_id(&mut self, asset_id: Option<String>) -> Self {
        self.builder.asset_id(asset_id);
        self.clone()
    }

    pub fn with_hostname(&mut self, hostname: Option<String>) -> Self {
        self.builder.hostname(hostname);
        self.clone()
    }

    pub fn with_first_timestamp_ms(&mut self, timestamp: u64) -> Self {
        self.builder.first_seen_timestamp(timestamp);
        self.clone()
    }

    pub fn with_last_seen_timestamp_ms(&mut self, timestamp: u64) -> Self {
        self.builder.last_seen_timestamp(timestamp);
        self.clone()
    }

    pub fn build(&self) -> PyResult<AssetNode> {

        let built_node = match self.builder.build() {
            Ok(built_node) => built_node,
            Err(e) => {
                return Err(
                    PyErr::new::<AssetBuilderError, _>(format!("{}", e))
                )
            }
        };

        Ok(
            AssetNode {
                inner_node: built_node
            }
        )
    }
}
