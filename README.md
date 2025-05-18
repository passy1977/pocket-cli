# pocket-cli
[![Ask DeepWiki](https://deepwiki.com/badge.svg)](https://deepwiki.com/passy1977/pocket-cli)
## Purpose and Scope

The Pocket CLI is a command-line interface system written in Rust that enables management of users and devices through a [remote server](https://github.com/passy1977/pocket-backend). This document provides a high-level overview of the system's architecture, components, and functionality. For more detailed information about specific components, please refer to their respective documentation pages (e.g., for the core library details, see Core Library (pocket)).

## Install
For install you need openssl installed on your linux system.
```bash
sudo pacman -S openssl
sudo apt-get install libssl-dev
sudo dnf install openssl-devel
```
Make sure that in your system it's [installed a version of rust](https://www.rust-lang.org/tools/install)

```bash
git clone https://github.com/passy1977/pocket-cli.git
cd pocket-cli/
cargo build --release
```