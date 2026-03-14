# Configuration Guide

This document describes the various flags and configuration options available in Vox-Hash.

## Global Options

- `--noverbose`: Disables the progress bars and detailed logging. Useful for scripting and piping.
- `--max-len <len>`: Global maximum length for brute-force operations (default: 6).
- `--charset-type <type>`: Selects a predefined charset:
  - `alphanumeric`: abc...z0...9
  - `lowercase`: abc...z
  - `uppercase`: ABC...Z
  - `digits`: 0...9
  - `custom`: Uses the string provided in `--charset`.
- `--charset <string>`: Provides a custom string of characters to use for brute-force.

## Command-Specific Options

### `dec` and `bulk-dec`
- `--key <hash>`: The target hash to decrypt (only for `dec`).
- `--input <file>`: File containing one hash per line (only for `bulk-dec`).
- `--auto`: Automatically detects the hashing algorithm based on length (MD5=32, SHA1=40).
- `--algo <algo>`: Explicitly sets the algorithm (sha1 or md5).
- `--conc <n>`: Number of concurrent threads for brute-force (default: 20).
- `--wordlist <path>`: Path to a plain text dictionary file.
- `--rainbow-table <path>`: Path to a structured JSON rainbow table.
- `--pattern <pattern>`: Use a specific pattern format (e.g., `[a-z]{4}`).
- `--min-len <n>`: Minimum length for brute-force candidates.
- `--length <n>`: Fixes the length of candidates (overrides min/max).
- `--prefix <string>`: Constant string prepended to all candidates.
- `--suffix <string>`: Constant string appended to all candidates.
- `--only-success`: For bulk operations, only displays hashes that were successfully cracked.

### `enc` and `bulk-enc`
- `--str <string>`: The string to hash (only for `enc`).
- `--input <file>`: File containing one string per line (only for `bulk-enc`).
- `--json`: Outputs the result in JSON format.

## Patterns

Vox-Hash supports a specialized pattern syntax: `[charset]{length}`.
- `charset`: A list of characters or character ranges (e.g., `a-z`, `0-9`).
- `length`: A fixed number of characters to generate.

Example: `[a-z0-9]{4}` will try all combinations of 4 lowercase letters and numbers.
