#!/bin/bash
# Quick development commands

# Detect docker-compose command
if command -v docker compose &> /dev/null; then
    DOCKER_COMPOSE="docker compose"
elif command -v docker-compose &> /dev/null; then
    DOCKER_COMPOSE="docker-compose"
else
    echo "❌ Error:  docker-compose not found!"
    exit 1
fi

case "$1" in
  rust)
    $DOCKER_COMPOSE exec rust-dev bash
    ;;
  bun)
    $DOCKER_COMPOSE exec bun-dev sh
    ;;
  build)
    $DOCKER_COMPOSE exec tauri-builder bun run tauri build
    ;;
  test)
    $DOCKER_COMPOSE exec test-runner cargo test
    ;;
  fmt)
    $DOCKER_COMPOSE exec rust-dev cargo fmt
    $DOCKER_COMPOSE exec bun-dev bun run format: frontend
    ;;
  lint)
    $DOCKER_COMPOSE exec rust-dev cargo clippy
    $DOCKER_COMPOSE exec bun-dev bun run lint
    ;;
  up)
    $DOCKER_COMPOSE up -d
    ;;
  down)
    $DOCKER_COMPOSE down
    ;;
  logs)
    $DOCKER_COMPOSE logs -f
    ;;
  ps)
    $DOCKER_COMPOSE ps
    ;;
  *)
    echo "Usage: ./docker/scripts/dev.sh {rust|bun|build|test|fmt|lint|up|down|logs|ps}"
    echo ""
    echo "Commands:"
    echo "  rust   - Enter Rust container"
    echo "  bun    - Enter Bun container"
    echo "  build  - Build Tauri application"
    echo "  test   - Run Rust tests"
    echo "  fmt    - Format code"
    echo "  lint   - Run linters"
    echo "  up     - Start all services"
    echo "  down   - Stop all services"
    echo "  logs   - View logs"
    echo "  ps     - List containers"
    exit 1
    ;;
esac