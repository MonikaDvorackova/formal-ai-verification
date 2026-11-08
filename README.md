# formal-ai-verification

Rust policy predicate and SMT counterexample checking.

## Requirements

Rust stable, Z3.

## Run

```bash
cargo test
z3 models/approval.smt2
z3 models/safe_approval.smt2
```

The intentionally unsafe model must return **sat**, demonstrating a counterexample. The guarded model must return **unsat**, demonstrating absence of a counterexample within its Boolean specification.

## Scope and limitations

This is a reference Boolean policy proof, not end-to-end verification of an AI deployment. The solver proves only properties represented by the supplied formulas. CI checks Rust tests and both Z3 results.

MIT license.

## Run without local setup

Run [CI](https://github.com/MonikaDvorackova/formal-ai-verification/actions/workflows/ci.yml) → **Run workflow** on `main`. GitHub provides the compiler, dependencies, and test environment. Local commands must be run from a cloned repository root, not your home directory.
