# Markdown

A basic Markdown parser, written in Rust, that is intended for personal use. It does not support all of Markdown, and there are several edge cases in which the parser will output unexpected HTML, but it works for my specific use cases.

## Quick start

This project targets Rust 1.72.1. It may be possible to compile this project on an older version of Rust and still have it work as intended, though there are no guarantees.

```
cargo build --release

# Specify an input file and an output file
target/release/markdown input.md -o output.html

# Pipe input through stdin
cat input.md | target/release/markdown -o output.html

# Get output through stdout
cat input.md | target/release/markdown | less
```

## License

Check [LICENSE](LICENSE) for licensing information.
