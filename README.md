# chess-rs

[![CI](https://github.com/tomcant/chess-rs/actions/workflows/ci.yml/badge.svg)](https://github.com/tomcant/chess-rs/actions/workflows/ci.yml)

A UCI compatible chess engine written in Rust.

## Features

- Move generation
  - Bitboards for pseudo-legal move generation
  - [Fancy magic](https://www.chessprogramming.org/Magic_Bitboards#Fancy) sliding piece attacks
- Search
  - Iterative deepening
  - Aspiration search
  - Negamax with alpha/beta pruning
  - Null-move pruning
  - Principal variation search
  - Quiescence search
  - Check extension
  - Transposition table with Zobrist keys
  - Move ordering heuristics
    - TT move
    - MVV/LVA
    - Killer moves
- Evaluation
  - Basic material counts
  - Piece-square tables
- Universal Chess Interface
  - Play via any UCI compatible GUI (e.g. Cute Chess, En Croissant)
  - Time management with `movetime` / `wtime` / `btime` / `winc` / `binc`
