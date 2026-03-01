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


## How it works
### Phase 1 — Heuristic planning
Fast greedy strategy (Shortest‑First by default) attempts to pack cuts into rods.

### Phase 2 — Backtracking
If phase 1 fails or leaves excessive waste, the engine switches to an exact solver:

- tries all valid combinations
- guarantees optimal packing when possible
- returns a full cut plan

## Example usage

### How to run
```bash
$ cargo run -- --rods "100x2,40x2,10x3" --cuts "50, 40, 30, 20, 10, 15" --dp-length 100 --visualize
```
### Example Output

```bash
=== CUTTING REPORT ===
Optimal cut order: [50, 40, 30, 20, 15, 10]
Used rods:
  Rod 10 mm used (leftover 0 mm)
  Rod 40 mm used (leftover 0 mm)
  Rod 40 mm used (leftover 10 mm)
  Rod 100 mm used (leftover 15 mm)
Offcuts: [10, 15]
Total waste: 25 mm

=== ASCII VISUALIZATION ===

Cuts in order:
Cut  1:   50 mm |#########################
Cut  2:   40 mm |####################
Cut  3:   30 mm |###############
Cut  4:   20 mm |##########
Cut  5:   15 mm |#######
Cut  6:   10 mm |#####

=== RODS USED ===

Rod 1: 10 mm (cuts: [10])
|#####|

Rod 2: 40 mm (cuts: [40])
|####################|

Rod 3: 40 mm (cuts: [30])
|###############.....|

Rod 4: 100 mm (cuts: [50, 20, 15])
|##########################################........|

```


## Storage‑Aware Workflow
Reusing leftovers across multiple cutting jobs
One of the defining features of this project is its storage‑aware cutting strategy.

Unlike traditional rod‑cutting optimizers that treat each job independently, this engine:
- tracks exact rods used,
- reports leftover lengths,
- exposes offcuts as reusable inventory,
- and allows you to feed those leftovers directly into the next optimization run.

This creates a continuous, real‑world workflow where material is reused across multiple production cycles, minimizing waste and reducing cost.

### Multi‑Job Optimization Loop
Every time you run the optimizer, the terminal output includes:

- a list of used rods,
- a list of offcuts,
- the total waste,
- and a cut plan.

Example output:

```bash
Used rods:
  Rod 100 mm used (leftover 10 mm)
  Rod 100 mm used (leftover 40 mm)
  Rod 100 mm used (leftover 55 mm)
  Rod 100 mm used (leftover 90 mm)

Offcuts: [10, 40, 55, 90]
Total waste: 195 mm
```

These offcuts represent your new stock for the next job.

### Step 1 — Convert leftovers into rod definitions
Take the Offcuts: list:

```bash
[10, 40, 55, 90]
```
Convert it into the rod format used by the CLI:

```bash
10x1,40x1,55x1,90x1
```
This becomes your new --rods argument.

### Step 2 — Run the next job using leftover stock
If your next job requires cuts:

```bash
20, 15, 10
```
Run:

```bash
cargo run -- --rods "90x1,55x1,40x1,10x1" --cuts "20,15,10"
```
The engine will:
- reuse your leftover rods first,
- avoid cutting fresh stock unless necessary,
- and minimize waste across jobs, not just within a single job.

### Why this matters
Most cutting optimizers assume:
- infinite stock,
- no leftovers,
- or that each job is independent.

This project models a real workshop, where:
- leftovers accumulate,
- storage matters,
- and material cost is significant.

The storage‑aware workflow helps you:
- reduce waste across multiple jobs,
- track inventory automatically,
- plan production more efficiently,
- and save money on material.
