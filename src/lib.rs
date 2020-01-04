use pyo3::prelude::*;


pub mod asset_node;
pub mod file_node;
pub mod graph;
pub mod process_node;
pub mod ip_address_node;
pub mod ip_connection_node;
pub mod ip_port_node;
pub mod network_connection_node;
pub mod process_outbound_connection_node;
pub mod process_inbound_connection_node;


#[pymodule]
fn grapl_graph_descriptions_py(_py: Python, m: &PyModule) -> PyResult<()> {
    m.add_class::<process_node::ProcessNode>()?;
    m.add_class::<process_node::ProcessNodeBuilder>()?;

    m.add_class::<file_node::FileNode>()?;
    m.add_class::<file_node::FileNodeBuilder>()?;

    m.add_class::<asset_node::AssetNode>()?;
    m.add_class::<asset_node::AssetNodeBuilder>()?;

    m.add_class::<ip_address_node::IpAddressNode>()?;
    m.add_class::<ip_address_node::IpAddressNodeBuilder>()?;

    m.add_class::<ip_connection_node::IpConnectionNode>()?;
    m.add_class::<ip_connection_node::IpConnectionNodeBuilder>()?;

    m.add_class::<ip_port_node::IpPortNode>()?;
    m.add_class::<ip_port_node::IpPortNodeBuilder>()?;

    m.add_class::<network_connection_node::NetworkConnectionNode>()?;
    m.add_class::<network_connection_node::NetworkConnectionNodeBuilder>()?;

    m.add_class::<process_outbound_connection_node::ProcessOutboundConnectionNode>()?;
    m.add_class::<process_outbound_connection_node::ProcessOutboundConnectionNodeBuilder>()?;

    m.add_class::<process_inbound_connection_node::ProcessInboundConnectionNode>()?;
    m.add_class::<process_inbound_connection_node::ProcessInboundConnectionNodeBuilder>()?;

    m.add_class::<graph::Graph>()?;

    Ok(())
}

#[cfg(test)]
mod tests {
    #[test]
    fn it_works() {
        assert_eq!(2 + 2, 4);
    }
}
