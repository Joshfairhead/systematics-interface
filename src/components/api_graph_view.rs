use yew::prelude::*;
use crate::api::models::{SystemData, Coordinate, TopologyEdge};

#[derive(Properties, PartialEq)]
pub struct ApiGraphViewProps {
    pub system: SystemData,
    #[prop_or_default]
    pub on_navigate: Option<Callback<String>>,
}

pub enum ApiGraphMsg {
    NodeClicked(usize),
    EdgeClicked(usize, usize),
    ToggleEdgeLabels,
}

pub struct ApiGraphView {
    selected_node: Option<usize>,
    selected_edge: Option<(usize, usize)>,
    show_edge_labels: bool,
}

impl Component for ApiGraphView {
    type Message = ApiGraphMsg;
    type Properties = ApiGraphViewProps;

    fn create(_ctx: &Context<Self>) -> Self {
        Self {
            selected_node: None,
            selected_edge: None,
            show_edge_labels: false,
        }
    }

    fn update(&mut self, ctx: &Context<Self>, msg: Self::Message) -> bool {
        match msg {
            ApiGraphMsg::NodeClicked(idx) => {
                // Check if this node has a navigation target
                let system = &ctx.props().system;
                if let Some(nav_edge) = system.navigation_edges.iter().find(|e| e.node == idx) {
                    // Navigate to the target system
                    if let Some(ref callback) = ctx.props().on_navigate {
                        callback.emit(nav_edge.target_system.clone());
                    }
                } else {
                    // No navigation target, just select/deselect
                    if self.selected_node == Some(idx) {
                        self.selected_node = None;
                    } else {
                        self.selected_node = Some(idx);
                        self.selected_edge = None;
                    }
                }
                true
            }
            ApiGraphMsg::EdgeClicked(from, to) => {
                let edge = if from < to { (from, to) } else { (to, from) };
                if self.selected_edge == Some(edge) {
                    self.selected_edge = None;
                } else {
                    self.selected_edge = Some(edge);
                    self.selected_node = None;
                }
                true
            }
            ApiGraphMsg::ToggleEdgeLabels => {
                self.show_edge_labels = !self.show_edge_labels;
                true
            }
        }
    }

    fn view(&self, ctx: &Context<Self>) -> Html {
        let system = &ctx.props().system;
        let on_toggle = ctx.link().callback(|_| ApiGraphMsg::ToggleEdgeLabels);

        html! {
            <div class="graph-view">
                <div class="graph-controls">
                    <label class="control-toggle">
                        <input
                            type="checkbox"
                            checked={self.show_edge_labels}
                            onclick={on_toggle}
                        />
                        <span>{"Show Edge Labels"}</span>
                    </label>
                </div>

                <svg
                    class="graph-svg"
                    viewBox="0 0 800 800"
                    preserveAspectRatio="xMidYMid meet"
                >
                    { self.render_edges(&system.edges, &system.coordinates, system) }
                    if self.show_edge_labels {
                        { self.render_edge_labels(&system.edges, &system.coordinates, &system.terms, &system.connectives) }
                    }
                    { self.render_nodes(ctx, &system.coordinates, system) }
                </svg>
            </div>
        }
    }
}

impl ApiGraphView {
    // No scaling needed - use coordinates directly
    fn scale_x(x: f64) -> f64 {
        x
    }

    fn scale_y(y: f64) -> f64 {
        y
    }

    fn render_edges(
        &self,
        edges: &[TopologyEdge],
        coordinates: &[Coordinate],
        system: &SystemData,
    ) -> Html {
        edges.iter().map(|edge| {
            // Safely get coordinates with bounds checking
            if edge.from >= coordinates.len() || edge.to >= coordinates.len() {
                return html! {};
            }

            let from_node = &coordinates[edge.from];
            let to_node = &coordinates[edge.to];

            let edge_tuple = if edge.from < edge.to {
                (edge.from, edge.to)
            } else {
                (edge.to, edge.from)
            };

            let is_selected = self.selected_edge == Some(edge_tuple);
            let stroke = if is_selected {
                &system.color_scheme.selected_edge
            } else {
                &system.color_scheme.edges
            };
            let stroke_width = if is_selected { 3.0 } else { 1.5 };

            html! {
                <line
                    x1={ Self::scale_x(from_node.x).to_string() }
                    y1={ Self::scale_y(from_node.y).to_string() }
                    x2={ Self::scale_x(to_node.x).to_string() }
                    y2={ Self::scale_y(to_node.y).to_string() }
                    stroke={ stroke.clone() }
                    stroke-width={ stroke_width.to_string() }
                    class="edge"
                />
            }
        }).collect::<Html>()
    }

    fn render_edge_labels(
        &self,
        edges: &[TopologyEdge],
        coordinates: &[Coordinate],
        terms: &[String],
        connectives: &[(String, String, String)],
    ) -> Html {
        edges.iter().enumerate().map(|(edge_idx, edge)| {
            // Safely get coordinates with bounds checking
            if edge.from >= coordinates.len() || edge.to >= coordinates.len() {
                return html! {};
            }

            let from_node = &coordinates[edge.from];
            let to_node = &coordinates[edge.to];

            // Calculate midpoint of the edge for label placement
            let mid_x = (from_node.x + to_node.x) / 2.0;
            let mut mid_y = (from_node.y + to_node.y) / 2.0;

            // Calculate angle of the edge for label rotation
            let dx = to_node.x - from_node.x;
            let dy = to_node.y - from_node.y;
            let angle = dy.atan2(dx) * 180.0 / std::f64::consts::PI;

            // Adjust angle to keep text readable (not upside down)
            // If angle is between 90 and 270 degrees (pointing left), flip by 180
            let rotation_angle = if angle > 90.0 || angle < -90.0 {
                angle + 180.0
            } else {
                angle
            };

            // Get the term names for this edge's from and to nodes
            let from_term = terms.get(edge.from).map(|s| s.as_str()).unwrap_or("");
            let to_term = terms.get(edge.to).map(|s| s.as_str()).unwrap_or("");

            // Find the connective that matches this edge
            // connectives tuple is (name, from_term, to_term)
            let label = connectives.iter()
                .find(|(_, conn_from, conn_to)| {
                    (conn_from == from_term && conn_to == to_term) ||
                    (conn_from == to_term && conn_to == from_term) // Check both directions
                })
                .map(|(name, _, _)| name.as_str())
                .unwrap_or("");

            // Only render if there's a label
            if label.is_empty() {
                return html! {};
            }

            // Apply offset for crossing edges in tetrad (edges that cross near center)
            // Detect crossing by checking if edges are nearly diagonal and close to center
            let is_diagonal = dx.abs() > 100.0 && dy.abs() > 100.0;
            let near_center = mid_x > 300.0 && mid_x < 500.0 && mid_y > 300.0 && mid_y < 500.0;

            if is_diagonal && near_center {
                // For tetrad crossing edges, offset alternately
                if edge_idx % 2 == 0 {
                    mid_y -= 25.0; // Move first crossing edge up more
                } else {
                    mid_y += 25.0; // Move second crossing edge down
                }
            }

            let mid_x_scaled = Self::scale_x(mid_x);
            let mid_y_scaled = Self::scale_y(mid_y);
            let rect_width = label.len() as f64 * 7.0;
            let rect_height = 16.0;

            html! {
                <g class="edge-label-group" transform={ format!("rotate({} {} {})", rotation_angle, mid_x_scaled, mid_y_scaled) }>
                    // Background rectangle for better readability
                    <rect
                        x={ (mid_x_scaled - rect_width / 2.0).to_string() }
                        y={ (mid_y_scaled - rect_height / 2.0).to_string() }
                        width={ rect_width.to_string() }
                        height={ rect_height.to_string() }
                        fill="rgba(255, 255, 255, 0.9)"
                        stroke="rgba(37, 99, 235, 0.3)"
                        stroke-width="0.5"
                        rx="4"
                        style="pointer-events: none;"
                    />
                    // Label text
                    <text
                        x={ mid_x_scaled.to_string() }
                        y={ mid_y_scaled.to_string() }
                        text-anchor="middle"
                        dominant-baseline="middle"
                        class="edge-label"
                        fill="#2563eb"
                        style="font-size: 10px; font-weight: 500; pointer-events: none; user-select: none;"
                    >
                        { label }
                    </text>
                </g>
            }
        }).collect::<Html>()
    }

    fn render_nodes(
        &self,
        ctx: &Context<Self>,
        coordinates: &[Coordinate],
        system: &SystemData,
    ) -> Html {
        coordinates.iter().enumerate().map(|(idx, coord)| {
            let is_selected = self.selected_node == Some(idx);
            let fill = if is_selected {
                &system.color_scheme.selected_node
            } else {
                &system.color_scheme.nodes
            };
            let radius = if is_selected { 18.0 } else { 12.0 };

            let onclick = ctx.link().callback(move |_| ApiGraphMsg::NodeClicked(idx));

            // Get vocabulary term for this node if available
            let term = system.terms.get(idx).map(|s| s.as_str()).unwrap_or("");

            html! {
                <g class="node" onclick={ onclick }>
                    <circle
                        cx={ Self::scale_x(coord.x).to_string() }
                        cy={ Self::scale_y(coord.y).to_string() }
                        r={ radius.to_string() }
                        fill={ fill.clone() }
                        stroke="white"
                        stroke-width="2"
                        style="cursor: pointer;"
                    />
                    <text
                        x={ Self::scale_x(coord.x).to_string() }
                        y={ Self::scale_y(coord.y).to_string() }
                        text-anchor="middle"
                        dominant-baseline="middle"
                        fill="white"
                        style="font-size: 12px; font-weight: bold; pointer-events: none; user-select: none;"
                    >
                        { idx + 1 }
                    </text>
                    // Render vocabulary label if available
                    if !term.is_empty() {
                        <text
                            x={ Self::scale_x(coord.x).to_string() }
                            y={ (Self::scale_y(coord.y) + radius + 16.0).to_string() }
                            text-anchor="middle"
                            dominant-baseline="middle"
                            fill="#333"
                            style="font-size: 14px; font-weight: 500; pointer-events: none; user-select: none;"
                        >
                            { term }
                        </text>
                    }
                </g>
            }
        }).collect::<Html>()
    }
}
