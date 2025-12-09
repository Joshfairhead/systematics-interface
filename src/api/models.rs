use serde::{Deserialize, Serialize};

/// Represents a coordinate point (matches v0.0.3 Coordinates struct)
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub struct Coordinate {
    pub x: f64,
    pub y: f64,
    /// Z-coordinate (optional, interface uses 2D)
    #[serde(default)]
    pub z: Option<f64>,
}

/// Represents an edge connecting two nodes by their indexes
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub struct TopologyEdge {
    pub from: usize,
    pub to: usize,
}

/// Geometry data from API - contains node indexes and their coordinates
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub struct GeometryData {
    pub system_name: String,
    pub k_notation: String,
    pub node_count: usize,
    pub coordinates: Vec<Coordinate>,
    pub indexes: Vec<usize>,
    /// Optional: edges may still be in geometry data during transition
    #[serde(default)]
    pub edges: Vec<TopologyEdge>,
}

/// Topology data from API - contains node indexes and edges (future structure)
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub struct TopologyData {
    pub system_name: String,
    pub k_notation: String,
    pub node_count: usize,
    pub indexes: Vec<usize>,
    pub edges: Vec<TopologyEdge>,
}

/// Vocabulary/terminology data for a system (matches v0.0.3 structure)
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub struct VocabularyData {
    pub system_name: String,
    pub display_name: String,
    pub k_notation: String,
    pub description: String,
    /// Terminal characters/terms for this system
    pub term_characters: Vec<String>,
    /// Connective characters (relationships between terms)
    #[serde(default)]
    pub connective_characters: Vec<(String, String, String)>,
}

impl VocabularyData {
    /// Get all terms (for backward compatibility)
    pub fn terms(&self) -> Vec<String> {
        self.term_characters.clone()
    }
}

/// Navigation edge for node-based system navigation
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub struct NavigationEdge {
    pub node: usize,          // Zero-based internally
    pub target_system: String,
}

/// Complete system data combining topology, geometry, and vocabulary
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub struct SystemData {
    pub system_name: String,
    pub display_name: String,
    pub k_notation: String,
    pub description: String,
    pub node_count: usize,
    pub coordinates: Vec<Coordinate>,
    pub indexes: Vec<usize>,
    pub edges: Vec<TopologyEdge>,
    pub color_scheme: ColorScheme,
    #[serde(default)]
    pub terms: Vec<String>,
    /// Connective characters from vocabulary
    #[serde(default)]
    pub connectives: Vec<(String, String, String)>,
    /// Navigation edges for node-based system navigation
    #[serde(default)]
    pub navigation_edges: Vec<NavigationEdge>,
}

/// Color scheme for rendering
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub struct ColorScheme {
    pub nodes: String,
    pub edges: String,
    pub selected_node: String,
    pub selected_edge: String,
}

impl SystemData {
    /// Merge topology, geometry, and vocabulary data into complete system data
    pub fn from_api_data(
        geometry: GeometryData,
        topology: Option<TopologyData>,
        vocabulary: VocabularyData,
        color_scheme: ColorScheme,
    ) -> Self {
        let edges = if let Some(topo) = topology {
            topo.edges
        } else {
            // Fallback to edges in geometry data during transition
            geometry.edges.clone()
        };

        SystemData {
            system_name: geometry.system_name.clone(),
            display_name: vocabulary.display_name,
            k_notation: geometry.k_notation,
            description: vocabulary.description,
            node_count: geometry.node_count,
            coordinates: geometry.coordinates,
            indexes: geometry.indexes,
            edges,
            color_scheme,
            terms: vocabulary.term_characters,
            connectives: vocabulary.connective_characters,
            navigation_edges: vec![],  // Empty by default, filled by GraphQL client
        }
    }

    /// Generate complete graph edges if API doesn't provide them
    pub fn with_complete_graph_edges(mut self) -> Self {
        if self.edges.is_empty() {
            self.edges = Self::generate_complete_graph_edges(self.node_count);
        }
        self
    }

    fn generate_complete_graph_edges(node_count: usize) -> Vec<TopologyEdge> {
        let mut edges = Vec::new();
        for i in 0..node_count {
            for j in (i + 1)..node_count {
                edges.push(TopologyEdge { from: i, to: j });
            }
        }
        edges
    }
}
