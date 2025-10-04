# chess-rs

[![CI](https://github.com/tomcant/chess-rs/actions/workflows/ci.yml/badge.svg)](https://github.com/tomcant/chess-rs/actions/workflows/ci.yml)

A UCI compatible chess engine written in Rust.

## Features

- Bitboards for pseudo-legal move generation
- Alpha/beta optimised Negamax search
- Transposition table with 64-bit Zobrist keys
- Iterative deepening with PV/TT move ordering
- Material and piece-square table evaluation
- Play via any UCI compatible GUI

## To-do List

- PVS (principal variation search)
- Move ordering with MVV/LVA
- Time management/pondering
