# Star Fee Routing - Meteora DLMM V2 Integration

A permissionless fee routing Anchor program for Meteora DLMM V2 that manages honorary quote-only fee positions and distributes fees to investors based on their locked token amounts from Streamflow.

## 🎯 Overview

This program implements a two-part system:

1. **Work Package A**: Initialize honorary fee positions that only accrue quote-mint fees
2. **Work Package B**: Permissionless 24-hour distribution crank that distributes fees proportionally to investors

## Getting Started

Click the [`Use this template`](https://github.com/PaulRBerg/rust-template/generate) button at the top of the page to
create a new repository with this repo as the initial state.

## Features

### Sensible Defaults

This template comes with sensible default configurations in the following files:

```text
├── .editorconfig
├── .gitignore
├── .prettierrc.yml
├── Cargo.toml
├── justfile
└── rustfmt.toml
```

### GitHub Actions

This template comes with GitHub Actions pre-configured. Your code will be linted and tested on every push and pull
request made to the `main` branch.

You can edit the CI script in [.github/workflows/ci.yml](./.github/workflows/ci.yml).

## Usage

See [The Rust Book](https://doc.rust-lang.org/book/) and [The Cargo Book](https://doc.rust-lang.org/cargo/index.html).

## License

This project is licensed under MIT.
