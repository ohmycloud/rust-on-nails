watch:
    cargo watch -w crates/web-pages -w crates/web-server -w crates/db --no-gitignore -x "run --bin web-server"
tailwind:
    cd crates/web-assets && tailwind-extra -i ./input.css -o ./dist/tailwind.css --watch
