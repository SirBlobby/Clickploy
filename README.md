# Clickploy

A minimal, self-hosted Platform as a Service (PaaS) for building and deploying applications quickly.

## Features
- **Auto-Build**: Uses `nixpacks` to automatically detect and build your application from a Git repository.
- **Docker-based**: Each deployment runs in an isolated Docker container.
- **Real-time Logs**: View build and runtime logs streamed directly to the dashboard.
- **Port Management**: Automatically assigns and manages ports for your applications.
- **Deployment History**: Track every build with detailed commit info and status.
- **Environment Variables**: Securely manage runtime configuration.
- **Manual Redeploy**: Trigger rebuilds with a single click.
- **Zero Configuration**: Just provide a repo URL, and Clickploy handles the rest.

## Tech Stack
- **Frontend**: SvelteKit, Tailwind CSS, Shadcn-Svelte
- **Backend**: Go (Gin), GORM (SQLite), Docker SDK
- **Build Engine**: Nixpacks
- **Database**: SQLite (Embedded)

## Getting Started

### Prerequisites
- Docker & Docker Compose
- Go 1.21+ (for development)
- Node.js 20+ (for development)

### Running Locally
1. Clone the repository.
2. Run `docker-compose up --build`.
3. Access the dashboard at `http://localhost:5173`.

## Architecture
Clickploy acts as a control plane for your deployments. It clones your repository, builds a Docker image using Nixpacks, and spins up a container. It manages a persistent database of projects and deployments, ensuring state is maintained across restarts.

## License
MIT