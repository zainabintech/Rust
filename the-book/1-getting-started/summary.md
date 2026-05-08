# Unit 1 — Getting Started (Summary)

A quick-reference cheat sheet of everything from 1.1 → 1.3.

## All commands

### Managing Rust itself (`rustup`)
```
rustup                       # the Rust version manager
rustup update                # update to the newest Rust version
rustup self uninstall        # uninstall Rust and rustup
rustup doc                   # open the offline Rust documentation
```

### The compiler (`rustc`)
```
rustc --version              # check Rust is installed
rustc main.rs                # compile a single .rs file → executable
.\main                       # run the executable (Windows)
./main                       # run the executable (Mac/Linux)
```

### Cargo — the build tool & package manager
```
cargo --version              # check Cargo is installed
cargo new <name>             # create a new Cargo project (creates folder)
cargo init                   # convert the current folder into a Cargo project
cargo new <name> --vcs=git   # force git init even inside an existing repo
cargo add <crate>            # add a dependency (e.g. cargo add rand@0.8.5)
cargo build                  # compile (debug build → target/debug/)
cargo build --release        # compile with optimizations → target/release/
cargo run                    # build + run in one step
cargo check                  # check it compiles, but don't produce an executable
--offline                    # flag: use cached crates instead of downloading
```

---

## Key points

### Files & naming
- Rust source files end in **`.rs`**.
- Multi-word filenames use **underscores**: `hello_world.rs` (not `helloworld.rs`).
- Cargo expects source files inside the **`src/`** folder.

### The `main` function
- Every executable Rust program **must** have a function named `main`.
- `main` is the entry point — the first code that runs.
- Bodies are wrapped in `{}`.

### Macros vs functions
- `println!`, `print!`, `format!`, `vec!`, `dbg!` are **macros** (note the `!`).
- A macro is a *template* — Rust expands it into real code at compile time.
- A function runs at runtime.
- `!` after a name = macro, no `!` = function.

### Syntax basics
- `;` ends a statement (like a period).
- `"..."` is a string.
- An **argument** is a value you pass to a function/macro.
- `//` starts a line comment.

### Compiling vs running
- `rustc` (or `cargo build`) **compiles** — it does **not** run the program.
- Running the executable is a separate step (`.\main`, `cargo run`).
- Rust is **ahead-of-time (AOT) compiled** → produces a standalone binary.
- The binary runs on its own; the user does **not** need Rust installed.
- Contrast: Python/Ruby/JS need their runtime installed on the user's machine.

### `rustc` vs `rustup`
- **`rustup`** = manages Rust itself (install/update/uninstall versions).
- **`rustc`** = the actual compiler that turns `.rs` files into executables.

### Cargo basics
- **Cargo = build system + package manager**, bundled into one tool.
- Two jobs: (1) build your code, (2) manage dependencies.
- A **package** = your project. **Dependencies** = libraries your project uses.
- Libraries are called **crates** in Rust.

### `Cargo.toml`
- Project config file in TOML format.
- `[package]` — name, version, edition.
- `[dependencies]` — list of crates your project needs.

### `Cargo.lock`
- Auto-generated on first `cargo build`.
- Tracks the **exact versions** of dependencies.
- **Don't edit it manually** — Cargo manages it.

### Build profiles
| | `cargo build` (debug) | `cargo build --release` |
|---|---|---|
| Output folder | `target/debug/` | `target/release/` |
| Compile speed | fast | slow |
| Run speed | slower | fast |
| Use for | development | shipping |

### `cargo check` habit
- Faster than `cargo build` (skips producing an executable).
- Run it often while writing code to confirm it still compiles.
- Run `cargo build` only when you actually need the binary.

### Working on an existing Cargo project
```
git clone <repo>
cd <project>
cargo build
```
