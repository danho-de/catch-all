# Catch-All

A lightweight static file server built with Rust using Actix-web, designed to serve as a catch-all route in Traefik for handling unmatched domain requests.

## Purpose

This service acts as a default fallback for Traefik reverse proxy, displaying a custom "Website Not Found" page when users access domains that aren't configured in your Traefik setup. Perfect for multi-domain environments where you want to provide a branded 404 experience instead of a generic error.

## Features

- **Traefik Integration**: Configured as lowest-priority catch-all route
- **Static File Server**: Serves files from the `./public` directory
- **Custom 404 Page**: Branded error page with dark mode support
- **Configurable**: Environment variables for port, email, and logo
- **Lightweight**: Statically-linked musl binary running on scratch Docker image (not 5MB)
- **Fast & Secure**: Built with Rust for performance and memory safety

## Environment Variables

Configure the server using the following environment variables:

| Variable | Description | Default |
|----------|-------------|---------|
| `PORT` | HTTP server port | `8080` |
| `EMAIL` | Contact email displayed on the page | `info@domain.local` |
| `LOGO_PATH` | URL to logo image | `https://placehold.co/400x300` |
| `LOGO_NAME` | Logo filename (for custom logos mounted via volume) | `logo.png` (will not used if is default name) |

### Example Configuration

In `docker-compose.yml`:

```yaml
environment:
  - PORT=8080
  - EMAIL=support@yourdomain.com
  - LOGO_PATH=https://yourdomain.com/logo.png
  - LOGO_NAME=myLogo.png # For custom logo mounted via volume
volumes:
  - ./data/logo:/app/public/logo:ro  # Mount custom logo directory if needed
```

## Building

### Prerequisites

- Rust 1.70 or later
- musl toolchain (for static linking)
- Docker & Docker Compose (for containerized deployment)

### First-Time Setup

Install the musl toolchain for static linking:

```bash
./setup-musl.sh
```

### Local Build

Build the statically-linked binary:

```bash
./build.sh
```

This will:
1. Format the Rust code
2. Build an optimized musl release binary
3. Strip debug symbols
4. Copy the binary to `./catch-all`

### Docker Build

```bash
docker-compose build
```

## Running

### Local Run

```bash
# With default settings
./catch-all
# With custom environment variables
PORT=8080 EMAIL=admin@example.com LOGO_PATH=https://yourdomain.com/logo.png ./catch-all
# For custom logo name
PORT=8080 EMAIL=admin@example.com LOGO_NAME=myLogo.png ./catch-all
```

The server will start on `http://localhost:8080`

### Docker Run with Traefik

```bash
docker-compose up -d
```

## Traefik Configuration

The service is configured in `docker-compose.yml` with the following Traefik labels:

- **Priority**: Set to `1` (lowest priority) to catch all unmatched requests
- **Rule**: `HostRegexp('^.+$')` matches any hostname
- **Entry Point**: `websecure` (HTTPS)
- **TLS**: Enabled automatically
- **Network**: Uses `traefik-proxy` external network

Any request that doesn't match other Traefik routes will be directed to this service.

## Project Structure

```
├── src/
│   └── main.rs          # Rust server implementation
├── public/              # Static files directory
│   ├── logo/            # Custom logo directory
│   └── index.html       # Custom 404 page with dark mode
├── Cargo.toml           # Rust dependencies and config
├── .cargo/
│   └── config.toml      # Cargo build configuration for musl
├── build.sh             # Build script for musl static binary
├── setup-musl.sh        # One-time musl toolchain setup
├── Dockerfile           # Minimal scratch-based image
├── docker-compose.yml   # Docker Compose with Traefik labels
└── .gitignore           # Git ignore rules
```

## Docker Image Details

- **Base Image**: `scratch` (minimal, secure)
- **Binary**: Statically-linked musl binary (~5-10MB)
- **Runtime**: No dependencies, completely self-contained
- **Size**: Final image size ~10-15MB including static files

## API Endpoints

### GET /config

Returns server configuration as JSON:

```json
# default response
{
  "email": "info@domain.local",
  "logo_path": "https://placehold.co/400x300",
  "logo_name": "logo.png"
}
```

## License

MIT