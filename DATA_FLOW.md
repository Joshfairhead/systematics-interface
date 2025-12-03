# Data Flow: API to Display

## Overview
This document explains exactly how data flows from your GraphQL API to the visual display.

## Complete Data Flow

```
GraphQL API (systematics-v0.0.3)
    ↓
    Query: { system(name: "Triad") { nodes, edges, points } }
    ↓
GraphQL Response (JSON)
    ↓
GraphQLClient.fetch_system() [src/api/graphql_client.rs:125]
    ↓
convert_gql_system_to_system_data() [src/api/graphql_client.rs:290]
    ↓
transform_coordinates_to_viewport() [src/api/graphql_client.rs:380]
    ↓
SystemData struct
    ↓
ApiGraphView.view() [src/components/api_graph_view.rs:54]
    ↓
render_edges() + render_nodes()
    ↓
SVG <circle> and <line> elements
    ↓
Browser Display (800x800 viewport)
```

## Detailed Breakdown

### Step 1: GraphQL Query
**Location**: `src/api/graphql_client.rs:125-193`

The interface requests:
```graphql
query GetSystem($name: String!) {
    system(name: $name) {
        points { x y z }      # ← COORDINATES for visual position
        edges { from to }     # ← CONNECTIONS between nodes
        nodes                 # ← NODE IDs (must match points count)
        # ... other metadata
    }
}
```

### Step 2: Coordinate Transformation
**Location**: `src/api/graphql_client.rs:320-331`

Raw API coordinates → Viewport coordinates:
```rust
// API gives you (in any scale):
points: [{"x": 0.0, "y": 1.0}, {"x": 1.0, "y": 0.0}, ...]

// Transformation happens here:
let raw_coordinates = gql_system.points.map(|c| Coordinate { x, y, z });
let coordinates = transform_coordinates_to_viewport(raw_coordinates, 800, 800, 100);

// Result: Scaled to fit 800x800 with 100px margins
coordinates: [{"x": 400.0, "y": 100.0}, {"x": 700.0, "y": 400.0}, ...]
```

**This is automatic** - you don't need to pre-scale coordinates in your API!

### Step 3: Rendering
**Location**: `src/components/api_graph_view.rs:54-159`

The transformed coordinates are used directly:
```rust
// Nodes (circles)
<circle
    cx={ coord.x }      // ← Uses transformed x
    cy={ coord.y }      // ← Uses transformed y
    r="12"
/>

// Edges (lines)
<line
    x1={ from_coord.x }  // ← Uses transformed x of node 'from'
    y1={ from_coord.y }  // ← Uses transformed y of node 'from'
    x2={ to_coord.x }    // ← Uses transformed x of node 'to'
    y2={ to_coord.y }    // ← Uses transformed y of node 'to'
/>
```

## What Each Field Controls

| API Field | Used For | Example | Effect on Display |
|-----------|----------|---------|------------------|
| `points[].x` | Horizontal position | `0.0` | Left-right placement |
| `points[].y` | Vertical position | `1.0` | Up-down placement |
| `points[].z` | (unused in 2D) | `null` | Ignored currently |
| `edges[].from` | Start node of line | `0` | Index into points array |
| `edges[].to` | End node of line | `1` | Index into points array |
| `nodes[]` | Node IDs | `[0,1,2]` | Must match points length |

## Critical Mapping

**The edges use array indices!**

```
Example: Triad with 3 nodes

API returns:
  nodes: [0, 1, 2]
  points: [
    {x: 0.0, y: 1.0},    ← Index 0
    {x: 0.0, y: -1.0},   ← Index 1
    {x: 1.0, y: 0.0}     ← Index 2
  ]
  edges: [
    {from: 0, to: 1},    ← Draws line from points[0] to points[1]
    {from: 1, to: 2},    ← Draws line from points[1] to points[2]
    {from: 2, to: 0}     ← Draws line from points[2] to points[0]
  ]
```

## What You Need to Change in API

### To fix node positions:
**Change**: `points` array in your API
**File to modify**: In systematics-v0.0.3 repo, wherever points are defined

```rust
// Example: Make Triad more symmetric
// Before:
points: [
    Point { x: 0.0, y: 1.0 },
    Point { x: 0.0, y: -1.0 },
    Point { x: 1.0, y: 0.0 },
]

// After (equilateral triangle):
points: [
    Point { x: 0.0, y: 1.0 },
    Point { x: 0.866, y: -0.5 },
    Point { x: -0.866, y: -0.5 },
]
```

### To fix connections:
**Change**: `edges` array in your API
**Ensure**: For complete graph K_n, you need n*(n-1)/2 edges

```rust
// K3 needs 3 edges
// K4 needs 6 edges
// K5 needs 10 edges
// etc.
```

### To verify your changes:
```bash
# 1. Modify coordinates in systematics-v0.0.3
# 2. Restart API
cargo run

# 3. Query to verify
curl -X POST http://localhost:8000/graphql \
  -H "Content-Type: application/json" \
  -d '{"query":"{ system(name: \"Triad\") { points { x y } } }"}'

# 4. Refresh browser to see changes
```

## Debugging Checklist

If a graph looks wrong:

- [ ] Check `points` count matches `nodes` count
- [ ] Verify `edges` reference valid indices (0 to points.length-1)
- [ ] Ensure coordinates are numeric (not null or invalid)
- [ ] Check edge connections form expected pattern
- [ ] Verify no duplicate edges
- [ ] Check coordinate scale (any scale works, but check it's reasonable)

## Quick Test

To see what coordinates the interface is actually using:

1. Open browser console (F12)
2. Run:
```javascript
// See all circle elements (nodes)
Array.from(document.querySelectorAll('circle')).map(c => ({
  cx: c.getAttribute('cx'),
  cy: c.getAttribute('cy')
}))

// See all line elements (edges)
Array.from(document.querySelectorAll('line.edge')).map(l => ({
  x1: l.getAttribute('x1'),
  y1: l.getAttribute('y1'),
  x2: l.getAttribute('x2'),
  y2: l.getAttribute('y2')
}))
```

These will show you the **transformed** coordinates (after scaling to 800x800).
