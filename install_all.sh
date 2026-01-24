#!/bin/bash
for dir in */; do
    if [ -f "$dir/Cargo.toml" ]; then
        echo "Installing $dir ..."
        cargo install --path "$dir"
    fi
done
