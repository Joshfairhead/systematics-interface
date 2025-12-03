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

/// All systems query response
#[derive(Deserialize, Debug)]
struct AllSystemsQueryResponse {
    #[serde(rename = "allSystems")]
    all_systems: Vec<GQLSystem>,
}

/// GraphQL System type (matches the API schema)
#[derive(Deserialize, Debug, Clone)]
struct GQLSystem {
    name: String,
    #[serde(rename = "coherenceAttribute")]
    coherence_attribute: String,
    #[serde(rename = "termDesignation")]
    term_designation: String,
    #[serde(rename = "connectiveDesignation")]
    connective_designation: String,
    source: String,
    nodes: Vec<i32>,  // Just array of integers
    edges: Vec<GQLEdge>,
    points: Vec<GQLCoordinate>,
    lines: Vec<GQLLine>,
    #[serde(rename = "termCharacters")]
    term_characters: Vec<GQLTerm>,  // Array of Term objects
    #[serde(rename = "connectiveCharacters")]
    connective_characters: Vec<GQLConnector>,
}

#[derive(Deserialize, Debug, Clone)]
struct GQLTopology {
    #[serde(rename = "systemName")]
    system_name: String,
    nodes: Vec<GQLNode>,
    edges: Vec<GQLEdge>,
}

#[derive(Deserialize, Debug, Clone)]
struct GQLGeometry {
    #[serde(rename = "systemName")]
    system_name: String,
    coordinates: Vec<GQLCoordinate>,
    lines: Option<Vec<GQLLine>>,
}

#[derive(Deserialize, Debug, Clone)]
struct GQLNode {
    index: usize,
}

#[derive(Deserialize, Debug, Clone)]
struct GQLEdge {
    from: usize,
    to: usize,
}

#[derive(Deserialize, Debug, Clone)]
struct GQLCoordinate {
    x: f64,
    y: f64,
    z: Option<f64>,
}

#[derive(Deserialize, Debug, Clone)]
struct GQLLine {
    start: GQLCoordinate,
    end: GQLCoordinate,
}

#[derive(Deserialize, Debug, Clone)]
struct GQLTerm {
    name: String,
    node: i32,
}

#[derive(Deserialize, Debug, Clone)]
struct GQLConnector {
    name: String,
    #[serde(rename = "fromTerm")]
    from_term: String,
    #[serde(rename = "toTerm")]
    to_term: String,
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

    /// Fetch a single system by name
    pub async fn fetch_system(&self, system_name: &str) -> Result<SystemData, ApiError> {
        let query = r#"
            query GetSystem($name: String!) {
                system(name: $name) {
                    name
                    coherenceAttribute
                    termDesignation
                    connectiveDesignation
                    source
                    nodes
                    edges {
                        from
                        to
                    }
                    points {
                        x
                        y
                        z
                    }
                    lines {
                        start {
                            x
                            y
                            z
                        }
                        end {
                            x
                            y
                            z
                        }
                    }
                    termCharacters {
                        name
                        node
                    }
                    connectiveCharacters {
                        name
                        fromTerm
                        toTerm
                    }
                }
            }
        "#;

        let variables = serde_json::json!({
            "name": system_name
        });

        let response: GraphQLResponse<SystemQueryResponse> =
            self.execute_query(query, Some(variables)).await?;

        if let Some(errors) = response.errors {
            return Err(ApiError::ParseError(
                errors.iter().map(|e| e.message.clone()).collect::<Vec<_>>().join(", ")
            ));
        }

        let data = response.data
            .ok_or_else(|| ApiError::NotFound(format!("System {} not found", system_name)))?;

        let system = data.system
            .ok_or_else(|| ApiError::NotFound(format!("System {} not found", system_name)))?;

        Ok(self.convert_gql_system_to_system_data(system))
    }

    /// Fetch all available systems
    pub async fn fetch_all_systems(&self) -> Result<Vec<SystemData>, ApiError> {
        let query = r#"
            query GetAllSystems {
                allSystems {
                    name
                    coherenceAttribute
                    termDesignation
                    connectiveDesignation
                    source
                    nodes
                    edges {
                        from
                        to
                    }
                    points {
                        x
                        y
                        z
                    }
                    lines {
                        start {
                            x
                            y
                            z
                        }
                        end {
                            x
                            y
                            z
                        }
                    }
                    termCharacters {
                        name
                        node
                    }
                    connectiveCharacters {
                        name
                        fromTerm
                        toTerm
                    }
                }
            }
        "#;

        let response: GraphQLResponse<AllSystemsQueryResponse> =
            self.execute_query(query, None).await?;

        if let Some(errors) = response.errors {
            return Err(ApiError::ParseError(
                errors.iter().map(|e| e.message.clone()).collect::<Vec<_>>().join(", ")
            ));
        }

        let data = response.data
            .ok_or_else(|| ApiError::NotFound("No systems found".to_string()))?;

        Ok(data.all_systems.into_iter()
            .map(|sys| self.convert_gql_system_to_system_data(sys))
            .collect())
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
        let node_count = gql_system.nodes.len();

        // Convert system name to lowercase for consistency
        let system_name = gql_system.name.to_lowercase();

        // Get color scheme from legacy config or use default
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
                (display_name, k_notation, system_name.clone())
            });

        // Convert coordinates (points in the API)
        let raw_coordinates: Vec<Coordinate> = gql_system.points
            .iter()
            .map(|c| Coordinate {
                x: c.x,
                y: c.y,
                z: c.z,
            })
            .collect();

        // Transform coordinates to fit in 800x800 viewport with margins
        let coordinates = transform_coordinates_to_viewport(raw_coordinates, 800.0, 800.0, 100.0);

        // Convert edges
        let edges: Vec<TopologyEdge> = gql_system.edges
            .iter()
            .map(|e| TopologyEdge {
                from: e.from,
                to: e.to,
            })
            .collect();

        // Convert nodes (array of i32) to indexes (array of usize)
        let indexes: Vec<usize> = gql_system.nodes
            .iter()
            .map(|&n| n as usize)
            .collect();

        // Extract term names from Term objects
        let terms: Vec<String> = gql_system.term_characters
            .iter()
            .map(|t| t.name.clone())
            .collect();

        // Convert connectors (connective characters)
        let connectives: Vec<(String, String, String)> = gql_system.connective_characters
            .iter()
            .map(|c| (c.name.clone(), c.from_term.clone(), c.to_term.clone()))
            .collect();

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
            connectives,
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
    // 3. Translate to viewport center
    coords
        .into_iter()
        .map(|coord| Coordinate {
            x: (coord.x - center_x) * scale + viewport_center_x,
            y: (coord.y - center_y) * scale + viewport_center_y,
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
