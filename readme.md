<div align="center">

# hyper-jump

_The console lights up, keys clack rapidly and then..._ 🛸

</div>

Pff, the Cardano aircraft, where the only thing faster than the transactions is the hyper-jump between versions! It's the Swiss Army knife of version managers, the Batman utility belt for developers and the TARDIS for your codebase – it's bigger on the inside with all the versions it can handle!

Imagine you're in the cockpit of the Cardano aircraft, your fingers dance across the command line like Ludovico does in the piano during a concerto. You type hyper-jump install <package> latest, and jump! You've just skipped through versions like a time traveler at Café De Flore reading a book aside Picasso. But beware, type the wrong command and you might just end up installing "Cardano-node-vintage-0.0.1-alpha-beta-gamma" – and get stuck in the past!

Now, let's talk about the all-in-one aspect. This isn't just a version manager; it's a version festival, where you can sample all the flavors of Cardano without the indigestion. Cross-platform? Whether you're a Windows wizard, a macOS maestro or a Linux luminary, the hyper-jump has got you covered. It's like having a universal adapter for your development needs.

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

| Package Name                                                  | Alias            | Description                                     |
| ------------------------------------------------------------- | ---------------- | ----------------------------------------------- |
| [Cardano Node](https://github.com/IntersectMBO/cardano-node)  | `cardano-node`   | Manage versions of the Cardano Node software.   |
| [Cardano CLI](https://github.com/cardano-scaling/cardano-cli) | `cardano-cli`    | Manage versions of the Cardano CLI tool.        |
| [Mithril](https://github.com/input-output-hk/mithril)         | `mithril-client` | Manage versions of the Mithril client software. |
| [Oura](https://github.com/txpipe/oura/tree/main)              | `oura`           | Manage versions of the Oura client software.    |
| [Aiken](https://github.com/aiken-lang/aiken)                  | `aiken`          | Manage versions of the Aiken client software.   |

## Installation

```bash
cargo install hyper-jump
```

## Configuration

#### On Linux and macOS:

Add the following line to your shell configuration file (e.g., `~/.bashrc`, `~/.zshrc`, etc.):

```bash
export PATH="$HOME/.local/share/hyper-jump/cardano-bin:$PATH"
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
hyper-jump use cardano-node <version>
```

### Install

Install a specific version of a package.

```sh
hyper-jump install cardano-node <version>
```

### Uninstall

Uninstall a specific version of a package.

```sh
hyper-jump uninstall cardano-node <version>
```

### List

List installed versions of a package:

```sh
hyper-jump list cardano-node
```

### List Remote

List remote versions available for a package.

```sh
hyper-jump list-remote cardano-node
```

### Erase

Remove all installed versions.

```sh
hyper-jump erase
```
