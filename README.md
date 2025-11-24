# git-cleanup 🧹

A simple, safe, and interactive CLI tool written in Rust to clean up local git branches that have already been merged into the main branch.

## Features

- 🔍 **Auto-detection**: Finds branches merged into `main` (or any specified target).
- 🛡️ **Safe**: Ignores the current branch and the target branch itself.
- ✅ **Interactive**: Presents a checklist UI to select which branches to delete.
- 🧪 **Dry-Run Mode**: Preview what would be deleted without taking action.
- 🎨 **Colorful**: Styled output for better readability.

## Installation

### Prerequisites
- Rust and Cargo installed (via `rustup`).

### Build and Install
Clone the repository and run:

```bash
cargo install --path .
