# Photosphere CLI

The Photosphere CLI is a command-line interface tool that you use to initialize, develop, scaffold, and maintain [service-template](https://github.com/solfacil/service-template) applications directly from a command shell.

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

With Photosphere CLI installed and at you `$PATH` you can safelly:
```sh
photosphere --ssh service new <service_name>
```

Where `--ssh` is an optional flag, as the default clone method is `HTTP` and `<service_name>` is
the path of your new service :D.

## Why "Photosphere"?

"Photosphere" is the deepest part of the Sun (internal) which can be directly oberserved (external) with visible light.
That's creates the idea of turning some internal concept into a external/public/visible one.

Photosphere CLI does exactly this! Transform our [service-template](https://github.com/solfacil/service-template) (internal) to your new service (external)!
