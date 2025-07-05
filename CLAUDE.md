# CLAUDE.md

This file provides guidance to Claude Code (claude.ai/code) when working with code in this repository.

## Project Structure

This is a Rust workspace containing multiple Bevy engine crates and games. The project is organized as:

- `crates/` - Core library crates
- `games/` - Game applications
- Uses Bevy 0.16 throughout

## Key Crates

### Core Engine Crates
- **raven_bvh**: Bounding Volume Hierarchy implementation for spatial queries and ray casting
- **raven_nav**: Navigation mesh generation and pathfinding system using recast-style tile-based approach
- **raven_terrain**: Infinite terrain generation using quadtree LOD with noise-based heightmaps
- **raven_editor**: Development tools and inspector UI with physics debugging
- **raven_util**: Shared utilities and helper functions

### Games
- **rts**: Real-time strategy game with units, buildings, and AI
- **minesweeper**: Classic minesweeper implementation
- **tic-tac-toe**: Simple tic-tac-toe game
- **lab**: Experimental sandbox environment for testing features

## Common Commands

### Build and Run
```bash
# Build entire workspace
cargo build

# Run specific examples
cargo run --bin rts
cargo run --bin lab
cargo run --example basic --package raven_terrain
cargo run --example sponza --package raven_bvh --features="camera,debug_draw"
cargo run --example agent --package raven_nav --features="debug_draw"

# Run with optimizations (better performance)
cargo run --release --bin rts
```

### Testing
```bash
# Run all tests
cargo test

# Run tests for specific crate
cargo test --package raven_nav
```

### Benchmarks
```bash
# Run benchmarks (available in raven_bvh and raven_nav)
cargo bench --package raven_bvh
cargo bench --package raven_nav
```

## Architecture Overview

### Feature System
Most crates use conditional compilation features:
- `debug_draw` - Enables gizmo visualization
- `camera` - Adds camera components for examples
- `trace` - Enables tracing/profiling
- `avian3d` - Physics integration

### Component Architecture
- Uses Bevy's ECS with required components pattern (edition 2024)
- Heavily uses `GlobalTransform` for world-space calculations
- Implements custom spatial data structures (BVH, quadtree, nav mesh)

### Key Systems
- **BVH**: Spatial acceleration structure for ray casting and collision detection
- **Navigation**: Tile-based nav mesh generation with dynamic obstacle avoidance
- **Terrain**: Infinite terrain using quadtree LOD with noise generation
- **Editor**: Runtime inspection and debugging tools

### Performance Considerations
- Uses `AsyncComputeTaskPool` for expensive operations (nav mesh generation)
- Implements spatial partitioning for efficient queries
- Profile config optimizes debug builds for dependencies while keeping user code unoptimized

## Development Notes

### Dependencies
- Uses `avian3d` for physics (version 0.3)
- `bevy-inspector-egui` for runtime inspection
- `noisy_bevy` for procedural noise generation
- `rand` ecosystem for random number generation

### Code Patterns
- Extensive use of `Query` filters with `Changed<T>` for efficient updates
- Custom `prelude` modules in each crate expose common types
- Uses `Local<T>` resources for system-local state
- Implements `Reflect` trait for runtime inspection

### Editor Integration
- Press backtick (`) to toggle editor UI
- F-keys toggle various debug visualizations
- Physics debugging with F1, wireframe with F6
- AABB visualization with F3