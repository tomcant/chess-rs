# chess-rs

[![CI](https://github.com/tomcant/chess-rs/actions/workflows/ci.yml/badge.svg)](https://github.com/tomcant/chess-rs/actions/workflows/ci.yml)

A UCI compatible chess engine written in Rust.

## Features

- Bitboards for pseudo-legal move generation
- Alpha/beta optimised Negamax search
- Iterative deepening with PV move ordering
- Material and piece-square table evaluation
- Play via any UCI compatible GUI

## To-do List

- PVS (principal variation search)
- Transposition table
- Move ordering with MVV/LVA and TT
- Time management/pondering
