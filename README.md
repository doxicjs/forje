# Forje CLI

**"Hammering code for a polished foundation."**

`forje` is a soon-to-be powerful CLI tool designed to generate various code patterns featured in the unreleased *“2khan’s Guide to Doxic Systems”*, `forje` simplifies development by automatically creating essential code structures with an opinionated writing style for both frontend and backend projects.

> ## Forjed in Progress
>
>`forje` is fresh from the anvil — raw, unrefined, and still glowing red-hot. Each update tempers the tool, hardening its features and sharpening its edge. Expect sparks, rough edges, and plenty of hammer strikes as we "forje" into a powerful code-crafting tool. Your feedback is the heat that shapes the steel — so strike while the iron’s hot!

*Don’t worry, no blacksmiths were harmed in the making of this tool.*

## Installation

### 1. Install Rust

To install this CLI tool, follow these steps:

If you don’t have Rust installed, you can do so using [rustup](https://rustup.rs/), the recommended way to manage Rust versions:

```sh
curl --proto '=https' --tlsv1.2 -sSf https://sh.rustup.rs | sh
```

After installation, run the following to ensure your environment is set up:

```sh
source $HOME/.cargo/env
```

Note: On Windows, you can download and run the Rust installer directly.

### 2. Install Forje CLI

Once Rust is installed, you can install the CLI directly via `cargo`:

```sh
cargo install forje
```

### 3. Verify Installation

Check if the installation was successful by running:

```sh
forje --version # or forje -V
```

### 4. (Optional) Add Cargo’s Bin Directory to PATH

If the CLI isn’t recognized in your terminal, ensure Cargo’s bin directory is added to your PATH.

#### Linux / macOS:
Add this to your .bashrc, .zshrc, or .profile:

```sh
export PATH="$HOME/.cargo/bin:$PATH"
```

#### Windows (PowerShell):

```powershell
$env:Path += ";$env:USERPROFILE\.cargo\bin"
```

### 5. Update the Tool (When Needed)

To update the CLI in the future, simply run:

```sh
cargo install forje --force
```

## Still in the Furnace

`forje` is just getting started, and so is its documentation. Expect detailed guides, examples, and best practices to be added as the tool evolves. Stay tuned — the anvil’s hot, and the hammer’s still swinging!