# Build script for dprint-plugin-asciidoc

Write-Output --Color Green "Building WASM plugin..."
cargo build --target wasm32-unknown-unknown --release

if ($?) {
    Write-Output --Color Green "Build successful!"
    Write-Output --Color Green "WASM file: target/wasm32-unknown-unknown/release/dprint_plugin_asciidoc.wasm"
} else {
    Write-Output --Color Red "Build failed!"
    exit 1
}
