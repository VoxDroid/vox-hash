# Revamp Implementation Checklist (From Scratch)

Last updated: 2026-03-13

## 0. Project Setup and Governance

- [x] Confirm revamp scope and success metrics.
- [x] Freeze current behavior expectations from `README.md` and current CLI help.
- [x] Define semantic versioning (v1.3.0).
- [x] Add/update `CONTRIBUTING.md` revamp section.
- [x] Define branching (main + feature branches).

## 1. Architecture and Repository Restructure

- [x] Create modular folder layout (`cli`, `app`, `domain`, `infra`).
- [x] Move business logic out of `src/main.rs`.
- [x] Add `config` and typed request/response models.
- [x] Add typed error model (`thiserror` optional) and unified result handling.
- [x] Introduce trait boundaries for hash algorithms and match providers.
- [x] Add architecture decision record (ADR) notes.

## 2. CLI Layer Rebuild

- [x] Re-declare all existing commands and flags in modular CLI structs.
- [x] Preserve user-facing command names and compatibility aliases if needed.
- [x] Centralize argument validation and conflict rules.
- [x] Improve help text examples per command.
- [x] Add robust parsing for pattern syntax and lengths.
- [x] Define exit code map for success, validation errors, runtime errors.

## 3. Hashing Core

- [x] Implement hash algorithm abstraction (`enum` or `trait`).
- [x] Keep `SHA1` and `MD5` parity.
- [x] Normalize and validate hash input casing/format.
- [x] Add deterministic result formatting.
- [x] Design extension point for future algorithms.

## 4. Candidate Generation Engine

- [x] Rebuild charset providers (`alphanumeric`, `lowercase`, `uppercase`, `digits`, custom).
- [x] Implement prefix/suffix-aware candidate builder.
- [x] Implement min/max/fixed length controller.
- [x] Implement pattern parser and validator with explicit error messages.
- [x] Add limits/guards to prevent accidental explosive workloads.

## 5. Matching Pipeline

- [x] Build orchestrator with explicit strategy order:
- [x] `rainbow_table -> common_patterns -> wordlist -> brute_force`.
- [ ] Make strategy ordering configurable (optional enhancement).
- [x] Add per-strategy timing and counters.
- [x] Support early-exit signal across workers.
- [x] Ensure deterministic behavior in no-match outcomes.

## 6. Bulk Processing Rebuild

- [x] Implement streaming line reader for large input files.
- [x] Implement chunked/batch processing with configurable batch size.
- [x] Add memory-safe defaults for large datasets.
- [x] Preserve `--only-success` behavior in plain output.
- [x] Ensure JSON output includes all relevant fields per record.

## 7. Rainbow Table Subsystem

- [x] Define table file schema and version tag.
- [x] Implement table generation with bounded resource checks.
- [x] Implement table loading with parse and schema validation.
- [ ] Consider alternative table storage (optional):
- [ ] newline-delimited JSON, binary, sqlite, or partitioned files.
- [x] Add corruption/invalid-format error handling.

## 8. Output and UX

- [x] Standardize structured output models (DecryptionResult).
- [x] Keep plain output concise for piping.
- [x] Make verbose logs consistent and timestamped.
- [x] Improve progress rendering for long-running operations.
- [x] Ensure no progress bar noise in non-interactive/noverbose mode.

## 9. Performance and Concurrency

- [x] Centralize thread pool config and bounds.
- [x] Add benchmark harness for core operations (benchmark command).
- [x] Evaluate batching strategy and tune defaults.
- [x] Prevent nested oversubscription in rayon contexts (fixed with ThreadPool).
- [x] Profile CPU/memory for representative workloads.

## 10. Error Handling and Reliability

- [x] Replace panics/`expect` with recoverable errors where appropriate.
- [x] Add contextual error messages for file IO and parsing (IoContext).
- [ ] Define retry behavior (None for current scope).
- [x] Add graceful shutdown/interrupt handling for long tasks (init in main).

## 11. Security Hardening

- [x] Validate all external inputs (files, JSON tables, pattern strings).
- [x] Guard against path misuse and unsafe file writes.
- [x] Add maximum workload threshold (10^12 candidates).
- [x] Document ethical/legal use constraints in CLI help.
- [x] Add dependency audit step (`cargo audit`) in CI.

## 12. Documentation Revamp

- [ ] Rewrite `README.md` to match new architecture and commands.
- [ ] Add `docs/architecture.md` with diagrams and flow.
- [ ] Add `docs/configuration.md` for all flags and behavior.
- [ ] Add `docs/performance.md` with benchmark methodology.
- [ ] Add migration notes from pre-revamp versions.

## 13. CI/CD and Release Engineering

- [x] Add CI workflow for:
- [x] `cargo fmt --check`
- [x] `cargo clippy -- -D warnings`
- [x] `cargo test`
- [x] `cargo build --release`
- [x] Add matrix builds (Linux/macOS/Windows).
- [x] Add release workflow for tagged versions.
- [ ] Add changelog automation or release note template.

## 14. Packaging and Distribution

- [ ] Verify reproducible release builds.
- [ ] Add install guidance for Cargo and binary releases.
- [ ] Add checksums/signatures for release artifacts.
- [ ] Validate `--help` and command examples in packaged binaries.

## 15. Backlog Items (Optional, Post-Revamp)

- [ ] Add plugin architecture for custom candidate generators.
- [ ] Add state checkpointing/resume for brute-force.
- [ ] Add TUI mode for long-running sessions.
- [ ] Add optional machine-readable telemetry output.

## 16. Final Acceptance Checklist

- [ ] Feature parity validated against legacy command behavior.
- [ ] Performance meets or exceeds baseline targets.
- [ ] Test coverage targets met.
- [ ] Security checks passing.
- [ ] Docs and examples verified end-to-end.
- [ ] Release candidate signed off.
