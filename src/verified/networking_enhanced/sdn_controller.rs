//! Software-Defined Networking Controller for VANTIS OS
//!
//! Implements an SDN controller with topology management, BFS shortest-path
//! computation, and flow rule installation. Designed for kernel-level
//! network programmability with verified resource bounds.

use core::fmt;

// ============================================================================
// Network Topology Types
// ============================================================================

/// Unique identifier for a network node (switch/router)
pub type NodeId = u64;

/// Unique identifier for a network port
pub type PortId = u32;

/// A network node in the SDN topology
#[derive(Debug, Clone)]
pub struct NetworkNode {
    pub id: NodeId,
    pub name: String,
    pub node_type: NodeType,
    pub ports: Vec<PortId>,
    pub is_active: bool,
    pub flow_table_capacity: usize,
    pub flow_table_used: usize,
}

/// Types of network nodes
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum NodeType {
    /// OpenFlow-compatible switch
    Switch,
    /// Layer 3 router
    Router,
    /// Virtual switch (e.g., OVS)
    VirtualSwitch,
    /// Edge gateway
    Gateway,
}

impl NetworkNode {
    pub fn new(id: NodeId, name: &str, node_type: NodeType, num_ports: u32) -> Self {
        Self {
            id,
            name: name.to_string(),
            node_type,
            ports: (1..=num_ports).collect(),
            is_active: true,
            flow_table_capacity: 4096,
            flow_table_used: 0,
        }
    }

    /// Remaining flow table capacity
    pub fn flow_table_remaining(&self) -> usize {
        self.flow_table_capacity.saturating_sub(self.flow_table_used)
    }

    /// Flow table utilization percentage
    pub fn flow_table_utilization(&self) -> f64 {
        if self.flow_table_capacity == 0 {
            return 0.0;
        }
        self.flow_table_used as f64 / self.flow_table_capacity as f64 * 100.0
    }
}

/// A link between two network nodes
#[derive(Debug, Clone)]
pub struct Link {
    pub src_node: NodeId,
    pub src_port: PortId,
    pub dst_node: NodeId,
    pub dst_port: PortId,
    pub bandwidth_mbps: u64,
    pub latency_us: u32,
    pub is_active: bool,
    pub utilization_pct: f64,
}

impl Link {
    pub fn new(
        src_node: NodeId, src_port: PortId,
        dst_node: NodeId, dst_port: PortId,
        bandwidth_mbps: u64, latency_us: u32,
    ) -> Self {
        Self {
            src_node, src_port, dst_node, dst_port,
            bandwidth_mbps, latency_us,
            is_active: true,
            utilization_pct: 0.0,
        }
    }

    /// Available bandwidth in Mbps
    pub fn available_bandwidth(&self) -> f64 {
        self.bandwidth_mbps as f64 * (1.0 - self.utilization_pct / 100.0)
    }
}

// ============================================================================
// Flow Rules
// ============================================================================

/// Match criteria for a flow rule
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct FlowMatch {
    pub src_ip: Option<u32>,
    pub dst_ip: Option<u32>,
    pub src_port: Option<u16>,
    pub dst_port: Option<u16>,
    pub protocol: Option<u8>,
    pub vlan_id: Option<u16>,
    pub in_port: Option<PortId>,
}

impl FlowMatch {
    pub fn new() -> Self {
        Self {
            src_ip: None, dst_ip: None,
            src_port: None, dst_port: None,
            protocol: None, vlan_id: None,
            in_port: None,
        }
    }

    /// Check if this match is a subset of another (more specific)
    pub fn matches(&self, other: &FlowMatch) -> bool {
        fn check<T: PartialEq>(a: &Option<T>, b: &Option<T>) -> bool {
            match (a, b) {
                (Some(av), Some(bv)) => av == bv,
                (None, _) => true,
                (Some(_), None) => false,
            }
        }
        check(&self.src_ip, &other.src_ip)
            && check(&self.dst_ip, &other.dst_ip)
            && check(&self.src_port, &other.src_port)
            && check(&self.dst_port, &other.dst_port)
            && check(&self.protocol, &other.protocol)
    }

    /// Number of specified fields (for priority calculation)
    pub fn specificity(&self) -> u32 {
        let mut count = 0;
        if self.src_ip.is_some() { count += 1; }
        if self.dst_ip.is_some() { count += 1; }
        if self.src_port.is_some() { count += 1; }
        if self.dst_port.is_some() { count += 1; }
        if self.protocol.is_some() { count += 1; }
        if self.vlan_id.is_some() { count += 1; }
        if self.in_port.is_some() { count += 1; }
        count
    }
}

impl Default for FlowMatch {
    fn default() -> Self {
        Self::new()
    }
}

/// Action to take when a flow matches
#[derive(Debug, Clone, PartialEq, Eq)]
pub enum FlowAction {
    /// Forward to a specific port
    Forward(PortId),
    /// Drop the packet
    Drop,
    /// Flood to all ports
    Flood,
    /// Forward to the controller
    ToController,
    /// Rewrite destination and forward
    Rewrite { new_dst_ip: u32, out_port: PortId },
    /// Set VLAN tag and forward
    SetVlan { vlan_id: u16, out_port: PortId },
}

/// A complete flow rule
#[derive(Debug, Clone)]
pub struct FlowRule {
    pub id: u64,
    pub node_id: NodeId,
    pub priority: u32,
    pub match_criteria: FlowMatch,
    pub action: FlowAction,
    pub packet_count: u64,
    pub byte_count: u64,
    pub idle_timeout_secs: u32,
    pub hard_timeout_secs: u32,
}

// ============================================================================
// Error Types
// ============================================================================

#[derive(Debug, Clone, PartialEq)]
pub enum SdnError {
    NodeNotFound(NodeId),
    NodeAlreadyExists(NodeId),
    LinkNotFound { src: NodeId, dst: NodeId },
    FlowTableFull(NodeId),
    NoPathFound { src: NodeId, dst: NodeId },
    InvalidPort { node: NodeId, port: PortId },
    DuplicateFlowRule(u64),
}

impl fmt::Display for SdnError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            SdnError::NodeNotFound(id) => write!(f, "Node {} not found", id),
            SdnError::NodeAlreadyExists(id) => write!(f, "Node {} already exists", id),
            SdnError::LinkNotFound { src, dst } => write!(f, "Link {}->{} not found", src, dst),
            SdnError::FlowTableFull(id) => write!(f, "Flow table full on node {}", id),
            SdnError::NoPathFound { src, dst } => write!(f, "No path from {} to {}", src, dst),
            SdnError::InvalidPort { node, port } => write!(f, "Invalid port {} on node {}", port, node),
            SdnError::DuplicateFlowRule(id) => write!(f, "Duplicate flow rule {}", id),
        }
    }
}

// ============================================================================
// SDN Controller
// ============================================================================

/// The main SDN controller managing topology and flow rules
pub struct SdnController {
    nodes: Vec<NetworkNode>,
    links: Vec<Link>,
    flow_rules: Vec<FlowRule>,
    next_flow_id: u64,
    max_flows_per_node: usize,
}

impl SdnController {
    /// Create a new SDN controller
    pub fn new() -> Self {
        Self {
            nodes: Vec::new(),
            links: Vec::new(),
            flow_rules: Vec::new(),
            next_flow_id: 1,
            max_flows_per_node: 4096,
        }
    }

    /// Add a node to the topology
    pub fn add_node(&mut self, node: NetworkNode) -> Result<(), SdnError> {
        if self.nodes.iter().any(|n| n.id == node.id) {
            return Err(SdnError::NodeAlreadyExists(node.id));
        }
        self.nodes.push(node);
        Ok(())
    }

    /// Remove a node and its associated links
    pub fn remove_node(&mut self, node_id: NodeId) -> Result<(), SdnError> {
        let idx = self.nodes.iter().position(|n| n.id == node_id)
            .ok_or(SdnError::NodeNotFound(node_id))?;
        self.nodes.remove(idx);
        self.links.retain(|l| l.src_node != node_id && l.dst_node != node_id);
        self.flow_rules.retain(|f| f.node_id != node_id);
        Ok(())
    }

    /// Add a bidirectional link between two nodes
    pub fn add_link(&mut self, link: Link) -> Result<(), SdnError> {
        // Verify both nodes exist
        if !self.nodes.iter().any(|n| n.id == link.src_node) {
            return Err(SdnError::NodeNotFound(link.src_node));
        }
        if !self.nodes.iter().any(|n| n.id == link.dst_node) {
            return Err(SdnError::NodeNotFound(link.dst_node));
        }
        self.links.push(link);
        Ok(())
    }

    /// Remove a link
    pub fn remove_link(&mut self, src: NodeId, dst: NodeId) -> Result<(), SdnError> {
        let initial_len = self.links.len();
        self.links.retain(|l| !(l.src_node == src && l.dst_node == dst));
        if self.links.len() == initial_len {
            return Err(SdnError::LinkNotFound { src, dst });
        }
        Ok(())
    }

    /// Get the number of nodes
    pub fn node_count(&self) -> usize {
        self.nodes.len()
    }

    /// Get the number of links
    pub fn link_count(&self) -> usize {
        self.links.len()
    }

    /// Get the number of flow rules
    pub fn flow_count(&self) -> usize {
        self.flow_rules.len()
    }

    /// Find shortest path using BFS
    pub fn shortest_path(&self, src: NodeId, dst: NodeId) -> Result<Vec<NodeId>, SdnError> {
        if !self.nodes.iter().any(|n| n.id == src) {
            return Err(SdnError::NodeNotFound(src));
        }
        if !self.nodes.iter().any(|n| n.id == dst) {
            return Err(SdnError::NodeNotFound(dst));
        }

        if src == dst {
            return Ok(vec![src]);
        }

        // BFS
        let mut visited: Vec<NodeId> = Vec::new();
        let mut queue: Vec<(NodeId, Vec<NodeId>)> = Vec::new();
        queue.push((src, vec![src]));
        visited.push(src);

        while !queue.is_empty() {
            let (current, path) = queue.remove(0);

            // Find all neighbors via active links
            let neighbors: Vec<NodeId> = self.links.iter()
                .filter(|l| l.is_active && l.src_node == current)
                .map(|l| l.dst_node)
                .chain(
                    self.links.iter()
                        .filter(|l| l.is_active && l.dst_node == current)
                        .map(|l| l.src_node)
                )
                .collect();

            for neighbor in neighbors {
                if neighbor == dst {
                    let mut result = path.clone();
                    result.push(dst);
                    return Ok(result);
                }
                if !visited.contains(&neighbor) {
                    visited.push(neighbor);
                    let mut new_path = path.clone();
                    new_path.push(neighbor);
                    queue.push((neighbor, new_path));
                }
            }
        }

        Err(SdnError::NoPathFound { src, dst })
    }

    /// Install a flow rule on a node
    pub fn install_flow(
        &mut self,
        node_id: NodeId,
        priority: u32,
        match_criteria: FlowMatch,
        action: FlowAction,
    ) -> Result<u64, SdnError> {
        let node = self.nodes.iter_mut()
            .find(|n| n.id == node_id)
            .ok_or(SdnError::NodeNotFound(node_id))?;

        if node.flow_table_used >= self.max_flows_per_node {
            return Err(SdnError::FlowTableFull(node_id));
        }

        let flow_id = self.next_flow_id;
        self.next_flow_id += 1;
        node.flow_table_used += 1;

        self.flow_rules.push(FlowRule {
            id: flow_id,
            node_id,
            priority,
            match_criteria,
            action,
            packet_count: 0,
            byte_count: 0,
            idle_timeout_secs: 300,
            hard_timeout_secs: 3600,
        });

        Ok(flow_id)
    }

    /// Remove a flow rule
    pub fn remove_flow(&mut self, flow_id: u64) -> Result<(), SdnError> {
        let idx = self.flow_rules.iter().position(|f| f.id == flow_id)
            .ok_or(SdnError::DuplicateFlowRule(flow_id))?;

        let node_id = self.flow_rules[idx].node_id;
        self.flow_rules.remove(idx);

        if let Some(node) = self.nodes.iter_mut().find(|n| n.id == node_id) {
            node.flow_table_used = node.flow_table_used.saturating_sub(1);
        }

        Ok(())
    }

    /// Install flow rules along a path
    pub fn install_path_flows(
        &mut self,
        path: &[NodeId],
        match_criteria: FlowMatch,
        priority: u32,
    ) -> Result<Vec<u64>, SdnError> {
        let mut flow_ids = Vec::new();

        for i in 0..path.len().saturating_sub(1) {
            let current = path[i];
            let next = path[i + 1];

            // Find the output port for this hop
            let out_port = self.links.iter()
                .find(|l| l.src_node == current && l.dst_node == next)
                .map(|l| l.src_port)
                .or_else(|| {
                    self.links.iter()
                        .find(|l| l.dst_node == current && l.src_node == next)
                        .map(|l| l.dst_port)
                })
                .unwrap_or(1);

            let flow_id = self.install_flow(
                current,
                priority,
                match_criteria.clone(),
                FlowAction::Forward(out_port),
            )?;
            flow_ids.push(flow_id);
        }

        Ok(flow_ids)
    }

    /// Get a node by ID
    pub fn get_node(&self, node_id: NodeId) -> Option<&NetworkNode> {
        self.nodes.iter().find(|n| n.id == node_id)
    }

    /// Get all flow rules for a node
    pub fn get_node_flows(&self, node_id: NodeId) -> Vec<&FlowRule> {
        self.flow_rules.iter().filter(|f| f.node_id == node_id).collect()
    }
}

impl Default for SdnController {
    fn default() -> Self {
        Self::new()
    }
}

// ============================================================================
// Tests
// ============================================================================

#[cfg(test)]
mod tests {
    use super::*;

    fn build_topology() -> SdnController {
        let mut ctrl = SdnController::new();
        // Create a simple 4-node topology: 1 -- 2 -- 3 -- 4
        ctrl.add_node(NetworkNode::new(1, "sw1", NodeType::Switch, 4)).unwrap();
        ctrl.add_node(NetworkNode::new(2, "sw2", NodeType::Switch, 4)).unwrap();
        ctrl.add_node(NetworkNode::new(3, "sw3", NodeType::Switch, 4)).unwrap();
        ctrl.add_node(NetworkNode::new(4, "sw4", NodeType::Switch, 4)).unwrap();

        ctrl.add_link(Link::new(1, 1, 2, 1, 10000, 100)).unwrap();
        ctrl.add_link(Link::new(2, 2, 3, 1, 10000, 100)).unwrap();
        ctrl.add_link(Link::new(3, 2, 4, 1, 10000, 100)).unwrap();
        ctrl
    }

    #[test]
    fn test_add_nodes() {
        let ctrl = build_topology();
        assert_eq!(ctrl.node_count(), 4);
        assert_eq!(ctrl.link_count(), 3);
    }

    #[test]
    fn test_duplicate_node() {
        let mut ctrl = SdnController::new();
        ctrl.add_node(NetworkNode::new(1, "sw1", NodeType::Switch, 4)).unwrap();
        let result = ctrl.add_node(NetworkNode::new(1, "sw1_dup", NodeType::Switch, 4));
        assert!(matches!(result, Err(SdnError::NodeAlreadyExists(1))));
    }

    #[test]
    fn test_remove_node() {
        let mut ctrl = build_topology();
        ctrl.remove_node(2).unwrap();
        assert_eq!(ctrl.node_count(), 3);
        // Links involving node 2 should be removed
        assert_eq!(ctrl.link_count(), 0);
    }

    #[test]
    fn test_shortest_path_direct() {
        let ctrl = build_topology();
        let path = ctrl.shortest_path(1, 2).unwrap();
        assert_eq!(path, vec![1, 2]);
    }

    #[test]
    fn test_shortest_path_multi_hop() {
        let ctrl = build_topology();
        let path = ctrl.shortest_path(1, 3).unwrap();
        assert_eq!(path, vec![1, 2, 3]);
    }

    #[test]
    fn test_shortest_path_same_node() {
        let ctrl = build_topology();
        let path = ctrl.shortest_path(1, 1).unwrap();
        assert_eq!(path, vec![1]);
    }

    #[test]
    fn test_no_path() {
        let mut ctrl = SdnController::new();
        ctrl.add_node(NetworkNode::new(1, "sw1", NodeType::Switch, 4)).unwrap();
        ctrl.add_node(NetworkNode::new(2, "sw2", NodeType::Switch, 4)).unwrap();
        // No link between them
        let result = ctrl.shortest_path(1, 2);
        assert!(matches!(result, Err(SdnError::NoPathFound { .. })));
    }

    #[test]
    fn test_install_flow() {
        let mut ctrl = build_topology();
        let flow_id = ctrl.install_flow(
            1, 100,
            FlowMatch { dst_ip: Some(0x0A000001), ..FlowMatch::new() },
            FlowAction::Forward(1),
        ).unwrap();
        assert_eq!(flow_id, 1);
        assert_eq!(ctrl.flow_count(), 1);
        assert_eq!(ctrl.get_node(1).unwrap().flow_table_used, 1);
    }

    #[test]
    fn test_remove_flow() {
        let mut ctrl = build_topology();
        let flow_id = ctrl.install_flow(
            1, 100, FlowMatch::new(), FlowAction::Drop,
        ).unwrap();
        ctrl.remove_flow(flow_id).unwrap();
        assert_eq!(ctrl.flow_count(), 0);
        assert_eq!(ctrl.get_node(1).unwrap().flow_table_used, 0);
    }

    #[test]
    fn test_install_path_flows() {
        let mut ctrl = build_topology();
        let path = ctrl.shortest_path(1, 4).unwrap();
        let flow_ids = ctrl.install_path_flows(
            &path,
            FlowMatch { dst_ip: Some(0x0A000004), ..FlowMatch::new() },
            100,
        ).unwrap();
        // Path is [1, 2, 4], so 2 flow rules (on nodes 1 and 2)
        assert_eq!(flow_ids.len(), 2);
        assert_eq!(ctrl.flow_count(), 2);
    }

    #[test]
    fn test_flow_match_specificity() {
        let m = FlowMatch {
            src_ip: Some(1), dst_ip: Some(2), protocol: Some(6),
            ..FlowMatch::new()
        };
        assert_eq!(m.specificity(), 3);
    }
}