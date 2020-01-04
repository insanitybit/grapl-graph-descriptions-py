use grapl_graph_descriptions::graph_description::Process as InnerProcess;
use grapl_graph_descriptions::graph_description::ProcessBuilder;

use pyo3::create_exception;
use pyo3::prelude::*;

create_exception!(process_node, ProcessBuilderError, pyo3::exceptions::ValueError);

#[pyclass]
#[derive(Clone)]
pub struct ProcessNode {
    pub(crate) inner_node: InnerProcess,
}

impl<'source> pyo3::FromPyObject<'source> for ProcessNode {
    fn extract(ob: &'source pyo3::types::PyAny) -> pyo3::PyResult<Self> {
        Ok(
            pyo3::PyTryFrom::try_from(ob).map(|x: &Self| x.clone())?
        )
    }
}

#[pyclass]
#[derive(Clone, Default)]
pub struct ProcessNodeBuilder {
    builder: ProcessBuilder,
}

#[pymethods]
impl ProcessNodeBuilder {
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

    pub fn with_state(&mut self, state: u32) -> Self {
        self.builder.state(state);

        self.clone()
    }

    pub fn with_process_id(&mut self, process_id: u64) -> Self {
        self.builder.process_id(process_id);
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

    pub fn with_terminated_timestamp_ms(&mut self, timestamp: u64) -> Self {
        self.builder.terminated_timestamp(timestamp);
        self.clone()
    }

    pub fn with_process_name(&mut self, process_name: Option<String>) -> Self {
        self.builder.process_name(process_name.unwrap_or_default());
        self.clone()
    }

    pub fn with_operating_system(&mut self, operating_system: Option<String>) -> Self {
        self.builder.operating_system(operating_system.unwrap_or_default());
        self.clone()
    }

    pub fn with_process_command_line(&mut self, process_command_line: Option<String>) -> Self {
        self.builder.process_command_line(process_command_line.unwrap_or_default());
        self.clone()
    }

    pub fn with_process_guid(&mut self, process_guid: Option<String>) -> Self {
        self.builder.process_guid(process_guid.unwrap_or_default());
        self.clone()
    }

    pub fn build(&self) -> PyResult<ProcessNode> {

        let built_node = match self.builder.build() {
            Ok(built_node) => built_node,
            Err(e) => {
                return Err(
                    PyErr::new::<ProcessBuilderError, _>(format!("{}", e))
                )
            }
        };

        Ok(
            ProcessNode {
                inner_node: built_node
            }
        )
    }
}
