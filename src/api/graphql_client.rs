use gloo_net::http::Request;
use serde::{Deserialize, Serialize};
use crate::api::models::{SystemView, ApiError, Coordinate};

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

/// System query response (for system(order:) query)
#[derive(Deserialize, Debug)]
struct SystemQueryResponse {
    system: Option<SystemView>,
}

/// System by name query response
#[derive(Deserialize, Debug)]
struct SystemByNameQueryResponse {
    #[serde(rename = "systemByName")]
    system_by_name: Option<SystemView>,
}

/// All systems query response
#[derive(Deserialize, Debug)]
struct AllSystemsQueryResponse {
    #[serde(rename = "allSystems")]
    all_systems: Vec<SystemView>,
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

    /// GraphQL fragment for system fields (reduces duplication)
    const SYSTEM_FIELDS: &'static str = r#"
        order
        name
        coherence
        termDesignation
        connectiveDesignation
        terms {
            id
            order
            position
            characterId
            character {
                id
                language
                value
            }
        }
        coordinates {
            id
            order
            position
            x
            y
            z
        }
        colours {
            id
            order
            position
            language
            value
        }
        lines {
            id
            baseId
            targetId
            linkType
            characterId
            tag
            order
            basePosition
            targetPosition
            baseCoordinate {
                id
                order
                position
                x
                y
                z
            }
            targetCoordinate {
                id
                order
                position
                x
                y
                z
            }
        }
        connectives {
            id
            baseId
            targetId
            linkType
            characterId
            tag
            order
            basePosition
            targetPosition
            character {
                id
                language
                value
            }
            baseCoordinate {
                id
                order
                position
                x
                y
                z
            }
            targetCoordinate {
                id
                order
                position
                x
                y
                z
            }
        }
    "#;

    /// Fetch a single system by order (1-12)
    pub async fn fetch_system_by_order(&self, order: i32) -> Result<SystemView, ApiError> {
        let query = format!(r#"
            query GetSystem($order: Int!) {{
                system(order: $order) {{
                    {}
                }}
            }}
        "#, Self::SYSTEM_FIELDS);

        let variables = serde_json::json!({
            "order": order
        });

        let response: GraphQLResponse<SystemQueryResponse> =
            self.execute_query(&query, Some(variables)).await?;

        if let Some(errors) = response.errors {
            return Err(ApiError::ParseError(
                errors.iter().map(|e| e.message.clone()).collect::<Vec<_>>().join(", ")
            ));
        }

        let data = response.data
            .ok_or_else(|| ApiError::NotFound(format!("System with order {} not found", order)))?;

        let system = data.system
            .ok_or_else(|| ApiError::NotFound(format!("System with order {} not found", order)))?;

        Ok(self.transform_coordinates(system))
    }

    /// Fetch a single system by name (uses systemByName API query)
    pub async fn fetch_system(&self, system_name: &str) -> Result<SystemView, ApiError> {
        let query = format!(r#"
            query GetSystemByName($name: String!) {{
                systemByName(name: $name) {{
                    {}
                }}
            }}
        "#, Self::SYSTEM_FIELDS);

        let variables = serde_json::json!({
            "name": system_name
        });

        let response: GraphQLResponse<SystemByNameQueryResponse> =
            self.execute_query(&query, Some(variables)).await?;

        if let Some(errors) = response.errors {
            return Err(ApiError::ParseError(
                errors.iter().map(|e| e.message.clone()).collect::<Vec<_>>().join(", ")
            ));
        }

        let data = response.data
            .ok_or_else(|| ApiError::NotFound(format!("System '{}' not found", system_name)))?;

        let system = data.system_by_name
            .ok_or_else(|| ApiError::NotFound(format!("System '{}' not found", system_name)))?;

        Ok(self.transform_coordinates(system))
    }

    /// Fetch all available systems (orders 1-12)
    pub async fn fetch_all_systems(&self) -> Result<Vec<SystemView>, ApiError> {
        let query = format!(r#"
            query GetAllSystems {{
                allSystems {{
                    {}
                }}
            }}
        "#, Self::SYSTEM_FIELDS);

        let response: GraphQLResponse<AllSystemsQueryResponse> =
            self.execute_query(&query, None).await?;

        if let Some(errors) = response.errors {
            return Err(ApiError::ParseError(
                errors.iter().map(|e| e.message.clone()).collect::<Vec<_>>().join(", ")
            ));
        }

        let data = response.data
            .ok_or_else(|| ApiError::NotFound("No systems found".to_string()))?;

        web_sys::console::log_1(&format!("Fetched {} systems from allSystems query", data.all_systems.len()).into());

        let systems: Vec<SystemView> = data.all_systems.into_iter()
            .map(|sys| {
                let transformed = self.transform_coordinates(sys);
                web_sys::console::log_1(&format!("Loaded system: {} (order {})", transformed.display_name(), transformed.order).into());
                transformed
            })
            .collect();

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

    /// Transform coordinates from API space to viewport space (800x800 with margins)
    fn transform_coordinates(&self, mut system: SystemView) -> SystemView {
        let viewport_width = 800.0;
        let viewport_height = 800.0;
        let margin = 100.0;

        // Transform main coordinates array only
        // Links will look up coordinates by position from this array
        system.coordinates = transform_coordinates_to_viewport(
            system.coordinates,
            viewport_width,
            viewport_height,
            margin,
        );

        system
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
        let mut coord = coords.into_iter().next().unwrap();
        coord.x = viewport_width / 2.0;
        coord.y = viewport_height / 2.0;
        return vec![coord];
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
    let center_x = (min_x + max_x) / 2.0;
    let center_y = (min_y + max_y) / 2.0;

    let extent_x = (max_x - min_x).max(0.0001);
    let extent_y = (max_y - min_y).max(0.0001);

    // Use the larger extent for both axes to preserve aspect ratio
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
        .map(|mut coord| {
            coord.x = (coord.x - center_x) * scale + viewport_center_x;
            coord.y = -(coord.y - center_y) * scale + viewport_center_y;  // Negate Y for SVG
            coord
        })
        .collect()
}
