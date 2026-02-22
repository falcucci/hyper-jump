# hyper-jump

hyper-jump is a small, cross-platform version manager for a fixed set of
command-line tools. it downloads release binaries, installs them under the
hyper-jump data dir and gives you a single command to manage versions.

this project is intentionally narrow. it does one job and keeps the surface
area small.

## motivation

i needed a small, predictable tool for a handful of binaries that ship on github releases. nix pkgs, homebrew and friends are fine for stable apps, but they tend to lag behind upstream tags and i didn't want to maintain overlays or wait for packaging updates.

i also wanted to handle the usual setup effort: nix's steep learning curve, inconsistent idioms and rough discoverability, plus brew's dependency resolution churn, disk bloat and periodic support or sudo friction on older macs. a single binary plus a private data dir was the simplest thing that stayed out of the way.

i avoided nix packages because they pull you into a full ecosystem with its own workflows and that’s more ceremony than this problem needed. even if you like nix, it’s a hard sell for teams who just want a binary to exist and a version to be pinned without retraining or extra concepts. hyper-jump keeps versions side by side, pulls directly from upstream releases and lets you switch instantly without touching system state.

## what this is

- a practical version manager for the supported packages listed below
- built for daily use rather than general-purpose plugin ecosystems
- a single cli with explicit subcommands and predictable paths

## what this is not

- a universal version manager for arbitrary tools
- a long-running background service
- a plugin framework

## installation

```bash
cargo install hyper-jump
```

## configuration

make sure the install directory is on your `PATH`.

```bash
export PATH="$(hj prefix):$PATH"
```

on macos, you can also set the user path once:

```bash
sudo launchctl config user path "$(hj prefix):${PATH}"
```

## usage

run `hj --help` if you can't remember the subcommands.

quick start

```sh
hj list-remote reth
hj install reth v1.10.2
hj use reth v1.10.2
hj list reth
```

commands

- `hj install <package> <version|latest>` install a version
- `hj use <package> <version|latest>` switch to a version and mark it as used
- `hj list <package>` show installed versions
- `hj list-remote <package>` show remote versions
- `hj uninstall <package> <version>` remove a version
- `hj erase` remove all installed versions
- `hj prefix` print the bin dir used for shims

notes

- `version` accepts tags like `v1.10.2` or `latest`
- `--output-format json|table` or `HYPER_JUMP_OUTPUT_FORMAT` changes list output format
- `--root-dir <path>` or `HYPER_JUMP_ROOT_DIR` overrides the data dir
- `HYPER_JUMP_PACKAGES_FILE` points to a custom `packages.toml`
- make sure the path from `hj prefix` is on your `PATH` or nothing will run

## supported packages

these come from `packages.toml`. by default the binary uses the embedded list,
or you can drop a file at `<root_dir>/packages.toml` to override it.

- [neovim](https://github.com/neovim/neovim)
- [jujutsu](https://github.com/jj-vcs/jj)
- [zellij](https://github.com/zellij-org/zellij)
- [reth](https://github.com/paradigmxyz/reth)
- [cardano node](https://github.com/IntersectMBO/cardano-node)
- [cardano cli](https://github.com/cardano-scaling/cardano-cli)
- [partner chains node](https://github.com/input-output-hk/partner-chains)
- [cardano submit api](https://github.com/IntersectMBO/cardano-node)
- [sidechain cli](https://github.com/input-output-hk/partner-chains-smart-contracts)
- [mithril client](https://github.com/input-output-hk/mithril)
- [scrolls](https://github.com/txpipe/scrolls)
- [oura](https://github.com/txpipe/oura/tree/main)
- [dolos](https://github.com/txpipe/dolos)
- [aiken](https://github.com/aiken-lang/aiken)

## potential features

- update notifications for new releases
- custom package sources
