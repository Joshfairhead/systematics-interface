# Coordinate Transformation

## Overview

The GraphQL API integration now includes automatic coordinate transformation to ensure that coordinates from any scale are properly displayed in the 800x800 SVG viewport.

## Implementation

Location: `src/api/graphql_client.rs` - `transform_coordinates_to_viewport()` function (lines 376-449)

## Features

### 1. Scale Independence
The API can return coordinates in any scale:
- Unit scale (0-1)
- Small scale (0-10)
- Large scale (0-1000)
- Mathematical coordinates (can be negative)
- Even all zeros (0, 0, 0)

### 2. Consistent Coordinate System
**Important**: The transformation preserves the coordinate system origin:
- API coordinate (0, 0) → Viewport coordinate (400, 400)
- All systems use the same coordinate space mapping
- y=0 in API space always maps to the same vertical position in viewport

### 3. Automatic Scaling
The transformation:
1. Finds the bounding box of all coordinates
2. Calculates center of bounding box
3. Uses maximum extent for uniform scaling
4. Preserves aspect ratio (uses same scale for x and y)
5. Centers the graph in viewport by transforming relative to center

### 4. Special Cases
- **Single Point**: Centered at (400, 400)
- **Collinear Points**: Handled with minimum extent fallback
- **All Same Point**: Treated as single point case

### 5. Margins
- Default: 100px margins considered in available space calculation
- Actual drawing area: Determined by max extent and uniform scaling
- Graph is centered in 800x800 viewport

## Examples

### Example 1: Monad (Single Point)
```
API returns: (0, 0)
Transformed to: (400, 400)
```

### Example 2: Dyad (Horizontal Line)
```
API returns: [(-1.0, 0.0), (1.0, 0.0)]

Calculation:
- Bounding box: min_x=-1, max_x=1, min_y=0, max_y=0
- Center: center_x=0, center_y=0
- Extents: extent_x=2, extent_y=0.0001 (fallback)
- Max extent: 2
- Scale: 600/2 = 300

Transformation (for each point):
  x_viewport = (x - 0) * 300 + 400
  y_viewport = (0 - 0) * 300 + 400 = 400

Result:
  (-1, 0) → (100, 400)
  (1, 0)  → (700, 400)

Both points at y=400 (viewport center)
```

### Example 3: Triad (Triangle)
```
API returns: [(0.0, 1.0), (0.0, -1.0), (1.0, 0.0)]

Calculation:
- Bounding box: min_x=0, max_x=1, min_y=-1, max_y=1
- Center: center_x=0.5, center_y=0
- Extents: extent_x=1, extent_y=2
- Max extent: 2
- Scale: 600/2 = 300

Transformation (for point at (1.0, 0.0)):
  x_viewport = (1.0 - 0.5) * 300 + 400 = 550
  y_viewport = (0.0 - 0.0) * 300 + 400 = 400

Point at API (1.0, 0.0) → Viewport (550, 400)
Point at y=0 → y=400 (same as Dyad!)
```

## Testing

To verify the transformation is working:

1. Start the GraphQL API server
2. Start the interface with `trunk serve --port 3000`
3. Select different systems (Monad, Dyad, Triad, etc.)
4. Verify graphs are visible and properly centered
5. Check browser console for coordinate values

## Troubleshooting

If graphs still don't appear:
1. Check browser console for JavaScript errors
2. Inspect SVG elements in browser DevTools
3. Verify coordinates are within 0-800 range
4. Check that edges reference valid node indices
5. Ensure color scheme values are valid CSS colors
