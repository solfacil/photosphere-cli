# Photosphere CLI

The Photosphere CLI is a command-line interface tool that you use to initialize, develop, scaffold, and maintain [Photosphere](https://github.com/solfacil/photosphere) applications directly from a command shell.

## Install

You can download our pre-built binaries from [releases](https://github.com/solfacil/photosphere-cli/releases) page.

### Build from source

`cargo` is needed to build source code as
```sh
cargo build --bin photosphere -r
```

Then binary will be available at `/target/release/photosphere`

Or you can build with [nix](https://nixos.org/) as
```sh
nix build .#packges.$TARGET.photosphere
```

Where `$TARGET` is your machine hardware name. Windows isn't supported, see []() for supported archs.

Then binary will be available at `/result/bin/photosphere`

## Setting up a project

_TODO_
