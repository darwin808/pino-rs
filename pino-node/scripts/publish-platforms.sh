#!/bin/bash
# Publish all platform packages to npm
# Run this after artifacts have been moved to npm/ directories

set -e

cd "$(dirname "$0")/.."

for dir in npm/*/; do
  if [ -d "$dir" ]; then
    # Check if .node file exists
    node_file=$(ls "$dir"/*.node 2>/dev/null || true)
    if [ -n "$node_file" ]; then
      echo "Publishing $dir"
      (cd "$dir" && npm publish --access public) || echo "Warning: Failed to publish $dir (may already exist)"
    else
      echo "Skipping $dir - no .node file found"
    fi
  fi
done

echo "Done publishing platform packages"
