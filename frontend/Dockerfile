FROM oven/bun:1-alpine as base
WORKDIR /app

# Install dependencies
COPY package.json bun.lock ./
RUN bun install --frozen-lockfile

# Copy source
COPY . .

# Build
RUN bun run build

# Expose Preview Port
EXPOSE 4173

# Run
CMD ["bun", "run", "preview", "--host"]
