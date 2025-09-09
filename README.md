# gdfastformat (MVP)
A minimal, **fast** GDScript formatter in Rust designed for editor on-save usage.


## Install
```bash
cargo install --path .
# or
cargo build --release
cp target/release/gdformat-rs /usr/local/bin/
```


If `tree-sitter-gdscript` fails to resolve from crates.io, switch to the git dependency in `Cargo.toml` (see comments).


## Usage
- From shell:
```bash
cat script.gd | gdformat-rs > script.gd.new
```


- In **Neovim** (on save):
```vim
" Fast, destructive whole-buffer format via STDIN/STDOUT
autocmd BufWritePre *.gd silent! keepjumps keepalt %!gdformat-rs
```


Optional flags:
```bash
gdformat-rs --indent-size 2 # use 2 spaces
gdformat-rs --no-parse-check # skip Tree-sitter validation
```


## Roadmap
- Replace heuristics with CST-guided indentation from Tree-sitter nodes
- Preserve & place comments precisely between nodes
- Spacing rules around operators, commas, colons
- Line wrapping & max width
- Config file (`.gdformat.toml`)
- `--check` mode and diff output
- Batch mode for project formatting
