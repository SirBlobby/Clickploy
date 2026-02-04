FROM golang:1.23-bullseye

# Install dependencies required for Nixpacks and Git interactions
RUN apt-get update && apt-get install -y \
    git \
    curl \
    && rm -rf /var/lib/apt/lists/*

# Install Nixpacks
RUN curl -sSL https://nixpacks.com/install.sh | bash

WORKDIR /app

# Cache Go modules
COPY go.mod go.sum ./
RUN go mod download

# Copy source code
COPY . .

# Build the binary
RUN CGO_ENABLED=0 GOOS=linux go build -o server ./cmd/server

# Expose API port
EXPOSE 8080

# Run the server
CMD ["./server"]
