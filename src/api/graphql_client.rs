use gloo_net::http::Request;
use serde::{Deserialize, Serialize};
use crate::api::models::{SystemData, ColorScheme, Coordinate, TopologyEdge};
use crate::api::client::ApiError;
use crate::core::system_config::SystemConfig;

/// GraphQL request structure
#[derive(Serialize)]
struct GraphQLRequest {
    query: String,
    #[serde(skip_serializing_if = "Option::is_none")]
    variables: Option<serde_json::Value>,
}

/// GraphQL response structure
#[derive(Deserialize, Debug)]
struct GraphQLResponse<T> {
    data: Option<T>,
    errors: Option<Vec<GraphQLError>>,
}

#[derive(Deserialize, Debug)]
struct GraphQLError {
    message: String,
}

/// System query response
#[derive(Deserialize, Debug)]
struct SystemQueryResponse {
    system: Option<GQLSystem>,
}

/// All systems query response (queries systems 1-12)
#[derive(Deserialize, Debug)]
struct SystemsQueryResponse {
    systems: Vec<GQLSystem>,
}

/// GraphQL System type (matches GqlSystemView from actual backend)
#[derive(Deserialize, Debug, Clone)]
struct GQLSystem {
    name: Option<String>,
    coherence: Option<String>,
    #[serde(rename = "termDesignation")]
    term_designation: Option<String>,
    #[serde(rename = "connectiveDesignation")]
    connective_designation: Option<String>,
    terms: Vec<GQLTerm>,
    coordinates: Vec<GQLCoordinate>,
    colours: Vec<GQLColour>,
    lines: Vec<GQLLink>,
    connectives: Vec<GQLLink>,
}

/// Term with character
#[derive(Deserialize, Debug, Clone)]
struct GQLTerm {
    position: i32,
    character: Option<GQLCharacter>,
}

/// Character value
#[derive(Deserialize, Debug, Clone)]
struct GQLCharacter {
    value: String,
}

/// Coordinate in 3D space
#[derive(Deserialize, Debug, Clone)]
struct GQLCoordinate {
    position: i32,
    x: f64,
    y: f64,
    z: f64,
}

/// Color value
#[derive(Deserialize, Debug, Clone)]
struct GQLColour {
    position: i32,
    value: String,  // Hex color (e.g., "#FF0000")
}

/// Link (used for both geometric lines and semantic connectives)
#[derive(Deserialize, Debug, Clone)]
struct GQLLink {
    #[serde(rename = "baseCoordinate")]
    base_coordinate: Option<GQLCoordinate>,
    #[serde(rename = "targetCoordinate")]
    target_coordinate: Option<GQLCoordinate>,
    #[serde(rename = "basePosition")]
    base_position: Option<i32>,
    #[serde(rename = "targetPosition")]
    target_position: Option<i32>,
    character: Option<GQLCharacter>,
}

/// GraphQL API client for systematics data
#[derive(Clone)]
pub struct GraphQLClient {
    endpoint: String,
}

impl GraphQLClient {
    /// Create a new GraphQL client with the specified endpoint
    pub fn new(endpoint: String) -> Self {
        Self { endpoint }
    }

    /// Fetch a single system by order (1-12)
    pub async fn fetch_system_by_order(&self, order: i32) -> Result<SystemData, ApiError> {
        let query = r#"
            query GetSystem($order: Int!) {
                system(order: $order) {
                    name
                    coherence
                    termDesignation
                    connectiveDesignation
                    terms {
                        position
                        character {
                            value
                        }
                    }
                    coordinates {
                        position
                        x
                        y
                        z
                    }
                    colours {
                        position
                        value
                    }
                    lines {
                        baseCoordinate {
                            x
                            y
                            z
                        }
                        targetCoordinate {
                            x
                            y
                            z
                        }
                        basePosition
                        targetPosition
                    }
                    connectives {
                        basePosition
                        targetPosition
                        character {
                            value
                        }
                    }
                }
            }
        "#;

        let variables = serde_json::json!({
            "order": order
        });

        let response: GraphQLResponse<SystemQueryResponse> =
            self.execute_query(query, Some(variables)).await?;

        if let Some(errors) = response.errors {
            return Err(ApiError::ParseError(
                errors.iter().map(|e| e.message.clone()).collect::<Vec<_>>().join(", ")
            ));
        }

        let data = response.data
            .ok_or_else(|| ApiError::NotFound(format!("System with order {} not found", order)))?;

        let system = data.system
            .ok_or_else(|| ApiError::NotFound(format!("System with order {} not found", order)))?;

        Ok(self.convert_gql_system_to_system_data(system))
    }

    /// Fetch a single system by name (converts name to order)
    pub async fn fetch_system(&self, system_name: &str) -> Result<SystemData, ApiError> {
        // Map system names to orders
        let order = match system_name.to_lowercase().as_str() {
            "monad" => 1,
            "dyad" => 2,
            "triad" => 3,
            "tetrad" => 4,
            "pentad" => 5,
            "hexad" => 6,
            "heptad" => 7,
            "octad" => 8,
            "ennead" => 9,
            "decad" => 10,
            "hendecad" => 11,
            "duodecad" => 12,
            _ => return Err(ApiError::NotFound(format!("Unknown system name: {}", system_name))),
        };

        self.fetch_system_by_order(order).await
    }

    /// Fetch all available systems (orders 1-12)
    pub async fn fetch_all_systems(&self) -> Result<Vec<SystemData>, ApiError> {
        // Query all systems by fetching each order individually
        let mut systems = Vec::new();

        for order in 1..=12 {
            match self.fetch_system_by_order(order).await {
                Ok(system) => systems.push(system),
                Err(e) => {
                    // Log warning but continue with other systems
                    web_sys::console::warn_1(&format!("Failed to fetch system order {}: {:?}", order, e).into());
                }
            }
        }

        if systems.is_empty() {
            return Err(ApiError::NotFound("No systems found".to_string()));
        }

        Ok(systems)
    }

    /// Execute a GraphQL query
    async fn execute_query<T: for<'de> Deserialize<'de>>(
        &self,
        query: &str,
        variables: Option<serde_json::Value>,
    ) -> Result<GraphQLResponse<T>, ApiError> {
        let request_body = GraphQLRequest {
            query: query.to_string(),
            variables,
        };

        let response = Request::post(&self.endpoint)
            .header("Content-Type", "application/json")
            .json(&request_body)
            .map_err(|e| ApiError::ParseError(e.to_string()))?
            .send()
            .await
            .map_err(|e| ApiError::NetworkError(e.to_string()))?;

        if !response.ok() {
            return Err(ApiError::NetworkError(format!(
                "Request failed with status: {}",
                response.status()
            )));
        }

        response
            .json::<GraphQLResponse<T>>()
            .await
            .map_err(|e| ApiError::ParseError(e.to_string()))
    }

    /// Convert GraphQL system to internal SystemData model
    fn convert_gql_system_to_system_data(&self, gql_system: GQLSystem) -> SystemData {
        // Get node count from terms/coordinates array
        let node_count = gql_system.terms.len().max(gql_system.coordinates.len());

        // Convert system name to lowercase for consistency
        let system_name = gql_system.name
            .as_deref()
            .unwrap_or("unknown")
            .to_lowercase();

        // Get default color scheme from legacy config or default
        let color_scheme = SystemConfig::get_by_name(&system_name)
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

        // Get metadata from legacy config for display name and description
        let (display_name, k_notation, description) = SystemConfig::get_by_name(&system_name)
            .map(|config| (config.display_name, config.k_notation, config.description))
            .unwrap_or_else(|| {
                let k_notation = format!("K{}", node_count);
                let display_name = capitalize_first(&system_name);
                let desc = gql_system.coherence
                    .as_deref()
                    .unwrap_or(&system_name)
                    .to_string();
                (display_name, k_notation, desc)
            });

        // Sort coordinates by position
        let mut coords_sorted = gql_system.coordinates.clone();
        coords_sorted.sort_by_key(|c| c.position);

        // Extract raw coordinates
        let raw_coordinates: Vec<Coordinate> = coords_sorted
            .iter()
            .map(|c| Coordinate {
                x: c.x,
                y: c.y,
                z: Some(c.z),
            })
            .collect();

        // Transform coordinates to fit in 800x800 viewport with margins
        let coordinates = transform_coordinates_to_viewport(raw_coordinates, 800.0, 800.0, 100.0);

        // Convert edges from lines using positions
        let mut edges: Vec<TopologyEdge> = Vec::new();
        for line in &gql_system.lines {
            if let (Some(base_pos), Some(target_pos)) = (line.base_position, line.target_position) {
                // Positions are 1-based, convert to 0-based indices
                if base_pos > 0 && target_pos > 0 {
                    edges.push(TopologyEdge {
                        from: (base_pos - 1) as usize,
                        to: (target_pos - 1) as usize,
                    });
                }
            }
        }

        // Create indexes (zero-based sequential indices for all nodes)
        let indexes: Vec<usize> = (0..node_count).collect();

        // Sort terms by position and extract names
        let mut terms_sorted = gql_system.terms.clone();
        terms_sorted.sort_by_key(|t| t.position);

        let terms: Vec<String> = terms_sorted
            .iter()
            .filter_map(|t| t.character.as_ref().map(|c| c.value.clone()))
            .collect();

        // Sort colours by position and extract values
        let mut colours_sorted = gql_system.colours.clone();
        colours_sorted.sort_by_key(|c| c.position);

        let term_colors: Vec<String> = colours_sorted
            .iter()
            .map(|c| c.value.clone())
            .collect();

        // Convert connectives to internal format
        // Build a position->term lookup for connectives
        let term_by_position: std::collections::HashMap<i32, String> = terms_sorted
            .iter()
            .filter_map(|t| {
                t.character.as_ref().map(|c| (t.position, c.value.clone()))
            })
            .collect();

        let connectives: Vec<(String, String, String)> = gql_system.connectives
            .iter()
            .filter_map(|c| {
                if let (Some(base_pos), Some(target_pos), Some(char_val)) = (
                    c.base_position,
                    c.target_position,
                    c.character.as_ref().map(|ch| ch.value.clone())
                ) {
                    if let (Some(base_term), Some(target_term)) = (
                        term_by_position.get(&base_pos),
                        term_by_position.get(&target_pos)
                    ) {
                        return Some((char_val, base_term.clone(), target_term.clone()));
                    }
                }
                None
            })
            .collect();

        // No navigation edges in the new API - leave empty for now
        let navigation_edges: Vec<crate::api::models::NavigationEdge> = Vec::new();

        SystemData {
            system_name,
            display_name,
            k_notation,
            description,
            node_count,
            coordinates,
            indexes,
            edges,
            color_scheme,
            terms,
            term_colors,
            connectives,
            navigation_edges,
        }
    }
}

/// Transform coordinates from API space to viewport space
///
/// The API may return coordinates in any scale (e.g., 0-1, 0-10, or even 0,0,0 for single points).
/// This function scales and centers them to fit within the viewport with margins.
fn transform_coordinates_to_viewport(
    coords: Vec<Coordinate>,
    viewport_width: f64,
    viewport_height: f64,
    margin: f64,
) -> Vec<Coordinate> {
    if coords.is_empty() {
        return coords;
    }

    // For a single point, center it in the viewport
    if coords.len() == 1 {
        return vec![Coordinate {
            x: viewport_width / 2.0,
            y: viewport_height / 2.0,
            z: coords[0].z,
        }];
    }

    // Find bounding box to determine scale
    let mut min_x = f64::INFINITY;
    let mut max_x = f64::NEG_INFINITY;
    let mut min_y = f64::INFINITY;
    let mut max_y = f64::NEG_INFINITY;

    for coord in &coords {
        min_x = min_x.min(coord.x);
        max_x = max_x.max(coord.x);
        min_y = min_y.min(coord.y);
        max_y = max_y.max(coord.y);
    }

    // Calculate the full extent needed to contain all points
    // Use the center of bounding box as origin, and max extent for scaling
    let center_x = (min_x + max_x) / 2.0;
    let center_y = (min_y + max_y) / 2.0;

    let extent_x = (max_x - min_x).max(0.0001);
    let extent_y = (max_y - min_y).max(0.0001);

    // Use the larger extent for both axes to preserve aspect ratio and coordinate system
    let max_extent = extent_x.max(extent_y);

    // Calculate available space (viewport minus margins on both sides)
    let available_width = viewport_width - 2.0 * margin;
    let available_height = viewport_height - 2.0 * margin;

    // Use smaller dimension to ensure graph fits in viewport
    let available_size = available_width.min(available_height);

    // Scale to fit available space
    let scale = available_size / max_extent;

    // Viewport center
    let viewport_center_x = viewport_width / 2.0;
    let viewport_center_y = viewport_height / 2.0;

    // Transform all coordinates:
    // 1. Translate to center at origin
    // 2. Scale
    // 3. Flip Y-axis (mathematical coords: y+ = up, SVG coords: y+ = down)
    // 4. Translate to viewport center
    coords
        .into_iter()
        .map(|coord| Coordinate {
            x: (coord.x - center_x) * scale + viewport_center_x,
            y: -(coord.y - center_y) * scale + viewport_center_y,  // Negate Y for SVG
            z: coord.z,
        })
        .collect()
}

/// Helper function to capitalize the first letter
fn capitalize_first(s: &str) -> String {
    let mut chars = s.chars();
    match chars.next() {
        None => String::new(),
        Some(first) => first.to_uppercase().collect::<String>() + chars.as_str(),
    }
}
