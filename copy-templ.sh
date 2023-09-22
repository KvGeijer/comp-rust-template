#!/bin/sh

# Use from this script from anywhere to copy the relevant template files to a new project.
# Just call it with the new project location as the only argument

templ_dir=$(dirname $0)
dest_dir=$1

rsync -av --include='/Cargo.toml' --include='/bundle.sh' --include='/src/***' --exclude='*' $templ_dir/ $dest_dir/

