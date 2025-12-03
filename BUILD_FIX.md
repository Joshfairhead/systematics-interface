# Build Fix

## Issue
The initial build was failing with the error:
```
error[E0433]: failed to resolve: could not find `api` in the crate root
 --> src/components/api_graph_view.rs:2:12
```

## Root Cause
Rust projects can have both a library target (`src/lib.rs`) and a binary target (`src/main.rs`). These are **separate compilation units** with their own module declarations.

When we added the `api` module to `src/lib.rs`, it was only available to the library. The binary in `src/main.rs` didn't have access to it, causing the build to fail when compiling the binary target.

## Solution
Updated `src/main.rs` to include the same modules as `src/lib.rs`:

**Before:**
```rust
mod app;
mod components;
mod core;

use app::App;

fn main() {
    yew::Renderer::<App>::new().render();
}
```

**After:**
```rust
mod app;
mod api_app;
mod components;
mod core;
mod api;

use api_app::ApiApp;

fn main() {
    yew::Renderer::<ApiApp>::new().render();
}
```

## Build Status
✅ `cargo build` - Success
✅ `trunk build` - Success
✅ WASM artifact generated - `dist/systematics-interface-*.wasm` (1.7MB)

## Warnings (Non-Critical)
The build generates 9 warnings about unused code:
- Unused `ApiClient` (will be used when connecting to real API)
- Unused `EdgeClicked` variants (interactive features)
- Unused helper methods

These are expected and don't affect functionality. They'll be resolved when the real API is connected and interactive features are enabled.

## Testing
You can now run the application with:
```bash
trunk serve
```

Then open http://localhost:8080 in your browser.
