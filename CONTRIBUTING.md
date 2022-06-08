# Contributing

Thanks for your interest in contributing to this project! PRs and issues are welcome!

## How to contribute

First, make sure you can build and run the project

1. Ensure you have [Rust](https://www.rust-lang.org/tools/install) or [nix](https://nixos.org) installed.
2. Checkout this repo `git clone https://github.com/solfacil/photosphere-cli.git`
3. Build the source as described [here](https://github.com/solfacil/photosphere-cli#build-from-source)
4. Run the tests `cargo test`
5. Build an example `cargo run --`

You should see a friendly output on console!

## Making PRs

To make a PR follow [GitHubs guide](https://docs.github.com/en/pull-requests/collaborating-with-pull-requests/proposing-changes-to-your-work-with-pull-requests/creating-a-pull-request).

PRs are all checked with

- `cargo check`
- `rustfmt`
- `cargo clippy`
- `cargo test`

so you can run these locally to ensure CI passes.

Most PRs should include tests.
