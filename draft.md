<div align="center">

# hyper-jump

</div>

_The console lights up, keys clack rapidly and then..._ 🛸

Pff, the Cardano aircraft, where the only thing faster than the transactions is the hyper-jump between versions! It's the Swiss Army knife of version managers, the Batman utility belt for developers and the TARDIS for your codebase – it's bigger on the inside with all the versions it can handle!

Imagine you're in the cockpit of the Cardano aircraft, your fingers dance across the command line like Ludovico during a concerto. You type hyper-jump install latest, and jump! You've just skipped through versions like a time traveler at Café De Flore reading a book aside Picasso. But beware, type the wrong command and you might just end up installing "Cardano-node-vintage-0.0.1-alpha-beta-gamma" – so retro, it's practically a collector's item!

Now, let's talk about the all-in-one aspect. This isn't just a version manager; it's a version festival, a version buffet where you can sample all the flavors of Cardano without the indigestion. Cross-platform? You bet! Whether you're a Windows wizard, a macOS maestro or a Linux luminary, the Cardano aircraft has got you covered. It's like having a universal adapter for your development needs.

## Core Features

- **Version Management**: Install and switch between different versions of Cardano ecosystem tools.
- **Remote Listing**: View available versions of packages from remote repositories.
- **Package Installation**: Download and install specific versions of packages with ease.
- **Package Uninstallation**: Remove installed versions of packages to declutter the environment.
- **Version Switching**: Seamlessly switch between installed versions with a single command.
- **Proxy Handling**: Hyper-Jump acts as a proxy, allowing users to run commands from the selected package version.

## Potential Features

- **Automatic Updates**: Implement an auto-update feature that checks for and installs the latest version of packages.
- **Dependency Resolution**: Introduce dependency management to ensure that all package dependencies are met for each version.
- **Environment Isolation**: Create isolated environments for different projects, each with its own set of package versions.
- **Rollback Capability**: Add the ability to rollback to previous versions in case of compatibility issues or other concerns.
- **Version Cleanup**: Introduce a cleanup command to remove old or unused package versions, freeing up system resources.
- **Custom Package Sources**: Allow users to add custom package sources or repositories for more flexibility.
- **Enhanced List Filtering**: Provide options to filter the list of installed and remote versions based on criteria such as release date or stability.
- **Interactive CLI**: Develop an interactive command-line interface that guides users through the version management process.

## Installation

```bash
cargo install hyper-jump
```

## Usage

The Hyper-Jump tool provides several subcommands to manage packages:

```bash
hyper-jump --help
```

### Use

Switch to a specific version of a package.

```sh
hyper-jump cardano-node use --version <version>
```

### Install

Install a specific version of a package.

```sh
hyper-jump cardano-node install --version <version>
```

### Uninstall

Uninstall a specific version of a package.

```sh
hyper-jump cardano-node uninstall --version <version>
```

### List Remote

List remote versions available for a package.

```sh
hyper-jump cardano-node list-remote
```
