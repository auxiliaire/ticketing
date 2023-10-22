#!/usr/bin/env bash

# START SCRIPT FOR AXUM AND YEW (Backend & Frontend) Together

echo "Starting Application backend & frontend..."
echo "Ctrl-C to exit both\n"

(trap 'kill 0' SIGINT; (trap 'kill 0' SIGINT; cargo -Z unstable-options -C ./ watch -c -w src -x run) & (cd frontend ; trunk serve) & wait)
