# Systematics Interface

A web-based visualization for complete graphs (K1-K12) with GraphQL API integration, built with Rust/Yew WebAssembly.

## Overview

Interactive visualization of complete graphs from Monad (K1) to Dodecad (K12), featuring:

- **GraphQL API Integration** - Real-time data from systematics backend
- **Edge Labels** - Toggle connective/character labels on graph edges
- **Geometric Layouts** - Optimized positioning for each system
- **Interactive Navigation** - Click nodes to navigate between systems
- **Glassmorphic UI** - Modern, polished interface design

## Quick Start

### Prerequisites
```bash
# Install Rust
curl --proto '=https' --tlsv1.2 -sSf https://sh.rustup.rs | sh

# Install Trunk
cargo install trunk
```

### Running the Application
```bash
# Development server (with hot reload)
trunk serve

# Production build
trunk build --release
```

The app will be available at `http://localhost:8080`

### Backend API
The app connects to the GraphQL API at `http://localhost:8000/graphql`. Make sure the [systematics backend](https://github.com/Joshfairhead/systematics-v0.0.3) is running.

## Features

### Graph Visualization
- **12 Complete Graphs** - Monad (K1) through Dodecad (K12)
- **SVG Rendering** - Crisp, scalable vector graphics
- **Color-Coded Systems** - Unique colors for each system
- **Node Labels** - Term labels from vocabulary data

### Edge Labels
- **Toggle Switch** - Enable/disable edge labels via top navigation
- **Connective Characters** - Display relationship labels on edges
- **Smart Positioning** - Labels rotate to follow edge angle
- **Collision Avoidance** - Offset for crossing edges (Tetrad)

### Navigation
- **System Selection** - Top navigation bar with all 12 systems
- **Node Navigation** - Click nodes to navigate to related systems
- **Breadcrumb Trail** - Track navigation history
- **Back Button** - Return to previous systems

## Project Structure

```
systematics-interface/
├── src/
│   ├── api/                    # GraphQL API client
│   │   ├── graphql_client.rs   # Query execution
│   │   └── models.rs           # Data models
│   ├── components/
│   │   ├── api_graph_view.rs   # Graph renderer
│   │   └── system_selector.rs  # Navigation bar
│   ├── core/
│   │   ├── geometry.rs         # Layout calculations
│   │   └── system_config.rs    # System definitions
│   ├── api_app.rs              # Main app with API
│   └── lib.rs                  # WASM entry
├── style.css                   # Glassmorphic UI styles
├── index.html                  # HTML template
└── archive/                    # Historical documentation
```

## Systems

| System | Graph | Nodes | Edges |
|--------|-------|-------|-------|
| Monad | K1 | 1 | 0 |
| Dyad | K2 | 2 | 1 |
| Triad | K3 | 3 | 3 |
| Tetrad | K4 | 4 | 6 |
| Pentad | K5 | 5 | 10 |
| Hexad | K6 | 6 | 15 |
| Heptad | K7 | 7 | 21 |
| Octad | K8 | 8 | 28 |
| Ennead | K9 | 9 | 36 |
| Decad | K10 | 10 | 45 |
| Hendecad | K11 | 11 | 55 |
| Dodecad | K12 | 12 | 66 |

## GraphQL Integration

### API Queries

The app uses these GraphQL queries:

- `allSystems` - Fetch all 12 systems at startup
- `systemByName(name: String!)` - Fetch specific system data

### Data Flow

1. **Startup**: Fetch all systems metadata
2. **Selection**: Load detailed system data (coordinates, terms, edges, connectives)
3. **Transform**: Convert API coordinates to viewport space (800x800 SVG)
4. **Render**: Display graph with nodes, edges, and optional labels

### Coordinate Transformation

The API returns coordinates in various scales. The frontend transforms them to fit an 800x800 viewport:

```rust
// Scale and center coordinates
let scaled_x = (x - min_x) * scale_factor + margin;
let scaled_y = (y - min_y) * scale_factor + margin;
```

## Current State

### Working Features
✅ All 12 systems render correctly
✅ GraphQL API integration
✅ Node navigation between systems
✅ Edge label toggle switch
✅ Edge labels for Triad and Tetrad
✅ Glassmorphic UI design

### Known Limitations
⚠️ Edge labels only show for systems with `connectives` data in API
⚠️ Pentad and higher systems need backend to populate connectives

## Development

### Architecture
- **Frontend**: Rust + Yew (React-like WASM framework)
- **Styling**: CSS with glassmorphism effects
- **API Client**: Custom GraphQL client with `reqwest`
- **Rendering**: SVG for geometric precision

### Key Components

**ApiApp** - Main application state and API calls
**ApiGraphView** - SVG graph rendering with edge labels
**SystemSelector** - Top navigation with edge label toggle

## Documentation

Historical implementation docs are in the `archive/` folder:
- API integration guides
- Coordinate transformation details
- GraphQL setup instructions
- Geometry implementation notes

## Technologies

- **Rust** - Systems programming language
- **Yew** - WebAssembly UI framework
- **WebAssembly** - High-performance web execution
- **SVG** - Scalable vector graphics
- **Trunk** - WASM bundler
- **GraphQL** - API query language

## License

Part of the Systematics project family.
