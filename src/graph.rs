use grapl_graph_descriptions::graph_description::{Graph as InnerGraph, Node};

use pyo3::prelude::*;
use crate::process_node::ProcessNode;
use crate::file_node::FileNode;
use crate::asset_node::AssetNode;
use crate::network_connection_node::NetworkConnectionNode;
use crate::process_inbound_connection_node::ProcessInboundConnectionNode;
use crate::ip_port_node::IpPortNode;
use crate::ip_address_node::IpAddressNode;

#[pyclass]
#[derive(Clone)]
pub struct Graph {
    pub inner_graph: InnerGraph,
}

impl<'source> pyo3::FromPyObject<'source> for Graph {
    fn extract(ob: &'source pyo3::types::PyAny) -> pyo3::PyResult<Self> {
        Ok(
            pyo3::PyTryFrom::try_from(ob).map(|x: &Self| x.clone())?
        )
    }
}


#[pymethods]
impl Graph {
    #[new]
    fn new(
        obj: &PyRawObject,
    ) {
        obj.init(
            Self {
                inner_graph: InnerGraph::new(0)
            }
        )
    }

    pub fn is_empty(&self) -> bool {
        self.inner_graph.is_empty()
    }

    pub fn merge(&mut self, other: &Self) {
        self.inner_graph.merge(&other.inner_graph);
    }

    pub fn add_process(&mut self, node: ProcessNode) {
        self.inner_graph.add_node(node.inner_node);
    }

    pub fn add_file(&mut self, node: FileNode) {
        self.inner_graph.add_node(node.inner_node);
    }

    pub fn add_asset(&mut self, node: AssetNode) {
        self.inner_graph.add_node(node.inner_node);
    }

    pub fn add_network_connection(&mut self, node: NetworkConnectionNode) {
        self.inner_graph.add_node(node.inner_node);
    }

    pub fn add_process_inbound_connection(&mut self, node: ProcessInboundConnectionNode) {
        self.inner_graph.add_node(node.inner_node);
    }

    pub fn add_process_outbound_connection(&mut self, node: ProcessInboundConnectionNode) {
        self.inner_graph.add_node(node.inner_node);
    }

    pub fn add_ip_port(&mut self, node: IpPortNode) {
        self.inner_graph.add_node(node.inner_node);
    }

    pub fn add_ip_address(&mut self, node: IpAddressNode) {
        self.inner_graph.add_node(node.inner_node);
    }

    pub fn add_edge(
        &mut self,
        edge_name: String,
        from: String,
        to: String,
    ) {
        self.inner_graph.add_edge(
            edge_name,
            from,
            to
        )
    }
}