#!/bin/bash

set -e

if [ $# -ne 1 ]; then
    echo "Usage: $0 <version>"
    echo "Example: $0 0.0.0"
    exit 1
fi

VERSION=$1

if git rev-parse "v$VERSION" >/dev/null 2>&1; then
    echo "Error: Tag v$VERSION already exists"
    exit 1
fi

echo "Updating Cargo.toml to version $VERSION..."
sed -i 's/^version = ".*"/version = "'"$VERSION"'"/' Cargo.toml

echo "Building project and updating Cargo.lock..."
cargo build --release

echo "Committing version bump..."
git add Cargo.toml Cargo.lock
git commit -m "chore(release): bump version to $VERSION"

echo "Creating tag v$VERSION..."
git tag -a "v$VERSION" -m "chore(release): v$VERSION"

echo "Pushing commits and tags to origin..."
CURRENT_BRANCH=$(git branch --show-current)
git push origin "$CURRENT_BRANCH" "v$VERSION"

echo "Successfully created and pushed release v$VERSION"