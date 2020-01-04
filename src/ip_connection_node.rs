use grapl_graph_descriptions::graph_description::IpConnection as InnerIpConnection;
use grapl_graph_descriptions::graph_description::IpConnectionBuilder;

use pyo3::create_exception;
use pyo3::prelude::*;

create_exception!(ip_connection_node, IpConnectionBuilderError, pyo3::exceptions::ValueError);

#[pyclass]
#[derive(Clone)]
pub struct IpConnectionNode {
    inner_node: InnerIpConnection,
}

impl<'source> pyo3::FromPyObject<'source> for IpConnectionNode {
    fn extract(ob: &'source pyo3::types::PyAny) -> pyo3::PyResult<Self> {
        Ok(
            pyo3::PyTryFrom::try_from(ob).map(|x: &Self| x.clone())?
        )
    }
}


#[pyclass]
#[derive(Clone, Default)]
pub struct IpConnectionNodeBuilder {
    builder: IpConnectionBuilder,
}

#[pymethods]
impl IpConnectionNodeBuilder {
    #[new]
    fn new(
        obj: &PyRawObject,
    ) {
        obj.init(
            Self::default()
        )
    }


    pub fn with_src_ip_address(&mut self, src_ip_address: String) -> Self {
        self.builder.src_ip_address(src_ip_address);
        self.clone()
    }

    pub fn with_dst_ip_address(&mut self, dst_ip_address: String) -> Self {
        self.builder.dst_ip_address(dst_ip_address);
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


    pub fn build(&self) -> PyResult<IpConnectionNode> {

        let built_node = match self.builder.build() {
            Ok(built_node) => built_node,
            Err(e) => {
                return Err(
                    PyErr::new::<IpConnectionBuilderError, _>(format!("{}", e))
                )
            }
        };

        Ok(
            IpConnectionNode {
                inner_node: built_node
            }
        )
    }
}
