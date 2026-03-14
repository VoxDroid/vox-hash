# Pull Request

Thank you for contributing to **vox-hash**! Please complete this template to help us review your changes efficiently. Ensure your pull request adheres to the [Contributing Guidelines](https://github.com/VoxDroid/vox-hash/blob/main/CONTRIBUTING.md) and the [Code of Conduct](https://github.com/VoxDroid/vox-hash/blob/main/CODE_OF_CONDUCT.md).

## Description

Provide a clear and concise description of the changes in this pull request. Explain:
- What problem it solves or what feature it adds.
- The approach you took to implement the changes.
- Any relevant technical details or trade-offs.

**Example**:
- Adds SHA256 hashing support to `enc` and `dec` commands.
- Updates `hash_string` function to include SHA256 via the `sha2` crate.
- Modifies CLI to accept `sha256` as an `--algo` option.

## Related Issues

List any GitHub issues this pull request addresses. Use the format `Fixes #123` or `Closes #123` to automatically link and close issues upon merging.

- Fixes # [issue number]
- Related to # [issue number]

## Type of Change

Check the appropriate box(es) to indicate the type of change:

- [ ] Bug fix (non-breaking change that resolves an issue)
- [ ] New feature (non-breaking change that adds functionality)
- [ ] Breaking change (fix or feature that would cause existing functionality to change)
- [ ] Documentation update (changes to README, CONTRIBUTING, or other docs)
- [ ] Code style or refactoring (no functional changes)
- [ ] Other (please describe):

## How Has This Been Tested?

Describe the testing you performed to verify your changes. Include:

- Manual tests (e.g., tested commands with sample inputs, wordlists).
- Platform testing (e.g., Linux, Windows, macOS).
- Edge cases considered (e.g., invalid inputs, large files).
- Outputs or screenshots (e.g., CLI output, JSON results).
- Environment tested (e.g., Rust 1.70, Ubuntu 22.04).

**Example**:
- Tested SHA256 hashing with `enc --algo sha256 --str test`.
- Verified `dec` with known SHA256 hashes using a wordlist.
- Ran `bulk-enc` and `bulk-dec` with a 1,000-line input file.
- Tested on Ubuntu 22.04 and Windows 11 with Rust 1.70.
- Included screenshot of `dec` command output.

## Screenshots or Outputs (if applicable)

If your changes affect CLI output or functionality, attach screenshots or text outputs to demonstrate the results. For example, show a new command’s output, progress bar, or JSON format.

## Checklist

Please confirm the following before submitting your pull request:

- [ ] My code follows the [Code Style Guidelines](CONTRIBUTING.md#code-style-guidelines) (e.g., Rust fmt, 4-space indentation).
- [ ] I have tested my changes on at least one platform (e.g., Linux, Windows, macOS).
- [ ] I have updated the documentation (e.g., README, CLI help) if my changes impact usage or setup.
- [ ] My changes do not introduce new errors or warnings (verified with `cargo clippy`).
- [ ] My pull request targets the `main` branch.
- [ ] I have run `cargo fmt` and `cargo test` to ensure formatting and tests pass.
- [ ] I have reviewed my changes to ensure they are focused and do not include unrelated modifications.

## Additional Context

Provide any additional information that might help reviewers understand your changes. For example:
- Why you chose a specific approach (e.g., performance considerations).
- Any limitations or known issues with your implementation.
- Future improvements you plan to address.

---

**Note**: Maintainers may request changes or clarification during the review process. Please respond promptly to feedback to ensure a smooth merge.

Thank you for your contribution to vox-hash!
