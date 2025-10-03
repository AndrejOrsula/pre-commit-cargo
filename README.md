# pre-commit-cargo

A set of pre-commit hooks using [`cargo`](https://doc.rust-lang.org/cargo/) for your Rust projects.

## Example `.pre-commit-config.yaml`

```yaml
repos:
  - repo: https://github.com/AndrejOrsula/pre-commit-cargo
    rev: 0.4.0
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
      - id: cargo-fix
        args: ["--allow-dirty", "--allow-staged"]
      - id: cargo-miri-test
      - id: cargo-miri-run
      - id: cargo-deny-check
```

## Only running `cargo update` once every so often

> [!NOTE]
> This feature is not yet available in a stable version of pre-commit-cargo. You’ll need to use an unstable version of pre-commit-cargo in order to access this feature.

By default, the `cargo-update` pre-commit hook will always run `cargo update` no matter what. If you have many dependencies, then this can result in your `Cargo.lock` file being updated too frequently. In order to avoid this problem, you can use the `--cutoff` option to make the `cargo update` pre-commit hook only run `cargo update` once a week:

```yaml
repos:
  - repo: https://github.com/AndrejOrsula/pre-commit-cargo
    rev: <commit hash>
    hooks:
      - id: cargo-update
        args: ["--cutoff", "1 week ago"]
```

For more information about how the `--cutoff` option works, run `cargo run -- --help` from the root of this repository.
