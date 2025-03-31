
watch:
    mold -run cargo watch --workdir /workspace/ -w crates/web-server -w crates/db -w crates/web-pages --no-gitignore -x "run --bin web-server"

tailwind:
    cd /workspace/crates/web-assets && tailwind-extra -i ./input.css -o ./dist/tailwind.css --watch

wasm:
    cd /workspace/crates/web-csr && wasm-pack build --target web --out-dir dist
