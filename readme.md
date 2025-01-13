<div align="center">

# hyper-jump

_The console lights up, keys clack rapidly and then..._ 🛸

</div>

hyper-jump is an agnostic all-in-one and cross-platform command-line version manager toolset which I have created and used personally based on internal tools I daily interact with for many different reasons, that being for educational purposes or as a professional project/task.

## Core Features

- **Version Management**: Install and switch between different versions of Cardano ecosystem tools.
- **Environment Isolation**: Create isolated environments for different projects, each with its own set of package versions.
- **Remote Listing**: View available versions of packages from remote repositories.
- **Package Installation**: Download and install specific versions of packages with ease.
- **Package Uninstallation**: Remove installed versions of packages to declutter the environment.
- **Version Switching**: Seamlessly switch between installed versions with a single command.
- **Version Cleanup**: Erase all installed versions of packages to start fresh.
- **Proxy Handling**: Hyper-Jump acts as a proxy, allowing users to run commands from the selected package version.

## Potential Features

- **Updates notification**: Notify users when new versions of packages are available.
- **Custom Package Sources**: Allow users to add custom package sources or repositories for more flexibility.
- **Enhanced List Filtering**: Provide options to filter the list of installed and remote versions based on criteria such as release date or stability.

## Supported Packages

| Package Name                                                                   | Alias                 | Description                                          |
| ------------------------------------------------------------------------------ | --------------------- | ---------------------------------------------------- |
| [Zellij](https://github.com/zellij-org/zellij)                                 | `zellij`              | Manage versions of the Zellij client software.       |
| [Reth](https://github.com/paradigmxyz/reth)                                    | `reth`                | Manage versions of the Reth client software.         |
| [Cardano Node](https://github.com/IntersectMBO/cardano-node)                   | `cardano-node`        | Manage versions of the Cardano Node software.        |
| [Cardano CLI](https://github.com/cardano-scaling/cardano-cli)                  | `cardano-cli`         | Manage versions of the Cardano CLI tool.             |
| [Partner Chains CLI](https://github.com/input-output-hk/partner-chains)        | `partner-chains-cli`  | Manage versions of the Partner Chains CLI tool.      |
| [Partner Chains Node](https://github.com/input-output-hk/partner-chains)       | `partner-chains-node` | Manage versions of the Partner Chains Node software. |
| [Cardano Submit Api](https://github.com/IntersectMBO/cardano-node)             | `cardano-submit-api`  | Manage versions of the Cardano Submit Api software.  |
| [SideChain](https://github.com/input-output-hk/partner-chains-smart-contracts) | `sidechain-cli`       | Manage versions of the SideChain CLI tool.           |
| [Mithril](https://github.com/input-output-hk/mithril)                          | `mithril-client`      | Manage versions of the Mithril client software.      |
| [Scrolls](https://github.com/txpipe/scrolls)                                   | `scrolls`             | Manage versions of the Scrolls client software.      |
| [Oura](https://github.com/txpipe/oura/tree/main)                               | `oura`                | Manage versions of the Oura client software.         |
| [Dolos](https://github.com/txpipe/dolos)                                       | `dolos`               | Manage versions of the Dolos client software.        |
| [Aiken](https://github.com/aiken-lang/aiken)                                   | `aiken`               | Manage versions of the Aiken client software.        |

## Installation

```bash
cargo install hyper-jump
```

## Configuration

#### On Linux and macOS:

Add the following line to your shell configuration file (e.g., `~/.bashrc`, `~/.zshrc`, etc.):

```bash
export PATH="$HOME/.local/share/hyper-jump/bin:$PATH"
```

Or add the hyper-jump binary path to your PATH by running:

```bash
sudo launchctl config user path "$(hyper-jump prefix):${PATH}"
```

## Usage

To manage packages, use the following subcommands:

### Help

Display help information:

```bash
hyper-jump --help
```

### Use

Switch to a specific version of a package.

```sh
hyper-jump use <package-name> <version>
```

### Install

Install a specific version of a package.

```sh
hyper-jump install <package-name> <version>
```

### List

List installed versions of a package:

```sh
hyper-jump list <package-name>
```

### List Remote

List remote versions available for a package.

```sh
hyper-jump list-remote <package-name>
```

### Uninstall

Uninstall a specific version of a package.

```sh
hyper-jump uninstall <package-name> <version>
```

### Erase

Remove all installed versions.

```sh
hyper-jump erase
```
