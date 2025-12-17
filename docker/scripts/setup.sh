#!/bin/bash
# Handy Docker MCP Setup Script

set -e

# Detect docker-compose command (v1 or v2)
if command -v docker compose &> /dev/null; then
    DOCKER_COMPOSE="docker compose"
elif command -v docker-compose &> /dev/null; then
    DOCKER_COMPOSE="docker-compose"
else
    echo "❌ Error: docker-compose not found!"
    echo "Please install Docker Compose:"
    echo "  sudo apt-get install docker-compose-plugin"
    echo "  OR"
    echo "  Follow:  https://docs.docker.com/compose/install/"
    exit 1
fi

echo "🚀 Setting up Handy Docker MCP environment..."
echo "📌 Using:  $DOCKER_COMPOSE"

# Pull all images
echo "📦 Pulling Docker images..."
$DOCKER_COMPOSE pull || {
    echo "⚠️  Some images failed to pull.  Continuing..."
}

# Create necessary directories
echo "📁 Creating project directories..."
mkdir -p src-tauri/resources/models
mkdir -p src-tauri/data
mkdir -p . cache/docker

# Install Rust dependencies in container
echo "🦀 Installing Rust dependencies..."
$DOCKER_COMPOSE run --rm rust-dev cargo fetch || {
    echo "⚠️  Rust dependency fetch failed. You may need to run manually."
}

# Install Node dependencies in container
echo "📦 Installing Bun dependencies..."
$DOCKER_COMPOSE run --rm bun-dev bun install || {
    echo "⚠️  Bun install failed. You may need to run manually."
}

# Download VAD model if not exists
if [ !  -f "src-tauri/resources/models/silero_vad_v4.onnx" ]; then
    echo "⬇️  Downloading Silero VAD model..."
    curl -o src-tauri/resources/models/silero_vad_v4.onnx \
        https://blob.handy.computer/silero_vad_v4.onnx || {
        echo "⚠️  VAD model download failed. Download manually from:"
        echo "  https://blob.handy.computer/silero_vad_v4.onnx"
    }
else
    echo "✅ VAD model already exists"
fi

echo ""
echo "✅ Setup complete!"
echo ""
echo "Next steps:"
echo "  1. Start services: $DOCKER_COMPOSE up -d"
echo "  2. View logs:       $DOCKER_COMPOSE logs -f"
echo "  3. Enter Rust:      $DOCKER_COMPOSE exec rust-dev bash"
echo "  4. Enter Bun:      $DOCKER_COMPOSE exec bun-dev sh"