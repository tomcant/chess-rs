# chess-rs

[![CI](https://github.com/tomcant/chess-rs/actions/workflows/ci.yml/badge.svg)](https://github.com/tomcant/chess-rs/actions/workflows/ci.yml)

A UCI compatible chess engine written in Rust.

## Features

- Move generation
  - Bitboards for pseudo-legal move generation
  - Sliding piece attack generation
- Search
  - Iterative deepening
  - Negamax with alpha/beta pruning
  - Aspiration search
  - Principal variation search
  - Quiescence search
  - Transposition table with Zobrist keys
  - Move ordering heuristics
    - TT move
    - MVV/LVA
    - Killer moves
- Evaluation
  - Basic material counts
  - Piece-square tables
- Universal Chess Interface
  - Play via any UCI compatible GUI
  - Time management with `movetime` / `wtime` / `btime` / `winc` / `binc`
