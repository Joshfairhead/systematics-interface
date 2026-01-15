# Quick Fix Guide: Adjusting Graph Display

## TL;DR - What to Change

**Problem**: A graph doesn't look right in the interface
**Solution**: Modify the `points` array in your systematics-v0.0.3 API

## Location in Your API

```bash
cd /path/to/systematics-v0.0.3

# Find where geometry/points are defined
rg "points|Point \{" --type rust

# Common locations:
# - src/geometry/*.rs
# - src/systems/*.rs
# - data/*.json or *.toml
```

## What the Interface Reads

From your GraphQL API, the interface uses:

1. **`points`** - Array of {x, y, z} coordinates
   - **This is what you change to fix display**
   - Controls where nodes appear
   - Scale doesn't matter (auto-scaled to fit)

2. **`edges`** - Array of {from, to} pairs
   - Controls which nodes connect
   - Uses array indices (0, 1, 2, ...)

3. **`nodes`** - Array of node IDs
   - Must have same length as `points`

## Example Fix

### Current API Response (Triad):
```json
{
  "points": [
    {"x": 0.0, "y": 1.0, "z": null},
    {"x": 0.0, "y": -1.0, "z": null},
    {"x": 1.0, "y": 0.0, "z": null}
  ]
}
```
**Problem**: Not an equilateral triangle

### Fixed Version:
```json
{
  "points": [
    {"x": 0.0, "y": 0.866, "z": null},
    {"x": 0.75, "y": -0.433, "z": null},
    {"x": -0.75, "y": -0.433, "z": null}
  ]
}
```
**Result**: Proper equilateral triangle

## Testing Your Changes

```bash
# 1. Edit your API code
vim src/geometry/triad.rs  # (or wherever it is)

# 2. Restart API
cargo run

# 3. Test with curl
curl -X POST http://localhost:8000/graphql \
  -H "Content-Type: application/json" \
  -d '{"query":"{ system(name: \"Triad\") { points { x y z } } }"}'

# 4. Refresh browser (Cmd+R or Ctrl+R)
```

## Common Issues

| Issue | Cause | Fix |
|-------|-------|-----|
| Nodes clustered in corner | Coordinates all near 0 | Spread them out (use range like -1 to 1) |
| Graph too small/large | (N/A) | Auto-scaled - will fit regardless |
| Nodes overlapping | Coordinates too similar | Increase spacing between points |
| Weird shape | Coordinates don't match intended geometry | Use proper geometric formulas |
| Missing nodes | Not enough points | Add more points to match node count |
| Missing edges | Missing edge definitions | Add all edge pairs for complete graph |

## Helpful Geometric Formulas

### Equilateral Triangle (K3)
```rust
let r = 1.0;  // radius
points = [
    Point { x: 0.0, y: r },
    Point { x: r * 0.866, y: -r * 0.5 },
    Point { x: -r * 0.866, y: -r * 0.5 },
]
```

### Square (K4)
```rust
let r = 1.0;
points = [
    Point { x: r, y: r },
    Point { x: -r, y: r },
    Point { x: -r, y: -r },
    Point { x: r, y: -r },
]
```

### Regular N-gon (Pentagon, Hexagon, etc.)
```rust
let n = 5;  // number of points
let r = 1.0;  // radius
points = (0..n).map(|i| {
    let angle = 2.0 * PI * (i as f64) / (n as f64);
    Point {
        x: r * angle.cos(),
        y: r * angle.sin(),
    }
}).collect()
```

## Verification

After changing coordinates, verify the graph displays correctly:

1. **Visual check**: Does it look like the intended shape?
2. **Symmetry**: Are similar nodes equally spaced?
3. **Edges**: Do all expected connections appear?
4. **Labels**: Are node numbers readable?

## Need More Info?

See these detailed guides:
- **API_GEOMETRY_GUIDE.md** - Complete guide to API geometry
- **DATA_FLOW.md** - How data flows from API to display
- **COORDINATE_TRANSFORMATION.md** - How coordinates are scaled
