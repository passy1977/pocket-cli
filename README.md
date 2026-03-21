# pocket-cli

[![Ask DeepWiki](https://deepwiki.com/badge.svg)](https://deepwiki.com/passy1977/pocket-cli)
[![License: GPL v3](https://img.shields.io/badge/License-GPLv3-blue.svg)](https://www.gnu.org/licenses/gpl-3.0)
[![Rust](https://img.shields.io/badge/rust-1.0%2B-orange.svg)](https://www.rust-lang.org)
[![Version](https://img.shields.io/badge/version-0.60.0-green.svg)](https://github.com/passy1977/pocket-cli)
[![Platform](https://img.shields.io/badge/platform-Linux-lightgrey.svg)](https://www.linux.org)
[![Build](https://img.shields.io/badge/build-passing-brightgreen.svg)](https://github.com/passy1977/pocket-cli)

## Purpose and Scope

The Pocket CLI is a command-line interface system written in Rust that enables management of users and devices through a [pocket-backend](https://github.com/passy1977/pocket-backend) remote server. This document provides a comprehensive overview of the system's architecture, components, and functionality.

## Architecture Overview

The Pocket CLI is structured as a Rust workspace with three main components:

### Core Library (`pocket`)
The foundational library providing shared functionality for both CLI tools:
- **Database Management**: SQLite-based persistent storage for configuration and authentication data
- **Security**: AES-256-CBC encryption for secure communication with the backend server
- **Socket Communication**: TCP-based client for server interaction via `127.0.0.1:8300`
- **Models & Traits**: Shared data structures and command protocol definitions
- **Utilities**: Password handling, random string generation, and common helpers

### User Management Tool (`pocket-user`)
Command-line binary for managing user accounts:
- Add, modify, remove, and retrieve user information
- Requires email, password, and name for user operations
- Commands: `add`, `mod`, `rm`, `get`

### Device Management Tool (`pocket-device`)  
Command-line binary for managing devices:
- Add, modify, remove, and retrieve device information
- Requires email and UUID for device operations
- Supports optional note fields for device descriptions
- Commands: `add`, `mod`, `rm`, `get`

## How It Works

### 1. Initialization & Configuration
On first run, the system:
- Creates a `.pocket` directory in the user's home folder
- Initializes a SQLite database (`pocket-cli.db`) with a `properties` table
- Generates a random 16-byte initialization vector (IV) for encryption

### 2. Server Authentication
The CLI tools authenticate with the backend server:
- Server password is provided via `-P` flag or retrieved from stored properties
- An encrypted session is established using AES-256-CBC encryption
- Authentication data is persisted locally for subsequent commands

### 3. Command Execution Flow
1. **Parse Arguments**: Command-line arguments are parsed into commands and options
2. **Validate Input**: Required parameters are validated (email, password, UUID, etc.)
3. **Server Login**: Establishes encrypted connection to backend via TCP socket
4. **Build Command**: Converts command objects to protocol strings using pipe (`|`) delimiters
5. **Send & Receive**: Transmits encrypted command to server and awaits response
6. **Display Result**: Outputs server response or error messages to user

### 4. Communication Protocol
Commands are sent to the server as pipe-delimited strings:
- Format: `COMMAND|parameter1|parameter2|...`
- Examples:
  - `ADD_USER|email@example.com|password|name`
  - `GET_DEVICE|email@example.com|uuid|note`
  - `MOD_USER|email@example.com|newpassword|newname`

### 5. Security Model
- **Encryption**: All server communications use AES-256-CBC with OpenSSL
- **Key Storage**: Server password and IV stored in local SQLite database
- **Session Management**: Persistent login state maintained across invocations
- **Socket Security**: Local TCP connection to backend proxy at `127.0.0.1:8300`

## Install
For installation you need OpenSSL installed on your Linux system.
```bash
sudo pacman -S openssl          # Arch Linux
sudo apt-get install libssl-dev # Debian/Ubuntu
sudo dnf install openssl-devel  # Fedora
```
Make sure that [Rust is installed](https://www.rust-lang.org/tools/install) on your system.

```bash
git clone https://github.com/passy1977/pocket-cli.git
cd pocket-cli/
cargo build --release
```

The compiled binaries will be available in `target/release/`:
- `pocket-user` - User management CLI
- `pocket-device` - Device management CLI

## Usage

### User Management (`pocket-user`)

```bash
# Add a new user
pocket-user add -P <server_password> -e <email> -p <password> -n <name>

# Modify an existing user
pocket-user mod -P <server_password> -e <email> -p <new_password> -n <new_name>

# Get user information
pocket-user get -P <server_password> -e <email>

# Remove a user
pocket-user rm -P <server_password> -e <email>
```

### Device Management (`pocket-device`)

```bash
# Add a new device
pocket-device add -P <server_password> -e <email>

# Get device information
pocket-device get -P <server_password> -e <email> -u <uuid>

# Modify device (update note)
pocket-device mod -P <server_password> -e <email> -u <uuid> --note <note>

# Remove a device
pocket-device rm -P <server_password> -e <email> -u <uuid>
```

### Command-Line Options

**Common Options (both tools):**
- `-P, --server-passwd <passwd>` : Server password (required on first run, stored for subsequent commands)
- `-e, --email <email>` : User/device email address (required)
- `-h, --help <command>` : Display help menu

**User Management Options (pocket-user):**
- `-p, --passwd <passwd>` : User password (required for add/mod operations)
- `-n, --name <name>` : User name (required for add/mod operations)

**Device Management Options (pocket-device):**
- `-u, --uuid <uuid>` : Device UUID (required for get/mod/rm operations)
- `--note <note>` : Device note/description (optional for mod operations)

## Examples

### Typical Workflow for User Management

```bash
# First time: Add a new user with server password
./pocket-user add -P mySecretServerPass123 -e john.doe@example.com -p userPass456 -n "John Doe"

# Output: User created successfully with ID: 42

# Get user details (server password already stored, no need to specify -P)
./pocket-user get -e john.doe@example.com

# Modify user information
./pocket-user mod -e john.doe@example.com -p newPassword789 -n "John M. Doe"

# Output: User updated successfully

# Remove user
./pocket-user rm -e john.doe@example.com

# Output: User removed successfully
```

### Typical Workflow for Device Management

```bash
# Add a new device for a user
./pocket-device add -P mySecretServerPass123 -e alice@example.com

# Output: Device created with UUID: a1b2c3d4-5e6f-7g8h-9i0j-k1l2m3n4o5p6

# Get device information
./pocket-device get -e alice@example.com -u a1b2c3d4-5e6f-7g8h-9i0j-k1l2m3n4o5p6

# Add a note to the device
./pocket-device mod -e alice@example.com -u a1b2c3d4-5e6f-7g8h-9i0j-k1l2m3n4o5p6 --note "Alice's laptop - Ubuntu 22.04"

# Output: Device updated successfully

# Get updated device information with note
./pocket-device get -e alice@example.com -u a1b2c3d4-5e6f-7g8h-9i0j-k1l2m3n4o5p6

# Remove device
./pocket-device rm -e alice@example.com -u a1b2c3d4-5e6f-7g8h-9i0j-k1l2m3n4o5p6

# Output: Device removed successfully
```

### Multi-Device Management Example

```bash
# Add multiple devices for the same user
./pocket-device add -e bob@company.com
# Output: Device created with UUID: 11111111-2222-3333-4444-555555555555

./pocket-device add -e bob@company.com  
# Output: Device created with UUID: 66666666-7777-8888-9999-000000000000

# Add descriptive notes to each device
./pocket-device mod -e bob@company.com -u 11111111-2222-3333-4444-555555555555 --note "Work Desktop - Windows 11"
./pocket-device mod -e bob@company.com -u 66666666-7777-8888-9999-000000000000 --note "Personal Laptop - MacOS"

# Query each device
./pocket-device get -e bob@company.com -u 11111111-2222-3333-4444-555555555555
./pocket-device get -e bob@company.com -u 66666666-7777-8888-9999-000000000000
```

### Help and Documentation

```bash
# Display help for pocket-user
./pocket-user -h

# Display help for pocket-device
./pocket-device -h
```

## Project Structure

```
pocket-cli/
├── pocket/              # Core library
│   ├── src/
│   │   ├── database/    # SQLite database layer
│   │   ├── models/      # Data structures and commands
│   │   ├── services/    # AES encryption, socket, args parsing
│   │   ├── traits/      # Command protocol traits
│   │   └── utils.rs     # Utility functions
│   └── Cargo.toml
├── pocket-user/         # User management CLI
│   ├── src/
│   │   ├── cli.rs       # Argument parsing and menu
│   │   ├── user.rs      # User command implementation
│   │   └── main.rs      # Entry point
│   └── Cargo.toml
├── pocket-device/       # Device management CLI
│   ├── src/
│   │   ├── cli.rs       # Argument parsing and menu
│   │   ├── device.rs    # Device command implementation
│   │   └── main.rs      # Entry point
│   └── Cargo.toml
└── Cargo.toml           # Workspace configuration
```

## Development

### Building for Development
```bash
# Build all binaries (debug mode)
cargo build

# Build with test arguments enabled
cargo build --features test_args

# Build individual binaries
cargo build --bin pocket-user
cargo build --bin pocket-device
```

### Running Tests
```bash
cargo test
```

### Available Tasks
The project includes VS Code tasks for common operations:
- `build-all-debug` - Build all components in debug mode
- `build-all-release` - Build all components in release mode
- `run-debug-pocket-user` - Run user CLI with test args
- `run-debug-pocket-device` - Run device CLI with test args
- `test-all` - Run all tests
- `clean` - Clean build artifacts

## Dependencies

### Core Library (`pocket`)
- **rusqlite**: SQLite database bindings with bundled SQLite
- **openssl-sys**: OpenSSL bindings for AES encryption
- **chrono**: Date and time handling
- **mac_address**: MAC address retrieval for device identification
- **libc**: Low-level C bindings

### CLI Tools
Both `pocket-user` and `pocket-device` depend solely on the `pocket` core library.

## License
GPL-3.0 License - See LICENSE file for details

## Author
Antonio Salsi

---

## Pocket Ecosystem

### Core Components

- **[Pocket Backend](https://github.com/passy1977/pocket-backend)** - Java/Spring Boot backend service providing REST APIs, authentication, and business logic
- **[Pocket Web Backend](https://github.com/passy1977/pocket-web-backend)** - Rust/Actix web server with rate limiting and session management
- **[Pocket Lib](https://github.com/passy1977/pocket-lib)** - C++ core library for performance-critical operations and cryptography

### Client Applications

- **[Pocket Web Frontend](https://github.com/passy1977/pocket-web-frontend)** - Modern web interface for browser-based access
- **[Pocket CLI](https://github.com/passy1977/pocket-cli)** - Command-line tools for user and device management
- **[Pocket iOS](https://github.com/passy1977/pocket-ios)** - Native iOS client (this repository)
