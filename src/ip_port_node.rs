    use grapl_graph_descriptions::graph_description::IpPort as InnerIpPort;
use grapl_graph_descriptions::graph_description::IpPortBuilder;
use pyo3::create_exception;
use pyo3::prelude::*;

create_exception!(ip_port_node, IpPortBuilderError, pyo3::exceptions::ValueError);

#[pyclass]
#[derive(Clone)]
pub struct IpPortNode {
    pub(crate) inner_node: InnerIpPort,
}

impl<'source> pyo3::FromPyObject<'source> for IpPortNode {
    fn extract(ob: &'source pyo3::types::PyAny) -> pyo3::PyResult<Self> {
        Ok(
            pyo3::PyTryFrom::try_from(ob).map(|x: &Self| x.clone())?
        )
    }
}

#[pyclass]
#[derive(Clone, Default)]
pub struct IpPortNodeBuilder {
    builder: IpPortBuilder,
}

#[pymethods]
impl IpPortNodeBuilder {
    #[new]
    fn new(
        obj: &PyRawObject,
    ) {
        obj.init(
            Self::default()
        )
    }

    pub fn with_ip_address(&mut self, ip_address: String) -> Self {
        self.builder.ip_address(ip_address);
        self.clone()
    }

    pub fn with_port(&mut self, port: u16) -> Self {
        self.builder.port(port);
        self.clone()
    }

    pub fn with_protocol(&mut self, protocol: String) -> Self {
        self.builder.protocol(protocol);
        self.clone()
    }

    pub fn build(&self) -> PyResult<IpPortNode> {

        let built_node = match self.builder.build() {
            Ok(built_node) => built_node,
            Err(e) => {
                return Err(
                    PyErr::new::<IpPortBuilderError, _>(format!("{}", e))
                )
            }
        };

        Ok(
            IpPortNode {
                inner_node: built_node
            }
        )
    }
}
