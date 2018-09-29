# rt-rs

> A ray tracer written in Rust, following Physically Based Rendering.

## Install

```
cargo install
```

## Run

```
./scripts/run.sh scenes/stuff.scene
```

## Important Basics

- rt-rs uses a right-handed coordinate system, with +Y pointing up and +Z pointing out of the screen.
- Front-face winding order is therefore counter-clockwise.
