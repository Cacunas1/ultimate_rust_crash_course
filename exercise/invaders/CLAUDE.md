# CLAUDE.md

This file provides guidance to Claude Code (claude.ai/code) when working with code in this repository.

## Project Overview

This is a Space Invaders game implementation in Rust using the `crossterm` library for terminal UI and `rusty_audio` for sound effects. The game is part of the [Ultimate Rust Crash Course](https://agileperception.com/ultimate_rust_crash_course) exercises.

## Build and Run Commands

```bash
# Build the project
cargo build

# Run the game
cargo run

# Build for release (optimized)
cargo build --release

# Run tests
cargo test

# Check code without building
cargo check

# Format code
cargo fmt

# Lint code
cargo clippy
```

## Project Structure

- **src/main.rs**: The main game loop and entry point. Currently contains the skeleton of a Space Invaders game with:
  - Terminal initialization (raw mode, alternate screen, cursor hiding)
  - Audio system setup with sound effects (explode, lose, move, pew, startup, win)
  - Basic game loop with input handling (ESC or 'q' to quit)
  - Cleanup on exit

- **audio/**: Directory containing WAV audio files used by the game:
  - `explode.wav`: Sound when enemies are destroyed
  - `lose.wav`: Sound when player loses
  - `move.wav`: Sound for player movement
  - `pew.wav`: Sound for shooting
  - `startup.wav`: Startup sound
  - `win.wav`: Sound when player wins

## Dependencies

- **crossterm**: Terminal manipulation library for handling raw mode, alternate screen, and input polling
- **rusty_audio**: Audio library for playing WAV files
- **rusty_time**: Time utilities (included in dependencies)

## Key Concepts for Development

### Terminal Management
The game uses `crossterm` for terminal control:
- Raw mode: Processes keyboard input without waiting for Enter
- Alternate screen: Allows the game to use a clean screen and restore the original terminal state
- Cursor hiding: Hides the cursor during gameplay

### Game Loop Structure
The game loop follows a typical pattern:
1. Poll for input events without blocking
2. Process input
3. Update game state
4. Render frame
5. Repeat

### Audio System
The `rusty_audio::Audio` struct manages sound effects. Audio files are loaded once at startup and played by name during gameplay.

## Important Notes

- The Cargo.toml specifies edition = "2024", which may need verification as this is experimental
- Audio files are loaded from relative paths starting with "audio/" directory
- The game uses non-blocking event polling to allow continuous rendering and updates
