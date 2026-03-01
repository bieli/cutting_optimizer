# Cutting Optimizer
High‑performance rod & bar cutting optimization engine (Rust)
Cutting Optimizer is a fast, deterministic, and production‑ready engine for optimizing rod/bar cutting plans.
It is designed for manufacturing, CNC workflows, metal shops, woodworking, and any domain where material waste must be minimized.

### This project provides
- High‑performance algorithms (Rust)
- Two‑phase optimization (heuristic + backtracking)
- Minimal waste generation
- Transparent cut plans
- ASCII visualization of rods and cuts
- Unit tests included
- Extensible architecture

## Why use Cutting Optimizer?

### 1. Built for real‑world manufacturing
Most cutting optimizers are either too simplistic (greedy only) or too slow (pure backtracking).

This engine combines both approaches:
- Phase 1: Fast heuristic (Shortest‑First or custom strategies)
- Phase 2: Exact backtracking for perfect packing when possible

This gives you the best of both worlds:
speed + optimality.

### 2. Deterministic and predictable
Every run produces the same result for the same input.

This is critical for:
- CNC automation
- production repeatability
- quality control
- auditability

### 3. Zero dependencies
The core engine uses pure Rust, no external crates.

This means:
- no dependency vulnerabilities
- no version conflicts
- maximum performance
- easy embedding into other systems

### 4. Human‑readable ASCII visualization
You can instantly see how rods are used:

```bash
Rod 1: 100 mm (cuts: [60, 30, 10])
|##############################.................|
```

This is extremely useful for:
- debugging
- operator training
- production documentation

### 5. Extensible strategy system
You can plug in your own cutting strategies:

```rust
enum Strategy {
    ShortestFirst,
    LongestFirst,
    Custom(Box<dyn Fn(...) -> ...>),
}
```
This makes the engine suitable for:
- metal fabrication
- woodworking
- plastics
- composite materials
- automated saws
- CNC routers

