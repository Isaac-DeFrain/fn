use serde::Serialize;
use std::collections::HashMap;

pub trait Output {
    fn network_create() -> NetworkCreate;
    fn network_deploy() -> NetworkDeploy;
    fn network_destroy() -> NetworkDestroy;
    fn network_status() -> NetworkStatus;
    fn node_started() -> NodeStarted;
    fn node_stopped() -> NodeStopped;
    fn archive_data_dump() -> ArchiveDataDump;
    fn mina_logs_dump() -> MinaLogsDump;
    fn precomputed_block_dump() -> PrecomputedBlockDump;
    fn replayer_run() -> ReplayerRun;
}

#[derive(Clone, Debug, Serialize)]
pub struct NetworkCreate {
    network_id: NetworkId,
}

#[derive(Clone, Debug, Serialize)]
pub struct NetworkDeploy {
    network_id: NetworkId,
    nodes: HashMap<NodeId, NodeInfo>,
}

#[derive(Clone, Debug, Serialize)]
pub struct NetworkDestroy {}

#[derive(Clone, Debug, Serialize)]
pub struct NodeInfo {
    node_id: NodeId,
    node_type: NodeType,
    network_id: NetworkId,
    network_keypair: Option<NetworkKeypair>,
    graphql_uri: Option<GraphqlUri>,
}

#[derive(Clone, Debug, Serialize)]
pub struct NetworkStatus {}

#[derive(Clone, Debug, Serialize)]
pub struct NodeStarted {
    node_id: NodeId,
    fresh_state: bool,
    git_commit: String,
}

#[derive(Clone, Debug, Serialize)]
pub struct NodeStopped {
    node_id: NodeId,
}

#[derive(Clone, Debug, Serialize)]
pub struct ArchiveDataDump {
    node_id: NodeId,
    data: String,
}

#[derive(Clone, Debug, Serialize)]
pub struct MinaLogsDump {
    node_id: NodeId,
    logs: String,
}

#[derive(Clone, Debug, Serialize)]
pub struct PrecomputedBlockDump {
    node_id: NodeId,
    blocks: String,
}

#[derive(Clone, Debug, Serialize)]
pub struct ReplayerRun {
    node_id: NodeId,
    logs: String,
}

#[allow(dead_code)]
#[derive(Clone, Debug, Serialize)]
pub enum NodeType {
    ArchiveNode,
    BlockProducer,
    SeedNode,
    SnarkCoordinator,
    SnarkWorker,
}

macro_rules! impl_struct {
    ($name:ident, $type:ident) => {
        #[derive(Clone, Debug, Serialize)]
        pub struct $name($type);

        impl std::fmt::Display for $name {
            fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                write!(f, "{}", self.0)?;
                Ok(())
            }
        }
    };
}

impl_struct!(GraphqlUri, String);
impl_struct!(NetworkId, String);
impl_struct!(NetworkKeypair, String);
impl_struct!(NodeId, String);
impl_struct!(NodeLogs, String);
