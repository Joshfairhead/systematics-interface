# How to Enable GraphQL API

This is a quick reference for switching from mock data to the real GraphQL API.

## Steps

### 1. Edit `src/api_app.rs`

Open the file and make these two changes:

**Line 33:** Change `use_graphql` from `false` to `true`

```rust
// Before:
let use_graphql = false;

// After:
let use_graphql = true;
```

**Line 36:** Update the endpoint URL to your GraphQL server

```rust
// For production:
let graphql_endpoint = "https://your-api-domain.com/graphql".to_string();

// For local development:
let graphql_endpoint = "http://localhost:8080/graphql".to_string();
```

### 2. Rebuild and Run

```bash
trunk serve
```

That's it! The interface will now fetch data from the GraphQL API.

## Switching Back to Mock Data

Simply set `use_graphql` back to `false`:

```rust
let use_graphql = false;
```

## Verifying the Connection

When you run the interface:

1. Open the browser console (F12)
2. Look for network requests to your GraphQL endpoint
3. If successful, you'll see system data loading
4. If there's an error, check:
   - Is the API server running?
   - Is the endpoint URL correct?
   - Are there CORS issues? (check console for CORS errors)

## Example API Endpoints

Common GraphQL endpoint patterns:

- `http://localhost:8080/graphql` - Local development
- `https://api.example.com/graphql` - Production
- `https://your-app.vercel.app/api/graphql` - Vercel deployment
- `https://your-app.netlify.app/.netlify/functions/graphql` - Netlify functions

## Quick Troubleshooting

| Problem | Solution |
|---------|----------|
| Network error | Check API is running and URL is correct |
| CORS error | Configure CORS on API server |
| Parse error | Verify API response matches schema |
| Blank screen | Check browser console for errors |

## Full Documentation

See [GRAPHQL_INTEGRATION.md](GRAPHQL_INTEGRATION.md) for complete documentation.
