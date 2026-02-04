# Clickploy Backend

The backend control plane for Clickploy, written in Go.

## Overview
This service handles:
- **Project Management**: Creating and listing projects.
- **Port Assignment**: Allocating ports (4000-5000) for deployments.
- **Builds**: Using `nixpacks` to build Docker images from Git URLs.
- **Deployment**: interfacing with the Docker daemon to run containers.
- **Logs**: Streaming build and runtime logs via WebSockets.
- **Webhooks**: Handling Git push events.



### API Endpoints
- `GET /api/projects`: List projects
- `POST /api/projects`: Create a project
- `GET /api/activity`: Get recent activity
- `WS /api/deployments/:id/logs/stream`: Stream logs

## Structure
- `cmd/server/`: Entry point.
- `internal/api/`: Gin HTTP handlers.
- `internal/db/`: SQLite database setup.
- `internal/models/`: GORM models.
- `internal/builder/`: Nixpacks wrapper.
- `internal/deployer/`: Docker SDK wrapper.
