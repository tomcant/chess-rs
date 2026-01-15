<div align="center">

  ![Anodos logo](./anodos.png)

  <h1>Anodos – Chess Engine</h1>

  [![Build][build-badge]][build-link]
  [![Release][latest-badge]][latest-link]
  ![Downloads][downloads-badge]

</div>

## Overview

Anodos is a UCI-compatible chess engine written in Rust. Built from scratch with bitboard-based move generation and alpha-beta optimised search.

## Features

- Move generation
  - Bitboards for pseudo-legal move generation
  - [Fancy magic][fancy-magic-link] sliding piece attacks
- Search
  - Iterative deepening
  - Aspiration windows
  - Negamax with alpha/beta pruning
  - Null-move pruning
  - Extended/reverse futility pruning
  - Principal variation search
  - Quiescence search
  - Check extension
  - Transposition table with Zobrist keys
  - Move ordering
    - TT move
    - MVV/LVA
    - Killer moves
    - History heuristic
- Evaluation
  - Basic material counting
  - Piece-square tables
- Universal Chess Interface
  - Play via any UCI-compatible GUI (e.g. Cute Chess, En Croissant)
  - Time management with `movetime` / `wtime` / `btime` / `winc` / `binc`

## Roadmap

- Search
  - Late move reductions
  - Counter-move heuristic
  - Static exchange evaluation
  - Multi-threading
- Evaluation
  - Tapered PSQTs by game phase
  - Pawn structure, king safety, piece activity
  - Insufficient material draw detection
  - Syzygy tablebase support
- Testing
  - Super-fast perft with a dedicated transposition table

## Universal Chess Interface

Supported commands:

```
uci
isready
ucinewgame
position startpos [moves ...]
position fen <fen> [moves ...]
go [infinite]
go depth <n>
go nodes <n>
go movetime <ms>
go wtime <ms> btime <ms> [winc <ms>] [binc <ms>]
setoption name Hash value <MB>
stop
quit
```

## Non-standard Commands

Beyond the UCI protocol, the engine supports these debugging/utility commands:

| Command | Description |
|---------|-------------|
| `perft <depth>` | Run perft to validate move generation |
| `printboard` | Display the current position |
| `printfen` | Output the current position as a FEN string |
| `domove <move>` | Make a move on the current position (e.g., `domove e2e4`) |

## Compilation

This project uses Rust edition 2024, so you'll need Rust 1.85+ (stable).

### Building and Running

For the best performance, always use release mode:

```sh
cargo build --release
./target/release/anodos
```

Alternatively, you can run in release mode directly:

```sh
cargo run --release
```

To enable debug assertions (useful during development for catching bugs), use:

```sh
cargo run
```

Only use debug mode for testing and development; release mode is strongly recommended for actual games or benchmarks.

### PGO (Profile-guided Optimisation)

Binaries published on the [Releases][releases-link] page are built with PGO (see [.github/workflows/cd.yml][cd.yml-link]).

To run a local PGO build:

```sh
rustup component add llvm-tools-preview
cargo install cargo-pgo

# Build an instrumented binary
cargo pgo build

# Collect profiles by running the benchmark command
cargo pgo run -- bench

# Build the optimised (PGO) binary
cargo pgo optimize
```

## Testing

Run the test suite:

```sh
cargo test
```

This includes a shallow perft check from the starting position (depth 5).

To run the deeper perft test suite (ignored by default), include ignored tests and use release mode:

```sh
cargo test --release -- --include-ignored
```

## Benchmarking

To measure the engine's nodes-per-second performance, run the binary as follows:

```sh
./anodos bench [--depth <DEPTH>] [--tt-mb <MB>]
```

- `--depth` (default: 12) sets the search depth for each position
- `--tt-mb` (default: 64) sets the transposition table size in MB


[build-link]: https://github.com/tomcant/anodos/actions/workflows/ci.yml
[build-badge]: https://img.shields.io/github/actions/workflow/status/tomcant/anodos/ci.yml?style=for-the-badge&branch=main&logo=github

[latest-link]: https://github.com/tomcant/anodos/releases/latest
[latest-badge]: https://img.shields.io/github/v/release/tomcant/anodos?style=for-the-badge&label=latest%20release

[downloads-badge]: https://img.shields.io/github/downloads/tomcant/anodos/total?style=for-the-badge&color=blue

[fancy-magic-link]: https://www.chessprogramming.org/Magic_Bitboards#Fancy

[releases-link]: https://github.com/tomcant/anodos/releases
[cd.yml-link]: https://github.com/tomcant/anodos/blob/main/.github/workflows/cd.yml
