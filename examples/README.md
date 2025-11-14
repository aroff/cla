# Examples

This directory contains example programs demonstrating how to use `rustcla`.

## Running Examples

To run an example, use:

```bash
cargo run --example <example_name>
```

## Available Examples

### `basic_usage`

Demonstrates basic usage of the Critical Line Algorithm to compute the efficient frontier.

**Run with:**
```bash
cargo run --example basic_usage
```

**What it does:**
- Creates a portfolio optimization problem with 10 assets
- Computes the efficient frontier using CLA
- Finds the maximum Sharpe ratio portfolio
- Prints frontier statistics (returns range, volatility range)

This example matches the basic usage example from the Python `cvxcla` README.

### `minvar`

Demonstrates how to find the minimum variance portfolio using the Critical Line Algorithm.

**Run with:**
```bash
cargo run --example minvar
```

**What it does:**
- Creates a portfolio optimization problem with 3 assets
- Computes the efficient frontier using CLA
- Finds the portfolio with minimum variance (lowest risk)
- Verifies the variance calculation

### `unconstrained`

Demonstrates different portfolio selections along the efficient frontier based on risk preferences.

**Run with:**
```bash
cargo run --example unconstrained
```

**What it does:**
- Shows different points along the efficient frontier
- Displays maximum return portfolio (risk-seeking)
- Displays minimum variance portfolio (risk-averse)
- Computes maximum Sharpe ratio portfolio (optimal risk-adjusted return)

### `plot_frontier`

Demonstrates visualization of the efficient frontier using the `plotters` crate.

**Run with:**
```bash
cargo run --example plot_frontier --features plotting
```

**What it does:**
- Creates a portfolio optimization problem
- Computes and interpolates the efficient frontier
- Generates two plots:
  - `frontier_volatility.png`: Return vs. Volatility
  - `frontier_variance.png`: Return vs. Variance

**Note:** Requires the `plotting` feature to be enabled.

## Building Examples

All examples are built as part of the regular build process:

```bash
cargo build --examples
```

## Example Development

When adding new examples:

1. Create a new file `examples/<name>.rs`
2. Add a `main()` function
3. Include documentation comments explaining what the example demonstrates
4. Update this README with a description of the new example
5. Ensure the example compiles and runs successfully

