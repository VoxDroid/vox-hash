# Revamp Implementation Checklist (From Scratch)

Last updated: 2026-03-13

## 0. Project Setup and Governance

- [ ] Confirm revamp scope and success metrics.
- [ ] Freeze current behavior expectations from `README.md` and current CLI help.
- [ ] Define semantic versioning and migration policy.
- [ ] Set coding standards (fmt, clippy, error handling, docs).
- [ ] Add/update `CONTRIBUTING.md` revamp section.
- [ ] Define branching and release strategy.

## 1. Architecture and Repository Restructure

- [ ] Create modular folder layout (`cli`, `app`, `domain`, `infra`).
- [ ] Move business logic out of `src/main.rs`.
- [ ] Add `config` and typed request/response models.
- [ ] Add typed error model (`thiserror` optional) and unified result handling.
- [ ] Introduce trait boundaries for hash algorithms and match providers.
- [ ] Add architecture decision record (ADR) notes.

## 2. CLI Layer Rebuild

- [ ] Re-declare all existing commands and flags in modular CLI structs.
- [ ] Preserve user-facing command names and compatibility aliases if needed.
- [ ] Centralize argument validation and conflict rules.
- [ ] Improve help text examples per command.
- [ ] Add robust parsing for pattern syntax and lengths.
- [ ] Define exit code map for success, validation errors, runtime errors.

## 3. Hashing Core

- [ ] Implement hash algorithm abstraction (`enum` or `trait`).
- [ ] Keep `SHA1` and `MD5` parity.
- [ ] Normalize and validate hash input casing/format.
- [ ] Add deterministic result formatting.
- [ ] Design extension point for future algorithms.

## 4. Candidate Generation Engine

- [ ] Rebuild charset providers (`alphanumeric`, `lowercase`, `uppercase`, `digits`, custom).
- [ ] Implement prefix/suffix-aware candidate builder.
- [ ] Implement min/max/fixed length controller.
- [ ] Implement pattern parser and validator with explicit error messages.
- [ ] Add limits/guards to prevent accidental explosive workloads.

## 5. Matching Pipeline

- [ ] Build orchestrator with explicit strategy order:
- [ ] `rainbow_table -> common_patterns -> wordlist -> brute_force`.
- [ ] Make strategy ordering configurable (optional enhancement).
- [ ] Add per-strategy timing and counters.
- [ ] Support early-exit signal across workers.
- [ ] Ensure deterministic behavior in no-match outcomes.

## 6. Bulk Processing Rebuild

- [ ] Implement streaming line reader for large input files.
- [ ] Implement chunked/batch processing with configurable batch size.
- [ ] Add memory-safe defaults for large datasets.
- [ ] Preserve `--only-success` behavior in plain output.
- [ ] Ensure JSON output includes all relevant fields per record.

## 7. Rainbow Table Subsystem

- [ ] Define table file schema and version tag.
- [ ] Implement table generation with bounded resource checks.
- [ ] Implement table loading with parse and schema validation.
- [ ] Consider alternative table storage (optional):
- [ ] newline-delimited JSON, binary, sqlite, or partitioned files.
- [ ] Add corruption/invalid-format error handling.

## 8. Output and UX

- [ ] Standardize structured output models.
- [ ] Keep plain output concise for piping.
- [ ] Make verbose logs consistent and timestamped.
- [ ] Improve progress rendering for long-running operations.
- [ ] Ensure no progress bar noise in non-interactive/noverbose mode.

## 9. Performance and Concurrency

- [ ] Centralize thread pool config and bounds.
- [ ] Add benchmark harness for core operations.
- [ ] Evaluate batching strategy and tune defaults.
- [ ] Prevent nested oversubscription in rayon contexts.
- [ ] Profile CPU/memory for representative workloads.

## 10. Error Handling and Reliability

- [ ] Replace panics/`expect` with recoverable errors where appropriate.
- [ ] Add contextual error messages for file IO and parsing.
- [ ] Define retry behavior (if any) for recoverable operations.
- [ ] Add graceful shutdown/interrupt handling for long tasks.

## 11. Security Hardening

- [ ] Validate all external inputs (files, JSON tables, pattern strings).
- [ ] Guard against path misuse and unsafe file writes.
- [ ] Add optional maximum workload threshold with confirmation bypass.
- [ ] Document ethical/legal use constraints in CLI help.
- [ ] Add dependency audit step (`cargo audit`) in CI.

## 12. Documentation Revamp

- [ ] Rewrite `README.md` to match new architecture and commands.
- [ ] Add `docs/architecture.md` with diagrams and flow.
- [ ] Add `docs/configuration.md` for all flags and behavior.
- [ ] Add `docs/performance.md` with benchmark methodology.
- [ ] Add migration notes from pre-revamp versions.

## 13. CI/CD and Release Engineering

- [ ] Add CI workflow for:
- [ ] `cargo fmt --check`
- [ ] `cargo clippy -- -D warnings`
- [ ] `cargo test`
- [ ] `cargo build --release`
- [ ] Add matrix builds (Linux/macOS/Windows).
- [ ] Add release workflow for tagged versions.
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
