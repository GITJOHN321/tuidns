# Tuidns

A terminal-based DNS and domain diagnostics toolkit built with **Rust**, **Ratatui**, and **Crossterm**.

`Tuidns` provides a centralized interface for gathering domain, DNS, SSL, email authentication, and WHOIS information without switching between multiple command-line tools.

---

## Features

### DNS Analysis

* DNS resolution
* IP address lookup
* Connectivity checks
* SSL/TLS certificate inspection
* Host information overview

### Email Diagnostics

* MX record lookup
* SPF validation
* DKIM validation
* DMARC validation

### WHOIS Information

* Registrar information
* Domain expiration date
* Domain status information
* Automatic WHOIS server discovery

### Hosting Detection

* Hosting control panel detection
* cPanel detection
* Plesk detection
* DirectAdmin detection
* Other supported hosting panels

### Productivity

* Interactive terminal interface (TUI)
* Query history navigation
* Clipboard integration
* Parallel execution for improved performance
* Lightweight and fast

---

## Screenshots

<p align="center">
  <img src="docs/screenshots/facebook_tui.png" width="45%">
  <img src="docs/screenshots/google_tui.png" width="45%">
</p>

---

## Installation

### Build from Source

Clone the repository:

```bash
git clone https://github.com/GITJOHN321/tuidns.git
cd Tuidns
```

Build the project:

```bash
cargo build --release
```

The binary will be available at:

```bash
target/release/Tuidns
```

---

## Runtime Dependencies

The following external tools must be available in your system PATH:

| Dependency | Purpose                        |
| ---------- | ------------------------------ |
| whois      | WHOIS queries                  |
| openssl    | SSL/TLS certificate inspection |
| dig        | DNS record lookups             |
| nslookup   | DNS resolution fallback        |
| ping       | Connectivity testing           |

### macOS

Using Homebrew:

```bash
brew install whois
brew install openssl
brew install bind
```

Notes:

* `dig` and `nslookup` are included in the `bind` package.
* `ping` is included by default in macOS.

### Debian / Ubuntu

```bash
sudo apt update

sudo apt install \
    whois \
    openssl \
    dnsutils \
    iputils-ping
```

### RHEL / Rocky Linux / AlmaLinux

```bash
sudo dnf install \
    whois \
    openssl \
    bind-utils \
    iputils
```

---

## Usage

Launch the application:

```bash
Tuidns
```

Enter a domain name:

```text
example.com
```

The application will automatically perform all available diagnostics and display the results in separate panels.

---

## Keyboard Shortcuts

| Key        | Action                      |
| ---------- | --------------------------- |
| Enter      | Run domain analysis         |
| Up Arrow   | Previous query from history |
| Down Arrow | Next query from history     |
| Ctrl+C     | Copy result summary         |
| Esc        | Exit application            |

---

## Architecture

The project follows a modular layered architecture:

```text
src
├── ui
├── models
├── controllers
├── services
└── utils
```

### UI

Responsible for rendering widgets and layouts using Ratatui.

### Controllers

Coordinate application workflows and aggregate results from multiple services.

### Services

Contain the business logic for:

* DNS
* MX
* NS
* SPF
* DKIM
* DMARC
* WHOIS
* Panel detection

### Utils

Provide infrastructure and network-related helpers such as:

* SSL inspection
* Ping checks
* Port detection
* WHOIS server discovery
* IP resolution

---

## Why Tuidns?

System administrators and hosting support teams often need to run multiple commands to gather domain information:

```bash
dig example.com
whois example.com
openssl s_client ...
ping example.com
```

`Tuidns` consolidates all of this information into a single interactive terminal interface, making domain diagnostics faster and more convenient.

---

## License

MIT License
