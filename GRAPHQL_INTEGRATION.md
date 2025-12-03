# GraphQL Integration Guide

This document explains how to use the systematics-interface with the GraphQL API from [systematics-v0.0.3](https://github.com/Joshfairhead/systematics-v0.0.3).

## Overview

The interface now supports both **mock data** (for development) and **real GraphQL API** (for production). The implementation is in `src/api/graphql_client.rs` and provides a seamless way to fetch system topology, geometry, and vocabulary data.

## Quick Start

### Using Mock Data (Default)

By default, the interface uses mock data generated procedurally. This is useful for:
- Development without a running API
- Testing the UI
- Offline work

No configuration needed - just run:

```bash
trunk serve
```

The app will be available at http://localhost:8080

### Using the Real GraphQL API

To switch to the real API:

1. **Deploy or run the systematics-v0.0.3 GraphQL server**

2. **Update the API endpoint in `src/api_app.rs`:**

```rust
// Line 33: Change use_graphql to true
let use_graphql = true; // Changed from false

// Line 36: Update the GraphQL endpoint URL
let graphql_endpoint = "https://your-api-url.com/graphql".to_string();
// Or for local development:
// let graphql_endpoint = "http://localhost:8080/graphql".to_string();
```

3. **Rebuild and run:**

```bash
trunk serve
```

## GraphQL Client Architecture

### Data Flow

```
GraphQL API → GraphQLClient → SystemData → ApiGraphView → SVG Rendering
```

1. **GraphQL API** provides system data via queries
2. **GraphQLClient** executes GraphQL queries and parses responses
3. **SystemData** model holds the complete system information
4. **ApiGraphView** renders the graph using coordinates and edges
5. **SVG** displays the interactive visualization

### Key Components

#### GraphQLClient (`src/api/graphql_client.rs`)

The main client for interacting with the GraphQL API:

```rust
let client = GraphQLClient::new("http://localhost:8080/graphql".to_string());

// Fetch a single system
let system = client.fetch_system("pentad").await?;

// Fetch all systems
let systems = client.fetch_all_systems().await?;
```

#### Queries Supported

**GetSystem Query**
Fetches complete data for a single system including:
- System metadata (name, coherence attributes, designations)
- Topology (nodes and edges)
- Geometry (coordinates and lines)
- Terms (vocabulary elements)
- Connectors (relationships between terms)

**GetAllSystems Query**
Fetches all available systems with the same data structure.

## GraphQL Schema

The interface expects the following GraphQL schema from the API:

### Main Types

```graphql
type System {
  name: String!
  coherenceAttributes: [String!]!
  termDesignation: String
  connectiveDesignation: String
  source: String
  topology: Topology!
  geometry: Geometry!
  terms: [Term!]!
  connectors: [Connector!]!
}

type Topology {
  systemName: String!
  nodes: [Node!]!
  edges: [Edge!]!
}

type Geometry {
  systemName: String!
  coordinates: [Coordinate!]!
  lines: [Line!]!
}

type Coordinate {
  x: Float!
  y: Float!
  z: Float
}

type Node {
  index: Int!
}

type Edge {
  from: Int!
  to: Int!
}

type Term {
  name: String!
  system: String!
  nodeIndex: Int!
  coordinate: Coordinate
}

type Connector {
  name: String!
  from: String!
  to: String!
}
```

### Root Queries

```graphql
type Query {
  system(name: String!): System
  allSystems: [System!]!
}
```

## Data Mapping

The GraphQL client automatically maps the API response to the internal `SystemData` model:

| GraphQL Field | SystemData Field | Description |
|---------------|------------------|-------------|
| `name` | `system_name` | System identifier (monad, dyad, etc.) |
| `topology.nodes` | `indexes` | Node indexes [0, 1, 2, ...] |
| `topology.edges` | `edges` | Edge connections |
| `geometry.coordinates` | `coordinates` | 2D/3D positions |
| `terms` | `terms` | Term names/labels |
| `connectors` | `connectives` | Relationships between terms |

## Coordinate System

The interface uses an **800x800 SVG viewport**:
- Origin: (0, 0) at top-left
- Center: (400, 400)
- Typical range: 100-700 on both axes

The GraphQL API should return coordinates in this coordinate space.

## Error Handling

The client handles several error types:

```rust
pub enum ApiError {
    NetworkError(String),   // Connection issues
    ParseError(String),     // JSON/GraphQL parse errors
    NotFound(String),       // System not found
}
```

Errors are displayed in the UI with helpful messages.

## CORS Configuration

For local development or production, ensure your GraphQL server has CORS configured:

```
Access-Control-Allow-Origin: *
Access-Control-Allow-Methods: POST, OPTIONS
Access-Control-Allow-Headers: Content-Type
```

## Development Workflow

### 1. Development with Mock Data

```bash
# Default configuration - no API needed
trunk serve
```

### 2. Testing with Local API

```bash
# Terminal 1: Run the GraphQL API
cd /path/to/systematics-v0.0.3
cargo run

# Terminal 2: Update api_app.rs to use GraphQL
# Then run the interface
trunk serve
```

### 3. Production Build

```bash
# Update api_app.rs with production API URL
trunk build --release
```

The built files will be in `dist/` directory.

## Example GraphQL Responses

### System Query Response

```json
{
  "data": {
    "system": {
      "name": "pentad",
      "coherenceAttributes": ["five-fold", "pentagon"],
      "termDesignation": "K5",
      "connectiveDesignation": null,
      "source": "systematics",
      "topology": {
        "systemName": "pentad",
        "nodes": [
          {"index": 0},
          {"index": 1},
          {"index": 2},
          {"index": 3},
          {"index": 4}
        ],
        "edges": [
          {"from": 0, "to": 1},
          {"from": 0, "to": 2},
          {"from": 0, "to": 3},
          {"from": 0, "to": 4},
          {"from": 1, "to": 2},
          {"from": 1, "to": 3},
          {"from": 1, "to": 4},
          {"from": 2, "to": 3},
          {"from": 2, "to": 4},
          {"from": 3, "to": 4}
        ]
      },
      "geometry": {
        "systemName": "pentad",
        "coordinates": [
          {"x": 400.0, "y": 120.0, "z": null},
          {"x": 632.36, "y": 254.1, "z": null},
          {"x": 540.7, "y": 519.1, "z": null},
          {"x": 259.3, "y": 519.1, "z": null},
          {"x": 167.64, "y": 254.1, "z": null}
        ],
        "lines": []
      },
      "terms": [
        {"name": "Purpose", "system": "pentad", "nodeIndex": 0, "coordinate": null},
        {"name": "Higher Potential", "system": "pentad", "nodeIndex": 1, "coordinate": null},
        {"name": "Quintessence", "system": "pentad", "nodeIndex": 2, "coordinate": null},
        {"name": "Lower Potential", "system": "pentad", "nodeIndex": 3, "coordinate": null},
        {"name": "Source", "system": "pentad", "nodeIndex": 4, "coordinate": null}
      ],
      "connectors": [
        {"name": "Aspiration", "from": "Source", "to": "Purpose"},
        {"name": "Input", "from": "Source", "to": "Lower Potential"},
        {"name": "Output", "from": "Higher Potential", "to": "Purpose"}
      ]
    }
  }
}
```

### All Systems Query Response

```json
{
  "data": {
    "allSystems": [
      {
        "name": "monad",
        "topology": {...},
        "geometry": {...},
        "terms": [...],
        "connectors": [...]
      },
      {
        "name": "dyad",
        "topology": {...},
        "geometry": {...},
        "terms": [...],
        "connectors": [...]
      },
      ...
    ]
  }
}
```

## Color Schemes

Currently, color schemes are managed by `SystemConfig` (legacy). Each system has predefined colors:

- **nodes**: Primary node color
- **edges**: Edge color
- **selected_node**: Highlighted node color
- **selected_edge**: Highlighted edge color

Future versions may include color schemes in the GraphQL API.

## Troubleshooting

### Issue: "Network error"
**Solution**: Check that the GraphQL API is running and the endpoint URL is correct.

### Issue: "Parse error"
**Solution**: Verify the API response matches the expected GraphQL schema. Check the browser console for details.

### Issue: "System not found"
**Solution**: Ensure the system name is lowercase (monad, dyad, triad, etc.).

### Issue: Blank graph
**Solution**:
- Verify coordinates are in the 0-800 range
- Check that edges reference valid node indexes
- Ensure the system has a non-empty coordinates array

### Issue: CORS errors
**Solution**: Configure the GraphQL server to allow cross-origin requests from your frontend origin.

## Future Enhancements

1. **Environment Variables**: Read API URL from environment/config file
2. **Caching**: Cache API responses to reduce network calls
3. **Real-time Updates**: WebSocket support for live data
4. **Color Schemes**: Include in GraphQL API response
5. **Metadata API**: Fetch display names and descriptions from API
6. **Error Recovery**: Automatic retry logic with exponential backoff

## Related Files

- `src/api/graphql_client.rs` - GraphQL client implementation
- `src/api/models.rs` - Data models
- `src/api_app.rs` - Main application component
- `src/components/api_graph_view.rs` - Graph rendering component
- `schema.graphql` - GraphQL schema definition

## Additional Resources

- [systematics-v0.0.3 Repository](https://github.com/Joshfairhead/systematics-v0.0.3)
- [GraphQL Documentation](https://graphql.org/learn/)
- [Yew Framework](https://yew.rs/)
- [Trunk Build Tool](https://trunkrs.dev/)
