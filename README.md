# Rust-learning

## To run a specific example
```
cargo run -p guessing_game
```

## To run all the tests
```
cargo test
cargo test -p minigrep
```

## To format code before push
```
cargo fmt
```

## Other tips:
Recommended VsCode settings:
https://github.com/rust-lang-nursery/rustfmt
```json
{
    "workbench.colorTheme": "One Dark Pro",
    "rust.mode": null,
    "rust.rustup": {
        "nightlyToolchain": "nightly-x86_64-apple-darwin"
    },
    "window.zoomLevel": 0,
    "rust.rls.useRustfmt": true,
    "editor.formatOnSave": true,
}
```