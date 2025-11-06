use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub struct SystemConfig {
    pub name: String,
    pub display_name: String,
    pub node_count: usize,
    pub k_notation: String,
    pub description: String,
    pub color_scheme: ColorScheme,
    pub geometry: Option<Geometry>,
}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub struct ColorScheme {
    pub nodes: String,
    pub edges: String,
    pub selected_node: String,
    pub selected_edge: String,
}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub struct Geometry {
    pub indexes: Vec<usize>,
    pub coordinates: Vec<Coordinate>,
    pub edges: Vec<[usize; 2]>,
}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub struct Coordinate {
    pub x: f64,
    pub y: f64,
    pub z: Option<f64>,
}

impl SystemConfig {
    pub fn get_all_systems() -> Vec<SystemConfig> {
        let config_files = [
            ("monad", include_str!("../../configs/monad.json")),
            ("dyad", include_str!("../../configs/dyad.json")),
            ("triad", include_str!("../../configs/triad.json")),
            ("tetrad", include_str!("../../configs/tetrad.json")),
            ("pentad", include_str!("../../configs/pentad.json")),
            ("hexad", include_str!("../../configs/hexad.json")),
            ("heptad", include_str!("../../configs/heptad.json")),
            ("octad", include_str!("../../configs/octad.json")),
            ("ennead", include_str!("../../configs/ennead.json")),
            ("decad", include_str!("../../configs/decad.json")),
            ("undecad", include_str!("../../configs/undecad.json")),
            ("dodecad", include_str!("../../configs/dodecad.json")),
        ];

        config_files
            .iter()
            .filter_map(|(name, json_str)| {
                serde_json::from_str::<SystemConfig>(json_str)
                    .map_err(|e| {
                        web_sys::console::error_1(&format!("Failed to parse {} config: {}", name, e).into());
                        e
                    })
                    .ok()
            })
            .collect()
    }

    pub fn get_by_name(name: &str) -> Option<SystemConfig> {
        Self::get_all_systems()
            .into_iter()
            .find(|s| s.name == name)
    }
}
