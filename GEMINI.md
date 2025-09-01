# Project Overview

This is a modular game engine written in Rust. The engine is designed to be highly modular, with different functionalities like ECS (Entity Component System), physics, rendering, input, AI, and audio implemented as separate, interchangeable systems. The goal is to allow developers to create their "dream games" by combining and customizing these modules.

The engine uses the `specs` library for its ECS implementation and `wgpu` for rendering, although some demos use `minifb` for simpler graphics. Other key dependencies include `nalgebra` for physics calculations, `rodio` for audio, and `winit` for windowing.

# Building and Running

## Building the Engine

To build the engine library:

```bash
cargo build
```

To build in release mode for better performance:

```bash
cargo build --release
```

## Running Demos

The project includes several demos to showcase different features of the engine. Here's how to run them:

*   **ECS Demo:**
    ```bash
    cargo run --bin ecs_demo
    ```
*   **Physics Demo:**
    ```bash
    cargo run --bin physics_demo
    ```
*   **Simple Graphical Pong (Terminal-based):**
    ```bash
    cargo run --bin simple_graphical_pong
    ```
*   **Window Pong (Graphical):**
    ```bash
    cargo run --bin window_pong
    ```

For a complete list of demos, see the `[[bin]]` sections in the `Cargo.toml` file or the `RUNNING_DEMOS.md` file.

## Running Tests

To run the test suite:

```bash
cargo test
```

You can also run tests for specific systems:

```bash
cargo test physics
cargo test rendering
```

# Development Conventions

*   **Modularity:** The core principle of this project is modularity. Each major functionality is encapsulated in its own system. When adding new features, they should be implemented as separate systems if possible.
*   **System Isolation:** Systems should be as independent as possible, communicating with each other through well-defined interfaces, primarily the ECS.
*   **Demos:** Each system should have a corresponding demo that showcases its functionality in isolation.
*   **Testing:** Each system should have its own set of tests.
*   **Documentation:** The `docs` directory contains detailed documentation for each system. Any new systems or significant changes should be documented there.
