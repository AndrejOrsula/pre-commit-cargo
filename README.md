# pre-commit-cargo

A set of pre-commit hooks using [`cargo`](https://doc.rust-lang.org/cargo/) for your Rust projects.

## Example `.pre-commit-config.yaml`

```yaml
repos:
  - repo: https://github.com/AndrejOrsula/pre-commit-cargo
    rev: 0.3.0
    hooks:
      - id: cargo-fmt
      - id: cargo-update
      - id: cargo-clippy
        args: ["--all-targets", "--all-features"]
      - id: cargo-check
        args: ["--all-targets", "--all-features"]
      - id: cargo-test
        args: ["--all-targets", "--all-features"]
      - id: cargo-test-doc
        args: ["--all-features"]
      - id: cargo-doc
        args: ["--no-deps", "--document-private-items"]
      - id: cargo-miri-test
      - id: cargo-miri-run
      - id: cargo-deny-check
```
