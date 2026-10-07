# Azurite

Azurite is an interpreted programming language.
It currently includes a lexer, parser, AST, runtime environment, interpreter, and a CLI (`az`).

## Workspace layout

- `crates/lexer` - turns source code into tokens
- `crates/parser` - builds an AST from tokens
- `crates/ast` - shared AST and operator definitions
- `crates/environment` - runtime values and scoped variable environment
- `crates/interpreter` - evaluates AST nodes into runtime values
- `crates/cli` - binary entrypoint and REPL

## Prerequisites

- Rust toolchain (stable)
- Cargo

## Getting started

Build the workspace:

```bash
cargo build --release
```

Run tests:

```bash
cargo test
```

Start the REPL:

```bash
cargo run -p cli
```

Exit with:

```text
exit
```

## Running a file

Pass a source file path to the CLI:

```bash
cargo run -p cli -- test.az
```

or if you're using the binary directly:

```bash
az test.az
```

At the moment, file mode prints the parsed AST. REPL mode evaluates expressions.

## Current language support

- Numeric literals
- Identifiers
- Binary operators: `+`, `-`, `*`, `/`, `%`
- Parenthesized expressions
- Variable declarations with `let` and `const`

Example:

```az
let x = 4;
let y = 45 * (4 / 3);
x + y
```

Note: declarations currently require a trailing semicolon.

## Roadmap

- [x] Assignment and variable reassignment rules (`const` enforcement)
- [x] String and boolean literals
- [ ] Better CLI output for file mode (evaluate program output instead of only printing AST)
- [ ] Unary operators (like negation)
- [ ] Comparison and logical operators
- [ ] Conditional control flow (`if`/`else`)
- [ ] Functions and function calls
- [ ] Better error messages with source locations
