## Rust on Nails

Built with the [Rust on Nails](https://rust-on-nails.com/) architecture for secure full stack web applications.

```bash
cargo install cornucopia
cd crates/db
cargo build

brew install mold
cargo install just

# test gRPC
brew install grpcui
cargo run --bin web-server
grpcui -plaintext localhost:50051
```
