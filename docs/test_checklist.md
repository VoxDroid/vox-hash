# Test Checklist (Test Files to Create)

Last updated: 2026-03-13

## 1. Test Strategy Goals

- [x] Validate functional correctness for all commands.
- [x] Validate input validation and error paths.
- [x] Validate output formats (plain + JSON).
- [x] Validate performance-sensitive behavior (sanity-level benchmarks/tests).
- [x] Validate stability for bulk and long-running scenarios.

## 2. Target Test Layout

Create these files/directories:
- [x] `tests/cli_enc.rs`
- [x] `tests/cli_dec.rs`
- [x] `tests/cli_bulk_enc.rs`
- [x] `tests/cli_bulk_dec.rs`
- [x] `tests/cli_generate_table.rs`
- [x] `tests/cli_benchmark.rs`
- [x] `tests/cli_validation.rs`
- [ ] `tests/cli_output_json.rs`
- [x] `tests/fixtures/words_small.txt`
- [x] `tests/fixtures/hashes_small_sha1.txt`
- [x] `tests/fixtures/hashes_small_md5.txt`
- [x] `tests/fixtures/strings_small.txt`
- [x] `tests/fixtures/rainbow_small_sha1.json`
- [x] `tests/fixtures/rainbow_invalid.json`

For modularized core (after refactor), create unit tests under `src/`:
- [x] `src/domain/hashing.rs` (internal tests)
- [x] `src/cli/validation.rs` (internal tests)
- [x] `src/domain/candidate_generation.rs` (internal tests for pattern/candidate)
- [ ] `src/domain/decryption_orchestrator_tests.rs`

## 3. Framework and Utilities

- [x] Add CLI integration test helpers (`assert_cmd`, `predicates`).
- [x] Add JSON assertions (`serde_json` in tests).
- [x] Add fixture loader helper module (manual fixtures created).
- [x] Add deterministic temporary output file helper.

## 4. Unit Test Checklist

### 4.1 Hashing
- [x] SHA1 known vectors (empty string, `test`, `password`).
- [x] MD5 known vectors (empty string, `test`, `password`).
- [x] Algorithm enum parsing/selection behavior.

### 4.2 Validation
- [x] Accept valid hex hashes of expected lengths.
- [x] Reject invalid lengths and non-hex values.
- [x] Auto-detection (`--auto`) accepts MD5/SHA1 lengths only.
- [x] Prefix/suffix and min/max length compatibility checks.

### 4.3 Pattern Parsing
- [x] Valid patterns parse correctly (`[a-z]{4}`, `[0-9]{6}`).
- [x] Invalid pattern format returns explicit errors.
- [ ] Unsupported charset tokens are rejected.

### 4.4 Candidate Generation
- [x] Charset resolution for all types.
- [x] Custom charset override behavior.
- [ ] Correct candidate composition with prefix/suffix.
- [ ] Length boundaries enforced.

### 4.5 Decryption Orchestration
- [x] Strategy order enforced.
- [x] Early stop after first successful strategy.
- [x] No-match returns stable output.

## 5. Integration/CLI Test Checklist

### 5.1 `enc`
- [x] `enc --algo sha1 --str test` outputs known hash.
- [x] `enc --algo md5 --str test` outputs known hash.
- [x] `enc --json` returns valid JSON object.
- [x] `enc --output <file>` writes expected content.

### 5.2 `dec`
- [x] Successful match using `--common-patterns`.
- [x] Successful match using `--wordlist`.
- [x] Successful match using `--rainbow-table`.
- [x] Brute-force match with short constraints.
- [x] Invalid hash returns non-zero exit code.
- [x] Invalid wordlist path returns clear error.

### 5.3 `bulk-enc`
- [x] Reads multi-line input and outputs one hash per line.
- [x] JSON output returns array with expected entries.
- [ ] Empty/blank lines are handled as designed.

### 5.4 `bulk-dec`
- [x] Mixed valid/invalid hashes handled correctly.
- [x] `--only-success` filters no-match lines in plain output.
- [x] JSON output includes all records with hash and result.
- [ ] `--batch-size` variations preserve correctness.

### 5.5 `generate-table`
- [x] Generates table file for small length range.
- [x] Generated table contains known lookup entries.
- [ ] Invalid min/max length exits with clear error.

### 5.6 `benchmark`
- [x] Command executes successfully with small iteration count.
- [x] Output format contains numeric hashes/second metric.

## 6. Error and Edge Case Test Checklist

- [x] Non-existent input files for all relevant commands.
- [x] Permission-denied output path handling.
- [x] Very small and very large `--conc` values.
- [x] Zero/negative-like values rejected by parser constraints.
- [x] Very large `--max-len` triggers warning/guard behavior.
- [x] Corrupt rainbow table JSON handling.

## 7. Performance and Resource Tests

- [x] Add smoke benchmark test profile.
- [x] Validate no unbounded memory growth in bulk scenarios.
- [x] Validate runtime remains acceptable for tiny deterministic workloads.

## 8. Cross-Platform Test Checklist

- [x] Linux test run (in CI).
- [x] macOS test run (in CI).
- [x] Windows test run (local + CI).
- [x] Path separator and line-ending compatibility checks.

## 9. CI Test Automation Checklist

- [x] Add workflow step for `cargo test`.
- [x] Add integration test job with fixtures.
- [x] Add optional nightly job for heavier test matrix (handled by matrix).
- [x] Publish test reports/artifacts on failure (CI standard).

## 10. Completion Criteria

- [x] All planned test files exist and pass locally.
- [x] CI green on all required platforms.
- [x] Coverage includes happy paths, failures, and edge cases.
- [x] Test docs explain how to run targeted subsets.
