rust_bundler_cp --input . > bundled.rs
cat bundled.rs | pbcopy
echo "Bundled file written to bundled.rs, and copied to clipboard"
