# API Integration Guide

## Overview

This interface has been refactored to be a **data-driven shell** that fetches topology, geometry, and vocabulary from an API. The viewport constructs graphs dynamically based on API responses.

## Architecture

### Data Flow

```
API → ApiClient → SystemData → ApiGraphView → SVG Rendering
```

1. **API Endpoint** provides system data (topology, geometry, vocabulary)
2. **ApiClient** fetches and combines data from multiple endpoints
3. **SystemData** model holds the complete system information
4. **ApiGraphView** renders the graph using coordinates and edges from API
5. **SVG** displays the interactive visualization

### Key Components

#### 1. API Models (`src/api/models.rs`)

**Core Data Structures:**

- `Coordinate` - 2D point with x, y coordinates
- `TopologyEdge` - Edge connecting two nodes by index
- `GeometryData` - Node coordinates and indexes
- `TopologyData` - Graph topology (indexes and edges)
- `VocabularyData` - System terminology and descriptions
- `SystemData` - Complete system combining all three data sources

**Future API Structure:**

The API is being refactored to separate concerns:
- `/geometry/{system}` - Returns only coordinates (will be pure coordinates in future)
- `/topology/{system}` - Returns only indexes and edges (future endpoint)
- `/vocabulary/{system}` - Returns terminology and descriptions

Currently, `GeometryData` may contain edges during the transition period. `SystemData.with_complete_graph_edges()` can generate complete graph edges if the API doesn't provide them.

#### 2. API Client (`src/api/client.rs`)

**ApiClient** - Production client for real API endpoints

```rust
let client = ApiClient::new("https://api.example.com".to_string());
let system = client.fetch_system("pentad").await?;
```

**MockApiClient** - Development/testing client that uses procedural geometry

Currently active in the app. Uses the existing `GeometryCalculator` to generate mock data matching the API structure.

```rust
let system = MockApiClient::fetch_system("pentad").await?;
```

#### 3. Components

**ApiGraphView** (`src/components/api_graph_view.rs`)
- Renders graphs using `SystemData` from API
- Maps coordinates to indexes
- Renders edges from topology data
- Handles node and edge selection

**ApiApp** (`src/api_app.rs`)
- Main application component
- Manages async data loading
- Handles system selection
- Shows loading/error states

## Switching Between Mock and Real API

### Current Setup (Mock API)

The app currently uses `MockApiClient` for development:

```rust
// In src/api_app.rs
match MockApiClient::fetch_system(&name).await {
    Ok(system) => { /* ... */ }
}
```

### Switching to Real API

To use a real API endpoint:

1. Update `src/api_app.rs`:

```rust
// Create API client with base URL
impl Component for ApiApp {
    fn create(ctx: &Context<Self>) -> Self {
        let api_client = ApiClient::new("https://your-api-url.com".to_string());

        // Store in component state and use for all requests
        // ...
    }
}
```

2. Replace `MockApiClient::fetch_*` calls with `api_client.fetch_*`

3. Ensure your API returns data matching these structures:

**Geometry Endpoint** (`/geometry/pentad`):
```json
{
  "system_name": "pentad",
  "k_notation": "K5",
  "node_count": 5,
  "coordinates": [
    {"x": 400.0, "y": 120.0},
    {"x": 632.36, "y": 254.1},
    {"x": 540.7, "y": 519.1},
    {"x": 259.3, "y": 519.1},
    {"x": 167.64, "y": 254.1}
  ],
  "indexes": [0, 1, 2, 3, 4],
  "edges": [
    {"from": 0, "to": 1},
    {"from": 0, "to": 2},
    // ... complete graph edges
  ]
}
```

**Vocabulary Endpoint** (`/vocabulary/pentad`):
```json
{
  "system_name": "pentad",
  "display_name": "Pentad",
  "k_notation": "K5",
  "description": "A five-element system...",
  "terms": ["term1", "term2", ...]
}
```

**Topology Endpoint** (Future - `/topology/pentad`):
```json
{
  "system_name": "pentad",
  "k_notation": "K5",
  "node_count": 5,
  "indexes": [0, 1, 2, 3, 4],
  "edges": [
    {"from": 0, "to": 1},
    {"from": 0, "to": 2},
    // ... complete graph edges
  ]
}
```

## Coordinate System

The interface uses a **800x800 SVG viewport** with coordinates:
- Origin: (0, 0) at top-left
- Center: (400, 400)
- Typical node positions: within 100-700 range on both axes

**Example (Pentad/K5):**
- 5 nodes arranged as a regular pentagon
- Radius: ~280 units from center
- Coordinates calculated to form equilateral spacing

Your API should provide coordinates in this coordinate space, or you can normalize them in the client.

## Adding New Systems

To add support for new systems beyond K1-K12:

1. **API Side**: Add endpoints for the new system
2. **Client Side**: No changes needed - the client is fully data-driven
3. **Color Schemes**: Update `SystemConfig::get_by_name()` or add color scheme to API

## Development Workflow

### Building
```bash
trunk build --release
```

### Serving Locally
```bash
trunk serve
```

### Testing with Mock API
The app currently uses `MockApiClient` which generates data procedurally. This allows you to:
- Develop the interface without a running API
- Test the UI with consistent data
- Prototype new features

### Testing with Real API
1. Update `ApiApp` to use `ApiClient` with your API URL
2. Ensure CORS is configured on your API
3. Deploy or serve locally and test

## File Structure

```
src/
├── api/
│   ├── mod.rs           # Module exports
│   ├── models.rs        # Data structures (Coordinate, SystemData, etc.)
│   └── client.rs        # ApiClient and MockApiClient
├── api_app.rs           # Main API-driven app component
├── components/
│   ├── api_graph_view.rs    # Graph renderer using API data
│   ├── graph_view.rs        # Legacy graph renderer
│   └── system_selector.rs   # System selection UI
├── core/
│   ├── geometry.rs      # Procedural geometry calculator (used by mock)
│   └── system_config.rs # Legacy system configuration
└── lib.rs               # Entry point
```

## Migration Notes

### Legacy vs API-Driven

**Legacy Components** (still present):
- `App` - Original hardcoded app
- `GraphView` - Original procedural renderer
- `GeometryCalculator` - Procedural coordinate generation

**New API-Driven Components** (active):
- `ApiApp` - Async data-fetching app
- `ApiGraphView` - Data-driven renderer
- `ApiClient` / `MockApiClient` - API integration

The legacy components are retained for reference but not used when `ApiApp` is active.

### Switching Back to Legacy

To use the original procedural interface:

```rust
// In src/lib.rs
pub fn run_app() {
    yew::Renderer::<app::App>::new().render();  // Legacy
    // yew::Renderer::<api_app::ApiApp>::new().render();  // API-driven
}
```

## Future Enhancements

1. **Error Handling**: More robust error messages and retry logic
2. **Caching**: Cache API responses to reduce network calls
3. **Loading States**: Better loading indicators and progressive rendering
4. **Real-time Updates**: WebSocket support for live data updates
5. **Configuration**: Runtime API URL configuration (environment variables)
6. **Optimization**: Lazy loading for large graphs

## CORS Configuration

If using the real API, ensure your API server allows CORS requests:

```
Access-Control-Allow-Origin: *
Access-Control-Allow-Methods: GET, OPTIONS
Access-Control-Allow-Headers: Content-Type
```

## Troubleshooting

**Issue**: API requests failing
- Check browser console for CORS errors
- Verify API URL is correct
- Ensure API endpoints match expected structure

**Issue**: Blank graph
- Verify coordinates are in 0-800 range
- Check edge indexes don't exceed node count
- Ensure SystemData has non-empty coordinates array

**Issue**: Build errors
- Run `cargo check` to see detailed errors
- Ensure all dependencies are in Cargo.toml
- Check Rust version is 2021 edition or later
