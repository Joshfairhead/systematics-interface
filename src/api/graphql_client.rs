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
    #[serde(rename = "coherenceAttributes")]
    coherence_attributes: Option<Vec<String>>,
    #[serde(rename = "termDesignation")]
    term_designation: Option<String>,
    #[serde(rename = "connectiveDesignation")]
    connective_designation: Option<String>,
    source: Option<String>,
    topology: GQLTopology,
    geometry: GQLGeometry,
    terms: Vec<GQLTerm>,
    connectors: Vec<GQLConnector>,
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
    system: String,
    #[serde(rename = "nodeIndex")]
    node_index: usize,
    coordinate: Option<GQLCoordinate>,
}

#[derive(Deserialize, Debug, Clone)]
struct GQLConnector {
    name: String,
    from: String,
    to: String,
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
                    coherenceAttributes
                    termDesignation
                    connectiveDesignation
                    source
                    topology {
                        systemName
                        nodes {
                            index
                        }
                        edges {
                            from
                            to
                        }
                    }
                    geometry {
                        systemName
                        coordinates {
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
                    }
                    terms {
                        name
                        system
                        nodeIndex
                        coordinate {
                            x
                            y
                            z
                        }
                    }
                    connectors {
                        name
                        from
                        to
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
                    coherenceAttributes
                    termDesignation
                    connectiveDesignation
                    source
                    topology {
                        systemName
                        nodes {
                            index
                        }
                        edges {
                            from
                            to
                        }
                    }
                    geometry {
                        systemName
                        coordinates {
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
                    }
                    terms {
                        name
                        system
                        nodeIndex
                        coordinate {
                            x
                            y
                            z
                        }
                    }
                    connectors {
                        name
                        from
                        to
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
        let node_count = gql_system.topology.nodes.len();

        // Get color scheme from legacy config or use default
        let color_scheme = SystemConfig::get_by_name(&gql_system.name)
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
        let (display_name, k_notation, description) = SystemConfig::get_by_name(&gql_system.name)
            .map(|config| (config.display_name, config.k_notation, config.description))
            .unwrap_or_else(|| {
                let k_notation = format!("K{}", node_count);
                let display_name = capitalize_first(&gql_system.name);
                (display_name, k_notation, gql_system.name.clone())
            });

        // Convert coordinates
        let coordinates: Vec<Coordinate> = gql_system.geometry.coordinates
            .iter()
            .map(|c| Coordinate {
                x: c.x,
                y: c.y,
                z: c.z,
            })
            .collect();

        // Convert edges
        let edges: Vec<TopologyEdge> = gql_system.topology.edges
            .iter()
            .map(|e| TopologyEdge {
                from: e.from,
                to: e.to,
            })
            .collect();

        // Get indexes from topology nodes
        let indexes: Vec<usize> = gql_system.topology.nodes
            .iter()
            .map(|n| n.index)
            .collect();

        // Convert terms (term characters)
        let terms: Vec<String> = gql_system.terms
            .iter()
            .map(|t| t.name.clone())
            .collect();

        // Convert connectors (connective characters)
        let connectives: Vec<(String, String, String)> = gql_system.connectors
            .iter()
            .map(|c| (c.name.clone(), c.from.clone(), c.to.clone()))
            .collect();

        SystemData {
            system_name: gql_system.name,
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

/// Helper function to capitalize the first letter
fn capitalize_first(s: &str) -> String {
    let mut chars = s.chars();
    match chars.next() {
        None => String::new(),
        Some(first) => first.to_uppercase().collect::<String>() + chars.as_str(),
    }
}
