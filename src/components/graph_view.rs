use yew::prelude::*;
use crate::core::geometry::{GeometryCalculator, GraphLayout};
use crate::core::system_config::SystemConfig;

#[derive(Properties, PartialEq)]
pub struct GraphViewProps {
    pub system: SystemConfig,
}

pub enum GraphMsg {
    NodeClicked(usize),
    EdgeClicked(usize, usize),
}

pub struct GraphView {
    selected_node: Option<usize>,
    selected_edge: Option<(usize, usize)>,
}

impl Component for GraphView {
    type Message = GraphMsg;
    type Properties = GraphViewProps;

    fn create(_ctx: &Context<Self>) -> Self {
        Self {
            selected_node: None,
            selected_edge: None,
        }
    }

    fn update(&mut self, _ctx: &Context<Self>, msg: Self::Message) -> bool {
        match msg {
            GraphMsg::NodeClicked(idx) => {
                if self.selected_node == Some(idx) {
                    self.selected_node = None;
                } else {
                    self.selected_node = Some(idx);
                    self.selected_edge = None;
                }
                true
            }
            GraphMsg::EdgeClicked(from, to) => {
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
        let layout = GeometryCalculator::calculate_system_layout(
            &system.name,
            800.0,
            800.0,
            1400.0,
        );

        html! {
            <div class="graph-view">
                {
                    if let Some(node_idx) = self.selected_node {
                        html! {
                            <div class="selection-info">
                                <strong>{ "Selected Node: " }</strong>
                                <span>{ format!("Node {}", node_idx) }</span>
                            </div>
                        }
                    } else if let Some((from, to)) = self.selected_edge {
                        html! {
                            <div class="selection-info">
                                <strong>{ "Selected Edge: " }</strong>
                                <span>{ format!("Node {} ↔ Node {}", from, to) }</span>
                            </div>
                        }
                    } else {
                        html! {}
                    }
                }
                <svg
                    class="graph-svg"
                    viewBox="0 0 1600 1600"
                >
                    { self.render_edges(&layout, system) }
                    { self.render_symbolic_circles(&layout, system) }
                    { self.render_nodes(ctx, &layout, system) }
                </svg>
            </div>
        }
    }
}

impl GraphView {
    fn render_edges(&self, layout: &GraphLayout, system: &SystemConfig) -> Html {
        layout.edges.iter().map(|edge| {
            let from_node = &layout.nodes[edge.from];
            let to_node = &layout.nodes[edge.to];

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
                    x1={ from_node.x.to_string() }
                    y1={ from_node.y.to_string() }
                    x2={ to_node.x.to_string() }
                    y2={ to_node.y.to_string() }
                    stroke={ stroke.clone() }
                    stroke-width={ stroke_width.to_string() }
                    class="edge"
                />
            }
        }).collect::<Html>()
    }

    fn render_symbolic_circles(&self, layout: &GraphLayout, system: &SystemConfig) -> Html {
        let mut circles = Vec::new();

        // Render main symbolic circle (for monad)
        if let Some(ref circle) = layout.symbolic_circle {
            circles.push(html! {
                <circle
                    cx={ circle.center.x.to_string() }
                    cy={ circle.center.y.to_string() }
                    r={ circle.radius.to_string() }
                    fill="none"
                    stroke={ system.color_scheme.nodes.clone() }
                    stroke-width="2"
                    class="symbolic-circle"
                />
            });
        }

        // Render multiple symbolic circles (for dyad, etc.)
        for circle in &layout.symbolic_circles {
            circles.push(html! {
                <circle
                    cx={ circle.center.x.to_string() }
                    cy={ circle.center.y.to_string() }
                    r={ circle.radius.to_string() }
                    fill="none"
                    stroke={ system.color_scheme.nodes.clone() }
                    stroke-width="2"
                    class="symbolic-circle"
                />
            });
        }

        html! { <>{ for circles }</> }
    }

    fn render_nodes(&self, ctx: &Context<Self>, layout: &GraphLayout, system: &SystemConfig) -> Html {
        layout.nodes.iter().enumerate().map(|(idx, node)| {
            let is_selected = self.selected_node == Some(idx);
            let fill = if is_selected {
                &system.color_scheme.selected_node
            } else {
                &system.color_scheme.nodes
            };
            let radius = if is_selected {
                layout.node_radius * 1.5
            } else {
                layout.node_radius
            };

            let onclick = ctx.link().callback(move |_| GraphMsg::NodeClicked(idx));

            html! {
                <g class="node" onclick={ onclick }>
                    <circle
                        cx={ node.x.to_string() }
                        cy={ node.y.to_string() }
                        r={ radius.to_string() }
                        fill={ fill.clone() }
                        stroke="white"
                        stroke-width="2"
                        style="cursor: pointer;"
                    />
                    <text
                        x={ node.x.to_string() }
                        y={ node.y.to_string() }
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
