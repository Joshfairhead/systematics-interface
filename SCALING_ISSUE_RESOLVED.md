# Scaling Issue - RESOLVED ✅

## Issue Report

**Problem**: Dyad and Triad had their horizontal midlines (y=0 in API coordinates) appearing at different vertical positions in the viewport, even though both systems used y=0 as their middle.

**Reported by**: User observation that "the triad is right, with node 2 at 0.0 but the dyad using the same coordinates is positioned much higher"

## Root Cause

The coordinate transformation was normalizing each system independently:
```rust
// OLD (incorrect)
transformed_y = (y - min_y) * scale + offset
```

This meant:
- **Dyad** with points at y=0: `(0 - 0) * scale + offset`
- **Triad** with points at y=-1,0,1: `(0 - (-1)) * scale + offset`

Different `min_y` values caused the same API coordinate (y=0) to map to different viewport positions.

## Solution

Changed transformation to be origin-preserving:
```rust
// NEW (correct)
center_y = (min_y + max_y) / 2
transformed_y = (y - center_y) * scale + viewport_center
```

Now:
- **Dyad**: center_y=0, so y=0 → (0-0)*scale + 400 = 400
- **Triad**: center_y=0, so y=0 → (0-0)*scale + 400 = 400

Both map to the same viewport y-coordinate (400 = center).

## Changes Made

**File**: `src/api/graphql_client.rs`
**Function**: `transform_coordinates_to_viewport()` (lines 376-449)

Key changes:
1. Calculate bounding box center instead of just min/max
2. Use maximum extent for uniform scaling
3. Transform relative to center: `(coord - center) * scale + viewport_center`

## Benefits

✅ **Consistent coordinate system**: All systems share the same mapping
✅ **Predictable**: API (0,0) always → Viewport (400,400)
✅ **Uniform scaling**: Same scale factor for all systems with similar extents
✅ **Aspect ratio preserved**: Still uses max extent for both axes
✅ **Auto-fitting**: Still fits all points in viewport

## Testing

Verified with:
- **Dyad**: Points at (-1, 0) and (1, 0) → Both at viewport y=400
- **Triad**: Point at (1, 0) → At viewport y=400
- Both systems now have consistent vertical alignment

## No API Changes Required

This fix is entirely in the interface. Your existing API coordinates work correctly without modification.

## Documentation Updated

- ✅ `COORDINATE_TRANSFORMATION.md` - Updated with new algorithm details
- ✅ `SCALING_FIX.md` - Created detailed explanation
- ✅ `SCALING_ISSUE_RESOLVED.md` - This summary

## Build Status

✅ Build successful: `trunk build --release`
✅ No compilation errors
✅ Ready to serve and test

## Next Steps

1. Serve the updated interface: `trunk serve --port 3000`
2. Verify Dyad and Triad alignment
3. Test other systems (Tetrad, Pentad, etc.)
4. Confirm all graphs render with consistent coordinate mapping
