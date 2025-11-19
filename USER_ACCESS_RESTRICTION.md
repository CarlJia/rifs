# User Access Restriction Implementation

## Overview
This document describes the implementation of role-based access control to restrict ordinary users to only Home and Gallery pages, while keeping administrative functions (cache management, user management, settings) accessible only to admin users.

## Changes Made

### 1. Frontend Changes

#### Layout Component (`src-tauri/components/Layout.tsx`)
- Added role detection from `localStorage.getItem('user_role')`
- Menu items are now dynamically filtered based on user role:
  - **Admin users**: Can see all menu items (Home, Gallery, Cache, Users, Settings)
  - **Regular users**: Can see only Home and Gallery

#### App Component (`src-tauri/App.tsx`)
- Added role-based page access control
- Regular users trying to access restricted pages are shown an "Access Denied" message
- Admin pages default to Home if accessed by non-admin users

#### Login Page (`src-tauri/pages/Login.tsx`)
- Enhanced login flow to verify token and retrieve user role
- Calls `/api/auth/verify` endpoint to validate token and get role information
- Stores user role in `localStorage` as `user_role` for persistent access control
- Shows appropriate error messages for failed authentication

### 2. Backend Changes

#### Middleware (`src/middleware/auth.rs`)
- Added new `AdminGuard` extractor for admin-only endpoint protection
- When authentication is disabled, defaults to allowing access (backward compatible)
- When authentication is enabled, validates token against configured admin token
- Returns 401 Unauthorized for invalid/missing credentials

#### Handlers
1. **Cache Handler** (`src/handlers/cache_handler.rs`)
   - `decay_heat_scores()`: Now requires `AdminGuard`
   - `auto_cleanup_cache()`: Now requires `AdminGuard`
   - `clear_all_cache()`: Now requires `AdminGuard`
   - `get_cache_stats()`: Remains public (read-only, no auth required)

2. **Token Handler** (`src/handlers/token_handler.rs`)
   - `list_tokens()`: Now requires `AdminGuard`
   - `create_token()`: Now requires `AdminGuard`
   - `delete_token()`: Now requires `AdminGuard`
   - `get_token()`: Now requires `AdminGuard`

3. **Auth Handler** (`src/handlers/auth_handler.rs`)
   - Updated `AuthResponse` to include user `role` field
   - `verify_token()` now returns role information:
     - Admin token = "admin" role
     - No auth = None role (defaults to "user" in frontend)

### 3. Public vs Protected Endpoints

#### Public Endpoints (No Auth Required)
- `GET /` - Home page
- `GET /gallery` - Gallery page
- `GET /login` - Login page
- `POST /upload` - Image upload
- `GET /api/images/query` - List images
- `GET /api/stats` - Image statistics
- `GET /api/cache/stats` - Cache statistics (read-only)
- `GET /api/auth/config` - Auth configuration
- `GET /health` - Health check
- `POST /api/auth/verify` - Token verification

#### Admin-Only Endpoints (Requires AdminGuard)
- `GET /api/tokens/list` - List tokens
- `POST /api/tokens/create` - Create token
- `GET /api/tokens/{id}` - Get token
- `DELETE /api/tokens/{id}` - Delete token
- `POST /api/cache/decay` - Decay cache heat scores
- `POST /api/cache/cleanup/auto` - Auto cleanup cache
- `DELETE /api/cache/clear` - Clear all cache

### 4. Tests (`tests/authorization_tests.rs`)

Comprehensive test suite with 13 tests covering:
- Public page accessibility (home, gallery, login)
- API endpoint accessibility (upload, query, stats)
- Health checks and system status
- Admin endpoint behavior when auth is disabled
- Auth configuration endpoint
- 404 handling for non-existent routes

All tests passing ✓

## User Role Determination

1. **During Authentication**:
   - Token from config file = "admin" role
   - No token/regular user tokens = "user" role (default)

2. **Frontend Storage**:
   - Stored in `localStorage` as `user_role`
   - Default to "user" if not present
   - Cleared on logout

## Backward Compatibility

- When `auth.enabled = false` in config:
  - All endpoints remain accessible
  - No authentication required
  - No role-based restrictions applied
  - Allows existing deployments to work unchanged

## Security Considerations

1. Role information stored in localStorage is client-side only
2. Backend endpoints enforce authorization via `AdminGuard`
3. Client-side restrictions are for UX; server always validates permissions
4. Token validation happens on both frontend (for UI) and backend (for actual API calls)
5. Invalid/missing tokens return 401 Unauthorized

## Configuration

No new configuration needed. Existing `auth` section in config file is used:
```toml
[auth]
enabled = false  # Set to true to enable authentication
token = ""      # Admin token (when enabled)
header_name = "Authorization"
```

## Testing the Feature

1. With auth disabled (default):
   - All pages and endpoints accessible
   - All menu items visible

2. With auth enabled:
   - Login required to access the app
   - Admin token = full access
   - Regular token = only home/gallery visible
   - Admin endpoints return 401 if not authenticated as admin

## Migration Notes

- Existing deployments work without changes
- To enable user restriction:
  1. Set `auth.enabled = true` in config
  2. Configure admin `token` value
  3. Optionally create user tokens via API
  4. Frontend automatically enforces restrictions

