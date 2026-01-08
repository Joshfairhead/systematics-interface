use serde::{Deserialize, Serialize};

/// Language enum matching GqlLanguage from backend
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq)]
#[serde(rename_all = "SCREAMING_SNAKE_CASE")]
pub enum Language {
    Canonical,
    Energy,
    Values,
    Society,
    Hex,
    Name,
}

/// Link type enum matching GqlLinkType from backend
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq)]
#[serde(rename_all = "SCREAMING_SNAKE_CASE")]
pub enum LinkType {
    Line,
    Connective,
}

/// Character matching GqlCharacter from backend
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub struct Character {
    pub id: String,
    pub language: Language,
    pub value: String,
}

/// Term matching GqlTerm from backend
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub struct Term {
    pub id: String,
    pub order: i32,
    pub position: i32,
    #[serde(rename = "characterId")]
    pub character_id: String,
    pub character: Option<Character>,
}

/// Coordinate matching GqlCoordinate from backend
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub struct Coordinate {
    pub id: String,
    pub order: i32,
    pub position: i32,
    pub x: f64,
    pub y: f64,
    pub z: f64,
}

/// Colour matching GqlColour from backend
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub struct Colour {
    pub id: String,
    pub order: i32,
    pub position: i32,
    pub language: Language,
    pub value: String,
}

/// Link matching GqlLink from backend (used for both lines and connectives)
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub struct Link {
    pub id: String,
    #[serde(rename = "baseId")]
    pub base_id: String,
    #[serde(rename = "targetId")]
    pub target_id: String,
    #[serde(rename = "linkType")]
    pub link_type: LinkType,
    #[serde(rename = "characterId")]
    pub character_id: Option<String>,
    pub tag: Option<String>,
    pub order: Option<i32>,
    #[serde(rename = "basePosition")]
    pub base_position: Option<i32>,
    #[serde(rename = "targetPosition")]
    pub target_position: Option<i32>,
    /// Resolved character for this link (for connectives)
    pub character: Option<Character>,
    /// Resolved base coordinate
    #[serde(rename = "baseCoordinate")]
    pub base_coordinate: Option<Coordinate>,
    /// Resolved target coordinate
    #[serde(rename = "targetCoordinate")]
    pub target_coordinate: Option<Coordinate>,
}

/// System view matching GqlSystemView from backend
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub struct SystemView {
    pub order: i32,
    pub name: Option<String>,
    pub coherence: Option<String>,
    #[serde(rename = "termDesignation")]
    pub term_designation: Option<String>,
    #[serde(rename = "connectiveDesignation")]
    pub connective_designation: Option<String>,
    pub terms: Vec<Term>,
    pub coordinates: Vec<Coordinate>,
    pub colours: Vec<Colour>,
    pub connectives: Vec<Link>,
    pub lines: Vec<Link>,
    /// All links (both lines and connectives)
    #[serde(default)]
    pub links: Vec<Link>,
}

impl SystemView {
    /// Get the system name, falling back to order-based name
    pub fn display_name(&self) -> String {
        self.name.clone().unwrap_or_else(|| {
            match self.order {
                1 => "Monad",
                2 => "Dyad",
                3 => "Triad",
                4 => "Tetrad",
                5 => "Pentad",
                6 => "Hexad",
                7 => "Heptad",
                8 => "Octad",
                9 => "Ennead",
                10 => "Decad",
                11 => "Undecad",
                12 => "Dodecad",
                _ => "Unknown",
            }.to_string()
        })
    }

    /// Get the K-notation for this system
    pub fn k_notation(&self) -> String {
        format!("K{}", self.order)
    }

    /// Get the description/coherence for this system
    pub fn description(&self) -> String {
        self.coherence.clone().unwrap_or_else(|| self.display_name())
    }

    /// Get the number of nodes in this system
    pub fn node_count(&self) -> usize {
        self.order as usize
    }

    /// Get the term value at a position (1-based)
    pub fn term_at(&self, position: i32) -> Option<&str> {
        self.terms.iter()
            .find(|t| t.position == position)
            .and_then(|t| t.character.as_ref())
            .map(|c| c.value.as_str())
    }

    /// Get the colour value at a position (1-based)
    pub fn colour_at(&self, position: i32) -> Option<&str> {
        self.colours.iter()
            .find(|c| c.position == position)
            .map(|c| c.value.as_str())
    }

    /// Get the coordinate at a position (1-based)
    pub fn coordinate_at(&self, position: i32) -> Option<&Coordinate> {
        self.coordinates.iter()
            .find(|c| c.position == position)
    }
}

/// Slice matching GqlSlice from backend (all entries at order+position)
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub struct Slice {
    pub order: i32,
    pub position: i32,
    pub term: Option<Term>,
    pub coordinate: Option<Coordinate>,
    pub colour: Option<Colour>,
}

/// API error type
#[derive(Debug)]
pub enum ApiError {
    NetworkError(String),
    ParseError(String),
    NotFound(String),
}

impl std::fmt::Display for ApiError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            ApiError::NetworkError(msg) => write!(f, "Network error: {}", msg),
            ApiError::ParseError(msg) => write!(f, "Parse error: {}", msg),
            ApiError::NotFound(msg) => write!(f, "Not found: {}", msg),
        }
    }
}

impl std::error::Error for ApiError {}
