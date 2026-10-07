# Demo Selenium Rust

Demonstration of:

* [Selenium](https://www.selenium.dev/) browser automation testing
* [Rust](https://www.rust-lang.org/) programming language
* [Cargo](https://doc.rust-lang.org/cargo/) build tool and package manager
* [ChromeDriver](https://developer.chrome.com/docs/chromedriver) extends WebDriver by adding Chromium-specific capabilities

There is no official Rust binding from the Selenium project itself.
[`thirtyfour`](https://crates.io/crates/thirtyfour) — whose name nods to
selenium's atomic number, 34 — is the de facto Selenium/WebDriver client
for Rust, and is what this demo uses.

The exact scenario this demo walks through (target URL, locators, form
interactions) is specified in [spec/index.md](spec/index.md); the code and
spec must agree.

## Install

### Install Rust and Cargo

Install Rust (which includes Cargo) from <https://www.rust-lang.org/tools/install>,
typically via `rustup`.

Run this to confirm your version:

```sh
rustc --version
```

Output should be at least:

```stdout
rustc 1.88.0
```

Run this to confirm your version:

```sh
cargo --version
```

### Install dependencies

```sh
cargo build
```

This downloads and compiles `thirtyfour` and the other dependencies listed
in [Cargo.toml](Cargo.toml).

### Install chromedriver

Unlike Playwright, Selenium/`thirtyfour` does not bundle or manage browser
binaries for you: you need a WebDriver server already running.

If you use macOS brew:

```sh
brew install chromedriver
```

Or download the version that matches your Chrome browser app from
<https://chromedriver.chromium.org/downloads> or, for newer Chrome
versions, <https://googlechromelabs.github.io/chrome-for-testing/>.

### Update

Run:

```sh
cargo update
```

## Run

Start `chromedriver` listening on port 9515 (the port this demo connects
to):

```sh
chromedriver --port=9515
```

Then, in another terminal, run:

```sh
cargo run
```

The script will do three things:

1. Launch a local Chrome web browser (via `chromedriver`) to view the free
   open source testing examples web page
   <https://testingexamples.github.io/en-001/practice/>.

2. Interact with the web page in various ways, such as finding elements,
   clicking on elements, filling in form inputs, etc.

3. Print some typical output that demonstrates the program is running
   successfully.

### Troubleshooting "This version of ChromeDriver …"

If you get an error message about a Chrome/chromedriver version mismatch,
harmonize your Chrome browser app and your `chromedriver` version (e.g.
`brew upgrade chromedriver` on macOS, or download a matching version from
the links above).

## Tracking

* Package: demo-selenium-rust
* Version: 1.0.0
* Created: 2026-09-03T00:00:00Z
* Updated: 2026-09-03T00:00:00Z
* License: GPL-2.0-or-greater or for custom license contact us
* Contact: Joel Parker Henderson (joel@joelparkerhenderson.com)
