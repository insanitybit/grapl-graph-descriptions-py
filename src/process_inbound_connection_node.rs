use grapl_graph_descriptions::graph_description::ProcessInboundConnection as InnerProcessInboundConnection;
use grapl_graph_descriptions::graph_description::ProcessInboundConnectionBuilder;

use pyo3::create_exception;
use pyo3::prelude::*;

create_exception!(process_inbound_connection_node, ProcessInboundConnectionBuilderError, pyo3::exceptions::ValueError);

#[pyclass]
#[derive(Clone)]
pub struct ProcessInboundConnectionNode {
    pub(crate) inner_node: InnerProcessInboundConnection,
}

impl<'source> pyo3::FromPyObject<'source> for ProcessInboundConnectionNode {
    fn extract(ob: &'source pyo3::types::PyAny) -> pyo3::PyResult<Self> {
        Ok(
            pyo3::PyTryFrom::try_from(ob).map(|x: &Self| x.clone())?
        )
    }
}

#[pyclass]
#[derive(Clone, Default)]
pub struct ProcessInboundConnectionNodeBuilder {
    builder: ProcessInboundConnectionBuilder,
}

#[pymethods]
impl ProcessInboundConnectionNodeBuilder {
    #[new]
    fn new(
        obj: &PyRawObject,
    ) {
        obj.init(
            Self::default()
        )
    }

    pub fn with_asset_id(&mut self, asset_id: String) -> Self {
        self.builder.asset_id(asset_id);
        self.clone()
    }

    pub fn with_ip_address(&mut self, src_ip_address: String) -> Self {
        self.builder.ip_address(src_ip_address);
        self.clone()
    }

    pub fn with_port(&mut self, src_port: u16) -> Self {
        self.builder.port(src_port);
        self.clone()
    }

    pub fn with_protocol(&mut self, protocol: String) -> Self {
        self.builder.protocol(protocol);
        self.clone()
    }

    pub fn with_state(&mut self, state: u32) -> Self {
        self.builder.state(state);
        self.clone()
    }

    pub fn with_created_timestamp(&mut self, created_timestamp: u64) -> Self {
        self.builder.created_timestamp(created_timestamp);
        self.clone()
    }

    pub fn with_terminated_timestamp(&mut self, terminated_timestamp: u64) -> Self {
        self.builder.terminated_timestamp(terminated_timestamp);
        self.clone()
    }

    pub fn with_last_seen_timestamp(&mut self, last_seen_timestamp: u64) -> Self {
        self.builder.last_seen_timestamp(last_seen_timestamp);
        self.clone()
    }

    pub fn build(&self) -> PyResult<ProcessInboundConnectionNode> {
        let built_node = match self.builder.build() {
            Ok(built_node) => built_node,
            Err(e) => {
                return Err(
                    PyErr::new::<ProcessInboundConnectionBuilderError, _>(format!("{}", e))
                );
            }
        };

        Ok(
            ProcessInboundConnectionNode {
                inner_node: built_node
            }
        )
    }
}
