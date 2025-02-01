use serde::{Deserialize, Serialize};

// ========================== SUCCESS RESPONSE TYPES ========================== //

// 200 OK - Node Information
#[derive(Serialize, Deserialize, Debug)]
pub struct NodeInfo {
    pub build_info: BuildInfo,
    pub upnp: bool,
    pub external_address: InetSocketAddress,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct BuildInfo {
    pub release_version: String,
    pub commit: String,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct InetSocketAddress {
    pub addr: String,
    pub port: u16,
}

// 200 OK - Node Version
#[derive(Serialize, Deserialize, Debug)]
pub struct NodeVersion {
    pub version: String,
}

// 200 OK - Chain Params
#[derive(Serialize, Deserialize, Debug)]
pub struct ChainParams {
    pub network_id: i32,
    pub num_zeros_at_least_in_hash: i32,
    pub group_num_per_broker: i32,
    pub groups: i32,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct SelfClique {
    pub clique_id: String,      // Clique ID
    pub nodes: Vec<CliqueNode>, // List of nodes in the clique
    pub self_ready: bool,       // Indicates if the node is self-ready
    pub synced: bool,           // Indicates if the node is fully synced
}

// Individual node in a clique
#[derive(Serialize, Deserialize, Debug)]
pub struct CliqueNode {
    pub address: String,     // Node's address (inet-address)
    pub rest_port: u16,      // REST API port
    pub ws_port: u16,        // WebSocket port
    pub miner_api_port: u16, // Miner API port
}

#[derive(Serialize, Deserialize, Debug)]
pub struct CliqueInfo {
    pub clique_id: String,          // Clique ID
    pub broker_id: i32,             // Broker ID
    pub group_num_per_broker: i32,  // Number of groups per broker
    pub address: InetSocketAddress, // Address information
    pub is_synced: bool,            // Synchronization status
    pub client_version: String,     // Client version
}

#[derive(Serialize, Deserialize, Debug)]
pub struct DiscoveredNeighbors {
    pub clique_id: String,          // Clique ID
    pub broker_id: i32,             // Broker ID
    pub broker_num: i32,            // Broker number
    pub address: InetSocketAddress, // Address information
}

// Represents a list of discovered peers and their statuses
#[derive(Serialize, Deserialize, Debug)]
pub struct Misbehaviors {
    pub peers: Vec<PeerInfo>, // List of peer information
}

// Individual peer entry
#[derive(Serialize, Deserialize, Debug)]
pub struct PeerInfo {
    pub peer: String, // Peer address (inet-address)
    pub status: PeerStatus, // Status of the peer
}

// Status of a peer (Enum for handling multiple cases)
#[derive(Serialize, Deserialize, Debug)]
#[serde(tag = "type")] // Discriminator to distinguish between different statuses
pub enum PeerStatus {
    Banned(BannedStatus),
    Penalty(PenaltyStatus),
}

// Banned peer details
#[derive(Serialize, Deserialize, Debug)]
pub struct BannedStatus {
    pub reason: String, // Reason for banning
    pub banned_until: String, // Timestamp or duration of ban
}

// Penalized peer details
#[derive(Serialize, Deserialize, Debug)]
pub struct PenaltyStatus {
    pub penalty_points: i32, // Number of penalty points assigned
    pub reason: String, // Reason for penalty
}
