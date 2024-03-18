#!/usr/bin/env bash

# START SCRIPT FOR AXUM AND YEW (Backend & Frontend) Together

echo "Starting Application backend & frontend..."
echo "Ctrl-C to exit both\n"

export SERVER_URL=http://127.0.0.1:8000

(trap 'kill 0' SIGINT; (trap 'kill 0' SIGINT; cargo -Z unstable-options -C ./ watch -c -w src -x run) & (cd frontend ; trunk serve) & wait)
