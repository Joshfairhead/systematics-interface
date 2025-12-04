use yew::prelude::*;
use crate::api::models::{SystemData, Coordinate, TopologyEdge};

#[derive(Properties, PartialEq)]
pub struct ApiGraphViewProps {
    pub system: SystemData,
}

pub enum ApiGraphMsg {
    NodeClicked(usize),
    EdgeClicked(usize, usize),
}

pub struct ApiGraphView {
    selected_node: Option<usize>,
    selected_edge: Option<(usize, usize)>,
}

impl Component for ApiGraphView {
    type Message = ApiGraphMsg;
    type Properties = ApiGraphViewProps;

    fn create(_ctx: &Context<Self>) -> Self {
        Self {
            selected_node: None,
            selected_edge: None,
        }
    }

    fn update(&mut self, _ctx: &Context<Self>, msg: Self::Message) -> bool {
        match msg {
            ApiGraphMsg::NodeClicked(idx) => {
                if self.selected_node == Some(idx) {
                    self.selected_node = None;
                } else {
                    self.selected_node = Some(idx);
                    self.selected_edge = None;
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
        }
    }

    fn view(&self, ctx: &Context<Self>) -> Html {
        let system = &ctx.props().system;

        html! {
            <div class="graph-view">
                <svg
                    class="graph-svg"
                    viewBox="0 0 800 800"
                    preserveAspectRatio="xMidYMid meet"
                >
                    { self.render_edges(&system.edges, &system.coordinates, system) }
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
                        { idx }
                    </text>
                </g>
            }
        }).collect::<Html>()
    }
}
