//! Utilities to parse visual processed files.

use glam::Affine3A;
use serde::{Deserialize, Serialize};


#[derive(Debug, Deserialize, Serialize)]
pub struct Visual {
    /// The root node.
    #[serde(rename = "node")]
    root_node: Node,
}


#[derive(Debug, Deserialize, Serialize)]
pub struct Node {
    /// Identifier of the node.
    #[serde(rename = "$unflatten=identifier")]
    identifier: String,
    /// Children nodes.
    #[serde(rename = "node")]
    nodes: Vec<Node>
}
