# API Geometry Adjustment Guide

## What the Interface Uses

The interface displays graphs using data from your GraphQL API. Here's exactly what it uses and where to make changes:

## Data Used for Rendering

### 1. **Coordinates (Primary - for visual display)**
- **API Field**: `points { x y z }`
- **What it does**: Defines where each node appears on the graph
- **This is what you need to adjust** if graphs look wrong

### 2. **Edges (Secondary - for connections)**
- **API Field**: `edges { from to }`
- **What it does**: Defines which nodes are connected by lines
- **Usually correct if it's a complete graph** (K_n has all possible edges)

### 3. **Node Indices**
- **API Field**: `nodes`
- **What it does**: Array of node IDs (e.g., [0, 1, 2] for Triad)
- **Note**: Must match the number of points

## Current API Data Examples

### Triad (K3)
```json
{
  "nodes": [0, 1, 2],
  "edges": [
    {"from": 0, "to": 1},
    {"from": 1, "to": 2},
    {"from": 2, "to": 0}
  ],
  "points": [
    {"x": 0.0, "y": 1.0, "z": null},
    {"x": 0.0, "y": -1.0, "z": null},
    {"x": 1.0, "y": 0.0, "z": null}
  ]
}
```
**Visualization**: This creates a triangle with:
- Node 0 at top (0, 1)
- Node 1 at bottom (0, -1)
- Node 2 at right (1, 0)

### Tetrad (K4)
```json
{
  "nodes": [0, 1, 2, 3],
  "points": [
    {"x": 0.0, "y": 1.0, "z": null},
    {"x": 1.0, "y": 0.0, "z": null},
    {"x": -1.0, "y": 0.0, "z": null},
    {"x": 0.0, "y": -1.0, "z": null}
  ]
}
```
**Visualization**: Creates a square/diamond:
- Node 0: top
- Node 1: right
- Node 2: left
- Node 3: bottom

## How to Adjust Coordinates in Your API

### Location in systematics-v0.0.3

Your geometry data is likely defined in one of these places:

1. **Static data file** (most likely):
   - Look for JSON, TOML, or similar config files
   - Search for: `geometry`, `coordinates`, `points`, or specific system names

2. **Rust code**:
   ```bash
   # Find where coordinates are defined
   cd /path/to/systematics-v0.0.3
   rg "points|coordinates" --type rust
   ```

3. **Database** (if using one):
   - Check your database schema for geometry tables

### What to Change

To adjust how a graph looks, modify the **`points`** array:

#### Example: Moving Triad to be more balanced
```rust
// Current (your API)
points: [
    Point { x: 0.0, y: 1.0, z: None },
    Point { x: 0.0, y: -1.0, z: None },
    Point { x: 1.0, y: 0.0, z: None },
]

// Better: Equilateral triangle centered at origin
points: [
    Point { x: 0.0, y: 0.866, z: None },      // Top
    Point { x: 0.75, y: -0.433, z: None },    // Bottom right
    Point { x: -0.75, y: -0.433, z: None },   // Bottom left
]
```

### Coordinate Tips

1. **Scale doesn't matter** - The interface auto-scales to fit 800x800 viewport
   - You can use: 0-1, 0-10, 0-100, or any range
   - Both positive and negative coordinates work

2. **Centering** - Center your coordinates around (0, 0) for best results
   - The transformation will center it, but starting centered helps

3. **Aspect ratio** - Will be preserved automatically

4. **Z-coordinate** - Currently ignored (2D display only), can be null

## Common Graph Layouts

### Triangle (K3)
```
Equilateral triangle:
[0.0, 0.866], [0.75, -0.433], [-0.75, -0.433]
```

### Square (K4)
```
[1, 1], [-1, 1], [-1, -1], [1, -1]
```

### Pentagon (K5)
```
Points arranged in circle:
angle = 2π / 5 for each point
```

### Hexagon (K6)
```
Points arranged in circle:
angle = 2π / 6 for each point
```

### 3D Polyhedra (K4+)
For tetrahedron, cube, octahedron, etc., project 3D coordinates to 2D:
- Use standard polyhedra coordinates
- Project using orthographic or perspective projection
- Store in x, y fields (z can be null or unused)

## Testing Your Changes

1. **Modify coordinates in your API code**
2. **Restart your API server**:
   ```bash
   cd /path/to/systematics-v0.0.3
   cargo run
   ```
3. **Refresh the interface** (Ctrl+R or Cmd+R)
4. **Select the system** you modified

## Debugging

### Check what the interface receives:
Open browser console (F12) and run:
```javascript
// This will show transformed coordinates
console.log(document.querySelectorAll('circle'));
```

### Check what the API sends:
```bash
curl -X POST http://localhost:8000/graphql \
  -H "Content-Type: application/json" \
  -d '{"query":"{ system(name: \"Triad\") { points { x y z } } }"}' \
  | python3 -m json.tool
```

## Quick Reference

| What needs fixing | Where to look | What to change |
|------------------|---------------|----------------|
| Node positions wrong | `points` field in API | x, y coordinates |
| Missing connections | `edges` field in API | from, to values |
| Wrong number of nodes | `nodes` field in API | Array length |
| Colors wrong | Not in API | Fixed in interface config |

## Example: Finding Your Geometry Code

```bash
cd /path/to/systematics-v0.0.3

# Find geometry definitions
rg "struct.*Geometry|struct.*Point" --type rust

# Find where Triad is defined
rg "Triad|triad" --type rust

# Find coordinate assignments
rg "x:.*y:" --type rust
```

The output will show you exactly which files to edit.
