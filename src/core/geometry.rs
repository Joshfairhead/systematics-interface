use std::f64::consts::PI;

#[derive(Debug, Clone, PartialEq)]
pub struct Point {
    pub x: f64,
    pub y: f64,
}

#[derive(Debug, Clone, PartialEq)]
pub struct Edge {
    pub from: usize,
    pub to: usize,
}

#[derive(Debug, Clone)]
pub struct GraphLayout {
    pub nodes: Vec<Point>,
    pub edges: Vec<Edge>,
    pub node_radius: f64,
    pub symbolic_circle: Option<SymbolicCircle>,
    pub symbolic_circles: Vec<SymbolicCircle>,
}

#[derive(Debug, Clone)]
pub struct SymbolicCircle {
    pub center: Point,
    pub radius: f64,
}

pub struct GeometryCalculator;

impl GeometryCalculator {
    pub fn calculate_system_layout(
        system_type: &str,
        center_x: f64,
        center_y: f64,
        size: f64,
    ) -> GraphLayout {
        let node_count = Self::get_node_count(system_type);
        let nodes = Self::calculate_node_positions(node_count, center_x, center_y, size);
        let edges = Self::generate_complete_graph_edges(node_count);
        let node_radius = 12.0; // Fixed node radius
        let symbolic_circle = Self::get_symbolic_circle(system_type, center_x, center_y, size);
        let symbolic_circles = Self::get_symbolic_circles(system_type, center_x, center_y, size);

        GraphLayout {
            nodes,
            edges,
            node_radius,
            symbolic_circle,
            symbolic_circles,
        }
    }

    fn get_node_count(system_type: &str) -> usize {
        match system_type {
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
            "undecad" => 11,
            "dodecad" => 12,
            _ => 1,
        }
    }

    fn calculate_node_positions(
        node_count: usize,
        cx: f64,
        cy: f64,
        size: f64,
    ) -> Vec<Point> {
        match node_count {
            1 => vec![Point { x: cx, y: cy }],
            2 => {
                let spacing = size * 0.18;
                vec![
                    Point { x: cx - spacing, y: cy },
                    Point { x: cx + spacing, y: cy },
                ]
            }
            3 => {
                // Equilateral triangle
                let side_length = size * 0.70;
                let height = side_length * (3.0_f64.sqrt() / 2.0);
                let half_side = side_length / 2.0;
                let right_offset = height / 2.0 + size * 0.05;

                vec![
                    Point { x: cx - height / 2.0, y: cy - half_side },
                    Point { x: cx + right_offset, y: cy },
                    Point { x: cx - height / 2.0, y: cy + half_side },
                ]
            }
            4 => {
                // Diamond shape for tetrad
                let diamond_size = size * 0.38;

                vec![
                    Point { x: cx, y: cy - diamond_size },
                    Point { x: cx + diamond_size, y: cy },
                    Point { x: cx - diamond_size, y: cy },
                    Point { x: cx, y: cy + diamond_size },
                ]
            }
            _ => {
                // Regular polygon for higher-order systems
                let radius = match node_count {
                    5 | 6 => size * 0.38,
                    _ => size * 0.38,
                };
                let rotation = -PI / 2.0; // Start at top

                (0..node_count)
                    .map(|i| {
                        let angle = 2.0 * PI * i as f64 / node_count as f64 + rotation;
                        Point {
                            x: cx + radius * angle.cos(),
                            y: cy + radius * angle.sin(),
                        }
                    })
                    .collect()
            }
        }
    }

    fn generate_complete_graph_edges(node_count: usize) -> Vec<Edge> {
        let mut edges = Vec::new();

        for i in 0..node_count {
            for j in (i + 1)..node_count {
                edges.push(Edge { from: i, to: j });
            }
        }

        edges
    }

    fn get_symbolic_circle(system_type: &str, center_x: f64, center_y: f64, size: f64) -> Option<SymbolicCircle> {
        match system_type {
            "monad" => {
                Some(SymbolicCircle {
                    center: Point { x: center_x, y: center_y },
                    radius: size * 0.45,
                })
            },
            _ => None,
        }
    }

    fn get_symbolic_circles(system_type: &str, center_x: f64, center_y: f64, size: f64) -> Vec<SymbolicCircle> {
        match system_type {
            "dyad" => {
                let spacing = size * 0.18;
                let radius = size * 0.36;
                vec![
                    SymbolicCircle {
                        center: Point { x: center_x - spacing, y: center_y },
                        radius,
                    },
                    SymbolicCircle {
                        center: Point { x: center_x + spacing, y: center_y },
                        radius,
                    },
                ]
            },
            _ => vec![],
        }
    }
}
