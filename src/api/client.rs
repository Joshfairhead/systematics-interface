use gloo_net::http::Request;
use serde::de::DeserializeOwned;
use crate::api::models::{GeometryData, TopologyData, VocabularyData, SystemData, ColorScheme};
use crate::core::system_config::SystemConfig;

/// API client for fetching systematics data
pub struct ApiClient {
    base_url: String,
}

#[derive(Debug)]
pub enum ApiError {
    NetworkError(String),
    ParseError(String),
    NotFound(String),
}

impl std::fmt::Display for ApiError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            ApiError::NetworkError(msg) => write!(f, "Network error: {}", msg),
            ApiError::ParseError(msg) => write!(f, "Parse error: {}", msg),
            ApiError::NotFound(msg) => write!(f, "Not found: {}", msg),
        }
    }
}

impl std::error::Error for ApiError {}

impl ApiClient {
    /// Create a new API client with the specified base URL
    pub fn new(base_url: String) -> Self {
        Self { base_url }
    }

    /// Fetch geometry data for a system
    pub async fn fetch_geometry(&self, system_name: &str) -> Result<GeometryData, ApiError> {
        let url = format!("{}/geometry/{}", self.base_url, system_name);
        self.fetch_json(&url).await
    }

    /// Fetch topology data for a system
    pub async fn fetch_topology(&self, system_name: &str) -> Result<TopologyData, ApiError> {
        let url = format!("{}/topology/{}", self.base_url, system_name);
        self.fetch_json(&url).await
    }

    /// Fetch vocabulary data for a system
    pub async fn fetch_vocabulary(&self, system_name: &str) -> Result<VocabularyData, ApiError> {
        let url = format!("{}/vocabulary/{}", self.base_url, system_name);
        self.fetch_json(&url).await
    }

    /// Fetch complete system data (combines all three data sources)
    pub async fn fetch_system(&self, system_name: &str) -> Result<SystemData, ApiError> {
        // Fetch all three data sources in parallel
        let geometry_future = self.fetch_geometry(system_name);
        let topology_future = self.fetch_topology(system_name);
        let vocabulary_future = self.fetch_vocabulary(system_name);

        // Wait for all futures
        let geometry = geometry_future.await?;
        let topology = topology_future.await.ok(); // Topology is optional during transition
        let vocabulary = vocabulary_future.await?;

        // Get color scheme from legacy config or use default
        let color_scheme = SystemConfig::get_by_name(system_name)
            .map(|config| ColorScheme {
                nodes: config.color_scheme.nodes,
                edges: config.color_scheme.edges,
                selected_node: config.color_scheme.selected_node,
                selected_edge: config.color_scheme.selected_edge,
            })
            .unwrap_or_else(|| ColorScheme {
                nodes: "#4A90E2".to_string(),
                edges: "#888888".to_string(),
                selected_node: "#FF6B6B".to_string(),
                selected_edge: "#FF6B6B".to_string(),
            });

        Ok(SystemData::from_api_data(geometry, topology, vocabulary, color_scheme))
    }

    /// Fetch all available systems
    pub async fn fetch_all_systems(&self) -> Result<Vec<SystemData>, ApiError> {
        let url = format!("{}/systems", self.base_url);
        self.fetch_json(&url).await
    }

    /// Generic JSON fetch helper
    async fn fetch_json<T: DeserializeOwned>(&self, url: &str) -> Result<T, ApiError> {
        let response = Request::get(url)
            .send()
            .await
            .map_err(|e| ApiError::NetworkError(e.to_string()))?;

        if !response.ok() {
            return Err(ApiError::NotFound(format!(
                "Request failed with status: {}",
                response.status()
            )));
        }

        response
            .json::<T>()
            .await
            .map_err(|e| ApiError::ParseError(e.to_string()))
    }
}

/// Create a mock API client for development/testing
pub struct MockApiClient;

impl MockApiClient {
    /// Generate mock geometry data from the existing geometry calculator
    pub async fn fetch_geometry(system_name: &str) -> Result<GeometryData, ApiError> {
        use crate::core::geometry::GeometryCalculator;

        let node_count = match system_name {
            "monad" => 1, "dyad" => 2, "triad" => 3, "tetrad" => 4,
            "pentad" => 5, "hexad" => 6, "heptad" => 7, "octad" => 8,
            "ennead" => 9, "decad" => 10, "undecad" => 11, "dodecad" => 12,
            _ => return Err(ApiError::NotFound(format!("Unknown system: {}", system_name))),
        };

        let layout = GeometryCalculator::calculate_system_layout(system_name, 400.0, 400.0, 700.0);

        let coordinates: Vec<crate::api::models::Coordinate> = layout.nodes
            .iter()
            .map(|p| crate::api::models::Coordinate {
                x: p.x,
                y: p.y,
                z: None, // 2D interface, z not used
            })
            .collect();

        let edges: Vec<crate::api::models::TopologyEdge> = layout.edges
            .iter()
            .map(|e| crate::api::models::TopologyEdge { from: e.from, to: e.to })
            .collect();

        Ok(GeometryData {
            system_name: system_name.to_string(),
            k_notation: format!("K{}", node_count),
            node_count,
            coordinates,
            indexes: (0..node_count).collect(),
            edges,
        })
    }

    /// Generate mock vocabulary data matching v0.0.3 structure
    pub async fn fetch_vocabulary(system_name: &str) -> Result<VocabularyData, ApiError> {
        let config = SystemConfig::get_by_name(system_name)
            .ok_or_else(|| ApiError::NotFound(format!("Unknown system: {}", system_name)))?;

        // Mock term characters based on system type
        let term_characters = Self::get_term_characters(system_name);
        let connective_characters = Self::get_connective_characters(system_name);

        Ok(VocabularyData {
            system_name: system_name.to_string(),
            display_name: config.display_name,
            k_notation: config.k_notation,
            description: config.description,
            term_characters,
            connective_characters,
        })
    }

    /// Get mock term characters for a system
    fn get_term_characters(system_name: &str) -> Vec<String> {
        match system_name {
            "monad" => vec!["Unity".to_string()],
            "dyad" => vec!["Positive".to_string(), "Negative".to_string()],
            "triad" => vec!["Thesis".to_string(), "Antithesis".to_string(), "Synthesis".to_string()],
            "tetrad" => vec!["North".to_string(), "East".to_string(), "South".to_string(), "West".to_string()],
            "pentad" => vec![
                "Purpose".to_string(),
                "Higher Potential".to_string(),
                "Quintessence".to_string(),
                "Lower Potential".to_string(),
                "Source".to_string()
            ],
            "hexad" => (1..=6).map(|i| format!("Node {}", i)).collect(),
            "heptad" => (1..=7).map(|i| format!("Node {}", i)).collect(),
            "octad" => (1..=8).map(|i| format!("Node {}", i)).collect(),
            "ennead" => (1..=9).map(|i| format!("Node {}", i)).collect(),
            "decad" => (1..=10).map(|i| format!("Node {}", i)).collect(),
            "undecad" => (1..=11).map(|i| format!("Node {}", i)).collect(),
            "dodecad" => (1..=12).map(|i| format!("Node {}", i)).collect(),
            _ => vec![],
        }
    }

    /// Get mock connective characters (relationships between terms)
    fn get_connective_characters(system_name: &str) -> Vec<(String, String, String)> {
        match system_name {
            "monad" => vec![], // No connections in monad
            "dyad" => vec![
                ("Polarity".to_string(), "Positive".to_string(), "Negative".to_string())
            ],
            "pentad" => vec![
                ("Aspiration".to_string(), "Source".to_string(), "Purpose".to_string()),
                ("Input".to_string(), "Source".to_string(), "Lower Potential".to_string()),
                ("Output".to_string(), "Higher Potential".to_string(), "Purpose".to_string()),
            ],
            _ => vec![], // Simplified for other systems
        }
    }

    /// Generate mock complete system data
    pub async fn fetch_system(system_name: &str) -> Result<SystemData, ApiError> {
        let geometry = Self::fetch_geometry(system_name).await?;
        let vocabulary = Self::fetch_vocabulary(system_name).await?;

        let color_scheme = SystemConfig::get_by_name(system_name)
            .map(|config| ColorScheme {
                nodes: config.color_scheme.nodes,
                edges: config.color_scheme.edges,
                selected_node: config.color_scheme.selected_node,
                selected_edge: config.color_scheme.selected_edge,
            })
            .unwrap_or_else(|| ColorScheme {
                nodes: "#4A90E2".to_string(),
                edges: "#888888".to_string(),
                selected_node: "#FF6B6B".to_string(),
                selected_edge: "#FF6B6B".to_string(),
            });

        Ok(SystemData::from_api_data(geometry, None, vocabulary, color_scheme))
    }

    /// Generate mock list of all systems
    pub async fn fetch_all_systems() -> Result<Vec<SystemData>, ApiError> {
        let system_names = vec![
            "monad", "dyad", "triad", "tetrad", "pentad", "hexad",
            "heptad", "octad", "ennead", "decad", "undecad", "dodecad"
        ];

        let mut systems = Vec::new();
        for name in system_names {
            if let Ok(system) = Self::fetch_system(name).await {
                systems.push(system);
            }
        }

        Ok(systems)
    }
}
