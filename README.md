# H+ (Hyper+) – simpler than speaking

**H+** is a tiny, fast, beginner-friendly programming language experiment built in Rust.  
Goal: make coding feel more like natural language with keywords like `say`, `when`, `use`, and `let`.

This repository now includes:

- a Rust-based H+ interpreter
- built-in package support
- VS Code syntax highlighting for `.h+` files

> Note: this is still an early MVP / learning project, but it is now more usable and easier to extend.

## Quick Demo

```powershell
cargo run -- run examples/hello.h+
```

## H+ Syntax

```hplus
use math
use text

say "Welcome to H+"
let name = "h+"

say math.add(5, 3)
say math.fib(10)
say text.upper(name)
```

## Built-in Packages

H+ currently ships with lightweight built-in packages:

- `math`
  - `math.add(a, b)`
  - `math.sub(a, b)`
  - `math.fib(n)`
- `text`
  - `text.upper(value)`
  - `text.lower(value)`
  - `text.len(value)`
- `files`
  - `files.read(path)`
  - `files.exists(path)`
- `web`
  - `web.get(url)`
  - `web.status(url)`

List them from the CLI with:

```powershell
cargo run -- packages
```

## Syntax Highlighting

A VS Code language definition is included in this repository:

- `package.json`
- `syntaxes/hplus.tmLanguage.json`
- `syntaxes/hplus-language-configuration.json`

To use it in VS Code:

1. Open this repository in VS Code.
2. Install `vsce` if you want to package the extension:
   ```powershell
   npm install -g vsce
   ```
3. Package the extension:
   ```powershell
   vsce package
   ```
4. Install the generated `.vsix` file in VS Code.

After that, `.h+` files will get syntax highlighting for:

- keywords like `say`, `use`, `when`, `otherwise`, `let`
- package calls like `math.add(1, 2)`
- strings, comments, numbers, variables, and blocks

## Why this direction?

To keep H+ simple, fast, and lightweight:

- the interpreter is written only in Rust
- packages are built-in and resolved without a heavy dependency system
- syntax stays small and readable
- tooling is minimal but practical
