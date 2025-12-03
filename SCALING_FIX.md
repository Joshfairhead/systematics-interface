# Coordinate Scaling Fix

## Problem

Different systems (Dyad, Triad, etc.) were being scaled independently based on their own bounding boxes, causing inconsistent positioning. For example:

- **Dyad**: points at (-1, 0) and (1, 0) — horizontal line at y=0
- **Triad**: points at (0, 1), (0, -1), (1, 0) — triangle with center at y=0

Even though both had y=0 as their middle, they appeared at different vertical positions in the viewport because each was scaled and centered based on its own bounding box.

## Root Cause

The old transformation algorithm:
1. Found min/max for each coordinate axis
2. Normalized coordinates: `(coord - min) * scale + offset`
3. This shifted each system independently, breaking coordinate system consistency

## Solution

The new transformation algorithm (src/api/graphql_client.rs:399-449):

1. **Find bounding box** of all points
2. **Calculate center** of bounding box: `(min + max) / 2`
3. **Use maximum extent** for both axes to preserve aspect ratio
4. **Transform relative to center**: `(coord - center) * scale + viewport_center`

This ensures:
- ✅ All systems share the same coordinate space
- ✅ y=0 in API space → y=400 (center) in viewport space
- ✅ Consistent scaling across all systems
- ✅ Aspect ratio preserved
- ✅ Graphs automatically fit in viewport with margins

## Example

### Dyad
```
API coordinates:
  (-1.0, 0.0), (1.0, 0.0)

Old transformation:
  min_y = 0, max_y = 0
  Both → (0 - 0) * scale + offset = different position each time

New transformation:
  center_y = 0.0
  Both → (0.0 - 0.0) * scale + 400 = 400
  Result: Horizontal line at viewport center (y=400)
```

### Triad
```
API coordinates:
  (0.0, 1.0), (0.0, -1.0), (1.0, 0.0)

Old transformation:
  min_y = -1, max_y = 1
  Point at y=0 → (0 - (-1)) * scale + offset = different position

New transformation:
  center_y = 0.0
  Point at y=0 → (0.0 - 0.0) * scale + 400 = 400
  Result: Middle point at viewport center (y=400)
```

### Result
Now both Dyad and Triad have their y=0 line at the same vertical position (y=400 in viewport).

## Benefits

1. **Consistent coordinate system** - All systems use the same mapping
2. **Predictable positioning** - API coordinate (0, 0) always maps to viewport center (400, 400)
3. **Uniform scaling** - Uses same scale for X and Y axes
4. **Automatic fitting** - Still fits all points within viewport with margins

## Implementation Details

```rust
// Key transformation formula:
x_viewport = (x_api - center_x) * scale + viewport_center_x
y_viewport = (y_api - center_y) * scale + viewport_center_y

where:
- center_x, center_y = center of bounding box
- scale = available_size / max_extent
- viewport_center_x = 400 (half of 800)
- viewport_center_y = 400 (half of 800)
```

## Testing

To verify the fix:

1. **Build and serve**:
   ```bash
   trunk build --release
   trunk serve --port 3000
   ```

2. **View Dyad and Triad**:
   - Both should now have consistent vertical alignment
   - Their horizontal midlines should be at the same height

3. **Check console coordinates**:
   ```javascript
   // In browser console
   Array.from(document.querySelectorAll('circle')).map(c => ({
     cx: c.getAttribute('cx'),
     cy: c.getAttribute('cy')
   }))
   ```

   Points with API y=0 should all have cy=400.

## Migration Note

No changes needed in the API! This fix is entirely in the interface's coordinate transformation logic. Your existing API coordinates will work correctly with this new scaling approach.
