use grapl_graph_descriptions::graph_description::IpAddress as InnerIpAddress;
use grapl_graph_descriptions::graph_description::IpAddressBuilder;
use pyo3::create_exception;
use pyo3::prelude::*;

create_exception!(ip_address_node, IpAddressBuilderError, pyo3::exceptions::ValueError);

#[pyclass]
#[derive(Clone)]
pub struct IpAddressNode {
    pub(crate) inner_node: InnerIpAddress,
}

impl<'source> pyo3::FromPyObject<'source> for IpAddressNode {
    fn extract(ob: &'source pyo3::types::PyAny) -> pyo3::PyResult<Self> {
        Ok(
            pyo3::PyTryFrom::try_from(ob).map(|x: &Self| x.clone())?
        )
    }
}

#[pyclass]
#[derive(Clone, Default)]
pub struct IpAddressNodeBuilder {
    builder: IpAddressBuilder,
}

#[pymethods]
impl IpAddressNodeBuilder {
    #[new]
    fn new(
        obj: &PyRawObject,
    ) {
        obj.init(
            Self::default()
        )
    }

    pub fn with_ip_address(&mut self, ip_address_id: String) -> Self {
        self.builder.ip_address(ip_address_id);
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

    pub fn build(&self) -> PyResult<IpAddressNode> {

        let built_node = match self.builder.build() {
            Ok(built_node) => built_node,
            Err(e) => {
                return Err(
                    PyErr::new::<IpAddressBuilderError, _>(format!("{}", e))
                )
            }
        };

        Ok(
            IpAddressNode {
                inner_node: built_node
            }
        )
    }
}
