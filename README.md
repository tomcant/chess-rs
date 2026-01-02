<div align="center">

  ![Anodos logo][./anodos.png]

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
  - Futility pruning
  - Principal variation search
  - Quiescence search
  - Check extension
  - Transposition table with Zobrist keys
  - Move ordering heuristics
    - TT move
    - MVV/LVA
    - Killer moves
- Evaluation
  - Basic material counting
  - Piece-square tables
- Universal Chess Interface
  - Play via any UCI-compatible GUI (e.g. Cute Chess, En Croissant)
  - Time management with `movetime` / `wtime` / `btime` / `winc` / `binc`

## Roadmap

- Search
  - Reverse futility pruning
  - Late move reductions
  - History heuristic
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

## Additional Commands

Beyond the UCI protocol, the engine supports these debugging/utility commands:

| Command | Description |
|---------|-------------|
| `perft <depth>` | Run perft to validate move generation |
| `printboard` | Display the current position |
| `printfen` | Output the current position as a FEN string |
| `domove <move>` | Make a move on the current position (e.g., `domove e2e4`) |


[build-link]: https://github.com/tomcant/chess-rs/actions/workflows/ci.yml
[build-badge]: https://img.shields.io/github/actions/workflow/status/tomcant/chess-rs/ci.yml?style=for-the-badge&branch=main&logo=github

[latest-link]: https://github.com/tomcant/chess-rs/releases/latest
[latest-badge]: https://img.shields.io/github/v/release/tomcant/chess-rs?style=for-the-badge&label=latest%20release

[downloads-badge]: https://img.shields.io/github/downloads/tomcant/chess-rs/total?style=for-the-badge&color=blue

[fancy-magic-link]: https://www.chessprogramming.org/Magic_Bitboards#Fancy
