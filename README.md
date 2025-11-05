# Systematics Interface

A geometric visualization interface for complete graphs K1-K12, built with Rust and Yew WebAssembly.

## Overview

This interface displays complete graphs from 1 to 12 nodes (Monad through Dodecad) with geometric positioning and interactive features. Each system represents a complete graph with:

- **Indexed nodes** with visual coordinates
- **Complete edge connectivity** (all nodes connected to all other nodes)
- **Interactive selection** of nodes and edges
- **Geometric layouts** optimized for each system size
- **Color-coded systems** with unique visual themes

## Systems Included

| System | Graph | Nodes | Description |
|--------|-------|-------|-------------|
| Monad | K1 | 1 | The unity, the point, the source |
| Dyad | K2 | 2 | Duality, polarity, the line |
| Triad | K3 | 3 | Trinity, the triangle, three forces |
| Tetrad | K4 | 4 | Quaternary, the square, four elements |
| Pentad | K5 | 5 | Five-fold pattern, the pentagon |
| Hexad | K6 | 6 | Six-fold symmetry, the hexagon |
| Heptad | K7 | 7 | Seven principles, the heptagon |
| Octad | K8 | 8 | Eight-fold path, the octagon |
| Ennead | K9 | 9 | Nine spheres, completion |
| Decad | K10 | 10 | Ten sephiroth, perfection |
| Undecad | K11 | 11 | Eleven dimensions, transcendence |
| Dodecad | K12 | 12 | Twelve-fold cosmos, totality |

## Project Structure

```
systematics-interface/
├── src/
│   ├── lib.rs                  # WASM entry point
│   ├── main.rs                 # Main entry point
│   ├── app.rs                  # Main app component
│   ├── core/
│   │   ├── geometry.rs         # Graph layout calculations
│   │   └── system_config.rs    # System configurations
│   └── components/
│       ├── graph_view.rs       # SVG graph renderer
│       └── system_selector.rs  # System selection UI
├── configs/                    # JSON config files for each system
│   ├── monad.json
│   ├── dyad.json
│   └── ...
├── style.css                   # UI styling
└── index.html                  # HTML template
```

## Features

### Graph Visualization
- **SVG-based rendering** for crisp, scalable graphics
- **Geometric node positioning** using mathematical calculations
- **Complete graph edges** connecting all nodes
- **Symbolic circles** for monad and dyad systems

### Interactivity
- **Click nodes** to select and highlight them
- **View node indexes** (0 through n-1)
- **Selection feedback** with visual highlights and info display
- **System switching** via sidebar navigation

### Data Structure

Each graph is composed of:

```rust
pub struct GraphLayout {
    pub nodes: Vec<Point>,        // Node positions
    pub edges: Vec<Edge>,         // Complete graph edges
    pub node_radius: f64,         // Visual node size
    pub symbolic_circle: Option<SymbolicCircle>,
    pub symbolic_circles: Vec<SymbolicCircle>,
}

pub struct Point {
    pub x: f64,
    pub y: f64,
}

pub struct Edge {
    pub from: usize,  // Node index
    pub to: usize,    // Node index
}
```

## Building and Running

### Prerequisites
- Rust (latest stable)
- Trunk (for WebAssembly bundling)

```bash
# Install Trunk
cargo install trunk

# Build and serve
trunk serve

# Build for production
trunk build --release
```

The application will be available at `http://localhost:8080`

## Usage

1. **Select a system** from the sidebar (Monad through Dodecad)
2. **View the graph** with all nodes and edges displayed
3. **Click nodes** to select them and view their index
4. **Observe the geometry** - each system has a unique layout:
   - Monad: Single point with symbolic circle
   - Dyad: Two points with vesica piscis
   - Triad: Equilateral triangle
   - Tetrad: Diamond/square
   - Pentad+: Regular polygons

## Configuration Files

Each system has a JSON configuration in `configs/`:

```json
{
  "name": "triad",
  "display_name": "Triad",
  "node_count": 3,
  "k_notation": "K3",
  "description": "Trinity, the triangle, three forces",
  "color_scheme": {
    "nodes": "#9B59B6",
    "edges": "#888888",
    "selected_node": "#FF6B6B",
    "selected_edge": "#FF6B6B"
  }
}
```

## Future Enhancements

- Edge interaction and selection
- Node labeling options
- Export graph data
- Animation transitions between systems
- Custom node positioning
- Integration with systematic-constructor library

## Technologies

- **Rust** - Systems programming language
- **Yew** - React-like framework for WebAssembly
- **WebAssembly** - High-performance web execution
- **SVG** - Scalable vector graphics
- **Trunk** - WASM web application bundler

## License

Part of the Systematics project family.
