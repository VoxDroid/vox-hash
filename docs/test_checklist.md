# Test Checklist (Test Files to Create)

Last updated: 2026-03-13

## 1. Test Strategy Goals

- [ ] Validate functional correctness for all commands.
- [ ] Validate input validation and error paths.
- [ ] Validate output formats (plain + JSON).
- [ ] Validate performance-sensitive behavior (sanity-level benchmarks/tests).
- [ ] Validate stability for bulk and long-running scenarios.

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
- [ ] `tests/fixtures/words_small.txt`
- [ ] `tests/fixtures/hashes_small_sha1.txt`
- [ ] `tests/fixtures/hashes_small_md5.txt`
- [ ] `tests/fixtures/strings_small.txt`
- [ ] `tests/fixtures/rainbow_small_sha1.json`
- [ ] `tests/fixtures/rainbow_invalid.json`

For modularized core (after refactor), create unit tests under `src/`:
- [ ] `src/domain/hashing_tests.rs`
- [ ] `src/domain/validation_tests.rs`
- [ ] `src/domain/pattern_tests.rs`
- [ ] `src/domain/candidate_generation_tests.rs`
- [ ] `src/domain/decryption_orchestrator_tests.rs`

## 3. Framework and Utilities

- [ ] Add CLI integration test helpers (`assert_cmd`, `predicates`, `tempfile`).
- [ ] Add JSON assertions (`serde_json` in tests).
- [ ] Add fixture loader helper module (`tests/common/mod.rs`).
- [ ] Add deterministic temporary output file helper.

## 4. Unit Test Checklist

### 4.1 Hashing
- [ ] SHA1 known vectors (empty string, `test`, `password`).
- [ ] MD5 known vectors (empty string, `test`, `password`).
- [ ] Algorithm enum parsing/selection behavior.

### 4.2 Validation
- [ ] Accept valid hex hashes of expected lengths.
- [ ] Reject invalid lengths and non-hex values.
- [ ] Auto-detection (`--auto`) accepts MD5/SHA1 lengths only.
- [ ] Prefix/suffix and min/max length compatibility checks.

### 4.3 Pattern Parsing
- [ ] Valid patterns parse correctly (`[a-z]{4}`, `[0-9]{6}`).
- [ ] Invalid pattern format returns explicit errors.
- [ ] Unsupported charset tokens are rejected.

### 4.4 Candidate Generation
- [ ] Charset resolution for all types.
- [ ] Custom charset override behavior.
- [ ] Correct candidate composition with prefix/suffix.
- [ ] Length boundaries enforced.

### 4.5 Decryption Orchestration
- [ ] Strategy order enforced.
- [ ] Early stop after first successful strategy.
- [ ] No-match returns stable output.

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

- [ ] Non-existent input files for all relevant commands.
- [ ] Permission-denied output path handling.
- [ ] Very small and very large `--conc` values.
- [ ] Zero/negative-like values rejected by parser constraints.
- [ ] Very large `--max-len` triggers warning/guard behavior.
- [ ] Corrupt rainbow table JSON handling.

## 7. Performance and Resource Tests

- [ ] Add smoke benchmark test profile (not strict timing assertions).
- [ ] Validate no unbounded memory growth in bulk scenarios.
- [ ] Validate runtime remains acceptable for tiny deterministic workloads.

## 8. Cross-Platform Test Checklist

- [ ] Linux test run.
- [ ] macOS test run.
- [ ] Windows test run.
- [ ] Path separator and line-ending compatibility checks.

## 9. CI Test Automation Checklist

- [ ] Add workflow step for `cargo test`.
- [ ] Add integration test job with fixtures.
- [ ] Add optional nightly job for heavier test matrix.
- [ ] Publish test reports/artifacts on failure.

## 10. Completion Criteria

- [ ] All planned test files exist and pass locally.
- [ ] CI green on all required platforms.
- [ ] Coverage includes happy paths, failures, and edge cases.
- [ ] Test docs explain how to run targeted subsets.
