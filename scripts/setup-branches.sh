#!/bin/bash
set -e

echo "Creating 9 pipeline branches..."

BRANCHES=("first-dev" "dev" "lint" "test" "integration" "bench" "security" "staging" "master")

for BRANCH in "${BRANCHES[@]}"; do
    git branch "$BRANCH" 2>/dev/null || echo "$BRANCH exists"
    git push -u origin "$BRANCH" 2>/dev/null || echo "$BRANCH pushed"
done

echo "✅ All branches created!"