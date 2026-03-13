# vox-hash Project Overview

Last updated: 2026-03-13

## 1. Executive Summary

`vox-hash` is a Rust command-line tool focused on:
- Hash generation (`SHA1`, `MD5`)
- Hash matching/decryption by multiple strategies
- Bulk processing of input files
- Rainbow table generation and lookup
- Simple benchmark runs

Current implementation is a single-binary, single-file architecture (`src/main.rs`) with all logic in one module.

## 2. Product Scope

### 2.1 Primary Use Cases
- Generate hashes for strings.
- Recover plaintext candidates for known `SHA1`/`MD5` hashes.
- Process many inputs in batch mode.
- Precompute hash lookup data (rainbow table JSON files).
- Evaluate local hashing throughput.

### 2.2 User Personas
- Security researchers and penetration testers (authorized environments).
- Developers needing quick CLI hashing workflows.
- Learners exploring brute-force/hash matching behavior.

### 2.3 Non-Goals (Current Version)
- No network/API service mode.
- No GPU acceleration.
- No plugin system.
- No authenticated multi-user workflow.

## 3. Current Command Surface

Main commands:
- `enc`: hash one string.
- `dec`: find plaintext for one hash.
- `bulk-enc`: hash many lines from file.
- `bulk-dec`: decrypt many hashes from file.
- `generate-table`: create rainbow table JSON.
- `benchmark`: hashes/second estimate.

Global flags include:
- `--noverbose`
- `--max-len`
- `--charset-type`
- `--charset`

## 4. Current Functional Architecture

### 4.1 High-Level Flow
1. Parse CLI args with `clap`.
2. Resolve charset and runtime options.
3. Branch by subcommand.
4. Execute operation (hashing, brute-force, bulk, table generation, benchmark).
5. Render to stdout and/or file (plain or JSON).

### 4.2 Decryption Strategy Order
For `dec` and `bulk-dec`, matching is attempted in this sequence:
1. Rainbow table lookup (if provided)
2. Common pattern list (if enabled)
3. Wordlist scan (if provided)
4. Brute-force search with charset/prefix/suffix/pattern constraints

### 4.3 Concurrency Model
- Uses `rayon` thread pools (`ThreadPoolBuilder`) for parallel operations.
- Brute-force uses parallel iterators in batches to reduce memory pressure.
- Bulk decrypt processes hashes in chunks (`--batch-size`).

### 4.4 Data and File I/O
- Reads line-based input files (`BufReader` + `lines`).
- Writes plain text or JSON output files.
- Rainbow tables are stored as JSON object mappings (`hash -> plaintext`).

## 5. Current Code Structure

Repository structure (relevant):
- `Cargo.toml`
- `src/main.rs` (all CLI, domain logic, and IO)
- `README.md`
- `.github/` templates/funding metadata
- Policy docs (`CONTRIBUTING.md`, `SECURITY.md`, `SUPPORT.md`, etc.)

Current technical state:
- Monolithic source file.
- Minimal separation between parsing, business logic, and infrastructure.
- Primarily manual testing guidance; no visible dedicated test suite in repo root.

## 6. Technology Stack

### 6.1 Language and Build
- Rust edition `2024`
- Cargo build system

### 6.2 Core Dependencies
From `Cargo.toml`:
- `clap` (CLI parsing)
- `sha1` and `md5` (hashing)
- `rayon` (parallelism)
- `indicatif` (progress bars)
- `chrono` (timestamps)
- `serde_json` (JSON output/rainbow tables)
- `regex` (pattern parsing/validation)

## 7. Behavioral Rules and Validation (Current)

- Hash validation:
  - Hex string enforcement
  - Length-based validation (`MD5=32`, `SHA1=40`)
- Length constraints:
  - `min-len <= max-len`
  - Prefix/suffix length compatibility checks
- File checks:
  - Input existence validation for wordlists/hash files/tables
- Output modes:
  - Verbose and non-verbose behavior differ
  - Optional JSON formatting

## 8. Known Architectural Risks (Current)

- Single-file architecture increases maintenance cost.
- Shared concerns (CLI/domain/IO) are tightly coupled.
- Harder unit-test isolation due to limited modular boundaries.
- Potential memory and CPU pressure for large brute-force ranges.
- Rainbow tables as large JSON objects may not scale for very large datasets.

## 9. Proposed Revamp Target Architecture

### 9.1 Target Principles
- Clean module boundaries.
- Test-first and deterministic core logic.
- Clear separation of domain logic from CLI and filesystem.
- Extensible algorithm/provider model.
- Performance-aware defaults with explicit limits.

### 9.2 Suggested Module Layout
- `src/main.rs`: process entrypoint, lightweight bootstrapping.
- `src/cli/`: argument schema, command translation.
- `src/domain/`:
  - `hashing.rs`
  - `validation.rs`
  - `candidate_generation.rs`
  - `decryption_orchestrator.rs`
- `src/infra/`:
  - `file_io.rs`
  - `json_output.rs`
  - `progress.rs`
  - `threading.rs`
- `src/app/`: use-case services (`enc`, `dec`, bulk flows, benchmark).
- `src/errors.rs`: typed errors and conversion.
- `src/config.rs`: shared runtime config.

### 9.3 Suggested Layered Flow
1. CLI layer maps args to typed request models.
2. App layer orchestrates use cases.
3. Domain layer performs pure computations and matching.
4. Infra layer handles filesystem, terminal rendering, serialization.

## 10. Security and Compliance Context

- Repository includes `SECURITY.md` with private disclosure guidance.
- Project positions usage as ethical/legal only.
- Revamp should preserve explicit user warnings for misuse risk.
- Input files should be treated as untrusted and validated defensively.

## 11. DevEx and Quality Context

- Contribution guidelines recommend:
  - `cargo fmt`
  - `cargo clippy`
  - `cargo test`
- Revamp should add enforceable CI workflows for these checks.

## 12. Suggested Future Enhancements

- Additional algorithms (`SHA256`, `SHA512`, etc.) behind trait-based interfaces.
- Resume/checkpoint support for long brute-force sessions.
- Streaming or sharded rainbow-table formats.
- Better structured output contracts (JSON schema).
- Optional library crate split (`vox_hash_core`) + CLI wrapper.

## 13. Definition of Done for Revamp

A revamp is complete when:
- Architecture is modular and testable.
- Core flows maintain behavior parity (or documented intentional changes).
- Unit/integration/CLI snapshot tests cover critical paths.
- CI gates formatting, linting, tests, and release build.
- Documentation reflects actual architecture and usage.
