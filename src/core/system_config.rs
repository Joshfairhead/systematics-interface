use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub struct SystemConfig {
    pub name: String,
    pub display_name: String,
    pub node_count: usize,
    pub k_notation: String,
    pub description: String,
    pub color_scheme: ColorScheme,
}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub struct ColorScheme {
    pub nodes: String,
    pub edges: String,
    pub selected_node: String,
    pub selected_edge: String,
}

impl SystemConfig {
    pub fn get_all_systems() -> Vec<SystemConfig> {
        vec![
            SystemConfig {
                name: "monad".to_string(),
                display_name: "Monad".to_string(),
                node_count: 1,
                k_notation: "K1".to_string(),
                description: "".to_string(),
                color_scheme: ColorScheme {
                    nodes: "#1ABC9C".to_string(),
                    edges: "#888888".to_string(),
                    selected_node: "#FF6B6B".to_string(),
                    selected_edge: "#FF6B6B".to_string(),
                },
            },
            SystemConfig {
                name: "dyad".to_string(),
                display_name: "Dyad".to_string(),
                node_count: 2,
                k_notation: "K2".to_string(),
                description: "".to_string(),
                color_scheme: ColorScheme {
                    nodes: "#1ABC9C".to_string(),
                    edges: "#888888".to_string(),
                    selected_node: "#FF6B6B".to_string(),
                    selected_edge: "#FF6B6B".to_string(),
                },
            },
            SystemConfig {
                name: "triad".to_string(),
                display_name: "Triad".to_string(),
                node_count: 3,
                k_notation: "K3".to_string(),
                description: "".to_string(),
                color_scheme: ColorScheme {
                    nodes: "#1ABC9C".to_string(),
                    edges: "#888888".to_string(),
                    selected_node: "#FF6B6B".to_string(),
                    selected_edge: "#FF6B6B".to_string(),
                },
            },
            SystemConfig {
                name: "tetrad".to_string(),
                display_name: "Tetrad".to_string(),
                node_count: 4,
                k_notation: "K4".to_string(),
                description: "".to_string(),
                color_scheme: ColorScheme {
                    nodes: "#1ABC9C".to_string(),
                    edges: "#888888".to_string(),
                    selected_node: "#FF6B6B".to_string(),
                    selected_edge: "#FF6B6B".to_string(),
                },
            },
            SystemConfig {
                name: "pentad".to_string(),
                display_name: "Pentad".to_string(),
                node_count: 5,
                k_notation: "K5".to_string(),
                description: "".to_string(),
                color_scheme: ColorScheme {
                    nodes: "#1ABC9C".to_string(),
                    edges: "#888888".to_string(),
                    selected_node: "#FF6B6B".to_string(),
                    selected_edge: "#FF6B6B".to_string(),
                },
            },
            SystemConfig {
                name: "hexad".to_string(),
                display_name: "Hexad".to_string(),
                node_count: 6,
                k_notation: "K6".to_string(),
                description: "".to_string(),
                color_scheme: ColorScheme {
                    nodes: "#1ABC9C".to_string(),
                    edges: "#888888".to_string(),
                    selected_node: "#FF6B6B".to_string(),
                    selected_edge: "#FF6B6B".to_string(),
                },
            },
            SystemConfig {
                name: "heptad".to_string(),
                display_name: "Heptad".to_string(),
                node_count: 7,
                k_notation: "K7".to_string(),
                description: "".to_string(),
                color_scheme: ColorScheme {
                    nodes: "#1ABC9C".to_string(),
                    edges: "#888888".to_string(),
                    selected_node: "#FF6B6B".to_string(),
                    selected_edge: "#FF6B6B".to_string(),
                },
            },
            SystemConfig {
                name: "octad".to_string(),
                display_name: "Octad".to_string(),
                node_count: 8,
                k_notation: "K8".to_string(),
                description: "".to_string(),
                color_scheme: ColorScheme {
                    nodes: "#1ABC9C".to_string(),
                    edges: "#888888".to_string(),
                    selected_node: "#FF6B6B".to_string(),
                    selected_edge: "#FF6B6B".to_string(),
                },
            },
            SystemConfig {
                name: "ennead".to_string(),
                display_name: "Ennead".to_string(),
                node_count: 9,
                k_notation: "K9".to_string(),
                description: "".to_string(),
                color_scheme: ColorScheme {
                    nodes: "#1ABC9C".to_string(),
                    edges: "#888888".to_string(),
                    selected_node: "#FF6B6B".to_string(),
                    selected_edge: "#FF6B6B".to_string(),
                },
            },
            SystemConfig {
                name: "decad".to_string(),
                display_name: "Decad".to_string(),
                node_count: 10,
                k_notation: "K10".to_string(),
                description: "".to_string(),
                color_scheme: ColorScheme {
                    nodes: "#1ABC9C".to_string(),
                    edges: "#888888".to_string(),
                    selected_node: "#FF6B6B".to_string(),
                    selected_edge: "#FF6B6B".to_string(),
                },
            },
            SystemConfig {
                name: "undecad".to_string(),
                display_name: "Undecad".to_string(),
                node_count: 11,
                k_notation: "K11".to_string(),
                description: "".to_string(),
                color_scheme: ColorScheme {
                    nodes: "#1ABC9C".to_string(),
                    edges: "#888888".to_string(),
                    selected_node: "#FF6B6B".to_string(),
                    selected_edge: "#FF6B6B".to_string(),
                },
            },
            SystemConfig {
                name: "dodecad".to_string(),
                display_name: "Dodecad".to_string(),
                node_count: 12,
                k_notation: "K12".to_string(),
                description: "".to_string(),
                color_scheme: ColorScheme {
                    nodes: "#1ABC9C".to_string(),
                    edges: "#888888".to_string(),
                    selected_node: "#FF6B6B".to_string(),
                    selected_edge: "#FF6B6B".to_string(),
                },
            },
        ]
    }

    pub fn get_by_name(name: &str) -> Option<SystemConfig> {
        Self::get_all_systems()
            .into_iter()
            .find(|s| s.name == name)
    }
}
