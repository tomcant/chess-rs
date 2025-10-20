# chess-rs

[![CI](https://github.com/tomcant/chess-rs/actions/workflows/ci.yml/badge.svg)](https://github.com/tomcant/chess-rs/actions/workflows/ci.yml)

A UCI compatible chess engine written in Rust.

## Features

- Bitboards for pseudo-legal move generation
- Iterative deepening with alpha/beta optimised Negamax search
- Transposition table with 64-bit Zobrist keys
- TT move ordering combined with the MVV/LVA and Killer heuristics
- Material and piece-square table evaluation
- Play via any UCI compatible GUI
