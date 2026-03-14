# Contributing to vox-hash

Thank you for your interest in contributing to **vox-hash**! We welcome contributions from the community, including bug reports, feature requests, code improvements, and documentation enhancements. This guide outlines how to get involved and help improve this Rust-based CLI tool for hashing and brute-force hash matching.

## Table of Contents

- [Code of Conduct](#code-of-conduct)
- [How to Contribute](#how-to-contribute)
  - [Reporting Bugs](#reporting-bugs)
  - [Suggesting Features](#suggesting-features)
  - [Submitting Pull Requests](#submitting-pull-requests)
- [Development Setup](#development-setup)
- [Code Style Guidelines](#code-style-guidelines)
- [Testing](#testing)
- [Community](#community)

## Code of Conduct

All contributors are expected to adhere to the [Code of Conduct](CODE_OF_CONDUCT.md). This ensures a respectful and inclusive environment for everyone involved in the project.

## How to Contribute

### Reporting Bugs

If you encounter a bug in vox-hash:

1. **Check Existing Issues**: Search the [Issues page](https://github.com/VoxDroid/vox-hash/issues) to see if the bug has already been reported.
2. **Create a New Issue**: If the bug is new, open a new issue and provide:
   - A clear title and description of the bug.
   - Steps to reproduce the issue (e.g., specific command, input file).
   - Expected and actual behavior.
   - Screenshots, error messages, or verbose logs (`--noverbose false`).
   - Your environment (e.g., OS, Rust version, input file size).
3. **Use the Bug Report Template**: Follow the template provided in the issue creation form for consistency.

**Example Bug Report**:
- Title: "Brute-Force Hangs with Large Charset and Max Length"
- Description: `dec` command freezes when using `--max-len 10` with alphanumeric charset.
- Steps: Run `vox-hash dec --key 5baa61e4c9b93f3f0682250b6cf8331b7ee68fd8 --max-len 10`.
- Expected: Progress bar updates or match found.
- Actual: No output after 5 minutes.
- Environment: Rust 1.70, Ubuntu 22.04, 8GB RAM.

### Suggesting Features

We welcome ideas to enhance vox-hash! To suggest a feature:

1. **Check Existing Requests**: Review the [Issues page](https://github.com/VoxDroid/vox-hash/issues) to avoid duplicates.
2. **Submit a Feature Request**: Open a new issue and include:
   - A clear title and detailed description of the feature.
   - The problem it solves or the benefit it provides (e.g., adds SHA256, improves wordlist efficiency).
   - Any relevant examples, code snippets, or references to similar tools.
3. **Use the Feature Request Template**: Follow the provided template to structure your suggestion.

**Example Feature Request**:
- Title: "Add Support for SHA256 Hashing"
- Description: Include SHA256 as an algorithm option for `enc` and `dec` commands.
- Benefit: Expands tool applicability to modern security contexts.

### Submitting Pull Requests

To contribute code or documentation:

1. **Fork the Repository**:
   - Fork the [vox-hash repository](https://github.com/VoxDroid/vox-hash).
   - Clone your fork to your local machine:
     ```bash
     git clone https://github.com/YOUR_USERNAME/vox-hash.git
     ```

2. **Create a Branch**:
   - Create a new branch for your changes:
     ```bash
     git checkout -b feature/your-feature-name
     ```
   - Use descriptive branch names (e.g., `fix/brute-force-hang`, `feature/sha256-support`).

3. **Make Changes**:
   - Modify the Rust code or documentation as needed.
   - Follow the [Code Style Guidelines](#code-style-guidelines) below.
   - Test your changes locally across platforms if possible.

4. **Commit Changes**:
   - Write clear, concise commit messages:
     ```bash
     git commit -m "Add SHA256 hashing support to enc and dec commands"
     ```
   - Reference related issues (e.g., `Fixes #123`).

5. **Push and Create a Pull Request**:
   - Push your branch to your fork:
     ```bash
     git push origin feature/your-feature-name
     ```
   - Open a pull request (PR) against the `main` branch of the original repository.
   - Use the PR template and provide:
     - A description of the changes.
     - The issue number(s) addressed (if any).
     - Screenshots or sample outputs for CLI changes.
     - Testing performed (e.g., platforms tested, input files used).

6. **Code Review**:
   - Respond to feedback from maintainers.
   - Make requested changes and update your PR as needed.
   - Your PR will be merged once approved.

## Development Setup

To set up a development environment for vox-hash:

1. **Prerequisites**:
   - Rust (stable, 1.56+): Install via [rustup](https://rustup.rs/).
   - Git for version control.
   - A code editor (e.g., VS Code, IntelliJ with Rust plugin).

2. **Clone the Repository**:
   ```bash
   git clone https://github.com/VoxDroid/vox-hash.git
   cd vox-hash
   ```

3. **Install Dependencies**:
   - Rust dependencies are managed by Cargo and included in `Cargo.toml`:
     - `clap`, `sha1`, `md5`, `rayon`, `indicatif`, `chrono`, `serde_json`, `regex`
   - Run `cargo build` to fetch and compile dependencies:
     ```bash
     cargo build
     ```

4. **Prepare Test Files**:
   - Create sample input files (e.g., `words.txt`, `hashes.txt`) as described in [Installation](#installation).
   - Generate a small rainbow table for testing:
     ```bash
     cargo run -- generate-table --output test_table.json --min-len 1 --max-len 2 --algo sha1
     ```

5. **Build and Run**:
   - Build the project:
     ```bash
     cargo build --release
     ```
   - Run with a sample command:
     ```bash
     cargo run -- enc --algo sha1 --str test
     ```

6. **Test Changes**:
   - Verify output for all commands (`enc`, `dec`, `bulk-enc`, `bulk-dec`, `generate-table`, `benchmark`).
   - Check verbose logs and progress bars.
   - Test on multiple platforms (e.g., Linux, Windows) if possible.

## Code Style Guidelines

To maintain consistency in the codebase:

- **Rust**:
  - Follow the [Rust Style Guidelines](https://github.com/rust-dev-tools/fmt-rfcs/blob/master/guide/guide.md).
  - Use 4-space indentation.
  - Run `cargo fmt` to format code.
  - Run `cargo clippy` to catch common issues.
  - Use descriptive variable/function names (e.g., `hash_string`, `brute_force_hash`).
  - Add comments for complex logic or public functions.
  - Example:
    ```rust
    /// Hashes a string using the specified algorithm (SHA1 or MD5).
    fn hash_string(input: &str, algo: Algorithm) -> String {
        // ...
    }
    ```
- **CLI**:
  - Use clear, user-friendly error messages.
  - Maintain consistency with existing commands and flags (e.g., `--algo`, `--json`).
  - Update `--help` messages in `clap` attributes for new features.
- **File Structure**:
  - Keep source code in `src/` (e.g., `main.rs`).
  - Store sample input files in `examples/` (not tracked in Git).
  - Place generated outputs (e.g., rainbow tables) in `outputs/` (not tracked).
- **Error Handling**:
  - Use `Result` and `Option` for fallible operations.
  - Provide descriptive error messages with context (e.g., file paths, invalid inputs).
- **Performance**:
  - Optimize for large datasets (e.g., batch processing, efficient memory usage).
  - Use `rayon` for parallel tasks where appropriate.

## Testing

Before submitting a pull request:

- **Manual Testing**:
  - Test all commands with sample inputs:
    - `enc`: Hash various strings with SHA1 and MD5.
    - `dec`: Crack known hashes using wordlists, patterns, and brute-force.
    - `bulk-enc`: Process a file with multiple strings.
    - `bulk-dec`: Crack multiple hashes with different configurations.
    - `generate-table`: Create and verify a small rainbow table.
    - `benchmark`: Confirm reasonable performance metrics.
  - Verify JSON output and file writing.
  - Check progress bars and verbose logs.
- **Platform Testing**:
  - Test on at least one platform (e.g., Linux, Windows, macOS).
  - Verify compatibility with Rust 1.56+.
- **Edge Cases**:
  - Test with invalid inputs (e.g., non-existent files, malformed hashes).
  - Test with large files (e.g., 10,000 hashes) and charsets.
  - Test with extreme values (e.g., `--max-len 20`, `--conc 100`).
- **Automation**:
  - Run `cargo test` to execute unit tests (add tests for new functions).
  - Run `cargo fmt --check` to verify formatting.
  - Run `cargo clippy -- -D warnings` to ensure no lints.
- **Output Verification**:
  - Compare outputs against expected results (e.g., known SHA1/MD5 hashes).
  - Verify rainbow table lookups and JSON parsing.

## Community

Join the vox-hash community:

- **GitHub Discussions**: Share ideas or ask questions in the [Discussions](https://github.com/VoxDroid/vox-hash/discussions) section.
- **Issues**: Report bugs or suggest features on the [Issues page](https://github.com/VoxDroid/vox-hash/issues).
- **Email**: Contact the maintainer at [izeno.contact@gmail.com](mailto:izeno.contact@gmail.com).
- **GitHub Stars**: Show your support by starring the [repository](https://github.com/VoxDroid/vox-hash).

Thank you for contributing to vox-hash! Your efforts help make this tool better for security researchers and developers worldwide.
