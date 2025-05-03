# Support for vox-hash

Thank you for using **vox-hash**! We’re dedicated to ensuring you have a great experience with this Rust-based CLI tool for hashing and brute-force hash matching. This document outlines how to seek support, report issues, request features, and find additional resources.

## Table of Contents

- [Getting Help](#getting-help)
- [Reporting Bugs](#reporting-bugs)
- [Requesting Features](#requesting-features)
- [FAQs](#faqs)
- [Community and Contact](#community-and-contact)
- [Supporting the Project](#supporting-the-project)

## Getting Help

If you encounter issues or have questions about vox-hash, follow these steps:

1. **Check the Documentation**:
   - Review the [README](README.md) for installation, usage, and command details.
   - Ensure your environment meets the [System Requirements](README.md#system-requirements).

2. **Search Existing Issues**:
   - Visit the [Issues page](https://github.com/VoxDroid/vox-hash/issues) to see if your question or issue has been addressed.

3. **Explore FAQs**:
   - Check the [FAQs](#faqs) section below for solutions to common problems.

4. **Ask the Community**:
   - Post your question in the [GitHub Discussions](https://github.com/VoxDroid/vox-hash/discussions) section for community support.

5. **Contact the Maintainer**:
   - For private or urgent matters, email [izeno.contact@gmail.com](mailto:izeno.contact@gmail.com) or create a private issue or discussion on the [GitHub repository](https://github.com/VoxDroid/vox-hash).

## Reporting Bugs

If you find a bug in vox-hash:

1. **Verify the Issue**:
   - Ensure you’re using the latest version (1.3) from the [repository](https://github.com/VoxDroid/vox-hash).
   - Reproduce the issue with a sample command and input file.

2. **Submit a Bug Report**:
   - Open a new issue on the [Issues page](https://github.com/VoxDroid/vox-hash/issues).
   - Use the bug report template and include:
     - A clear title and description.
     - Steps to reproduce the bug (e.g., command, input file).
     - Expected vs. actual behavior.
     - Screenshots, error messages, or verbose logs (`--noverbose false`).
     - Your environment (e.g., OS, Rust version, file size).

3. **Follow Up**:
   - Respond to any questions or requests for clarification from maintainers.
   - Test any proposed fixes if requested.

**Example**:
- Title: "bulk-dec Fails with Large Hash File"
- Steps: Run `vox-hash bulk-dec --input hashes.txt --auto` with 10,000 hashes.
- Expected: Processes all hashes with results.
- Actual: Crashes with "out of memory" error.
- Environment: Rust 1.70, Windows 11, 4GB RAM.

## Requesting Features

Have an idea to improve vox-hash? We’d love to hear it!

1. **Check for Duplicates**:
   - Search the [Issues page](https://github.com/VoxDroid/vox-hash/issues) to ensure your feature hasn’t been suggested.

2. **Submit a Feature Request**:
   - Open a new issue using the feature request template.
   - Provide:
     - A clear title and detailed description.
     - The problem the feature solves or the benefit it provides.
     - Any examples, code snippets, or references to similar functionality.

3. **Engage with Feedback**:
   - Discuss your idea with maintainers and the community.
   - Be open to refining the proposal based on feedback.

**Example**:
- Title: "Add Incremental Brute-Force Checkpointing"
- Description: Save brute-force progress to resume after interruption.
- Benefit: Improves usability for long-running tasks.

## FAQs

**Q: Why does vox-hash report "Input file does not exist"?**  
A: Ensure the file path is correct and accessible. For example, use `words.txt` in the current directory or provide a full path (e.g., `/path/to/words.txt`). Check file permissions and run `ls` or `dir` to verify.

**Q: Why is brute-forcing slow for large max lengths?**  
A: Large `--max-len` values (e.g., >8) or broad charsets (e.g., alphanumeric) create billions of combinations. Use `--wordlist`, `--rainbow-table`, or `--pattern` to narrow the search. Increase `--conc` for more threads if your system supports it.

**Q: How do I create a wordlist or hash file?**  
A: Create a text file with one entry per line. Example `words.txt`:
```
password
admin
test
```
Example `hashes.txt`:
```
5baa61e4c9b93f3f0682250b6cf8331b7ee68fd8
d41d8cd98f00b204e9800998ecf8427e
```

**Q: Why does the rainbow table generation fail with large lengths?**  
A: Generating tables for `--max-len` > 4 can produce massive files and consume significant memory. Try smaller lengths (e.g., 1-3) or a smaller charset (e.g., `--charset-type digits`). Ensure sufficient disk space.

**Q: How do I optimize brute-force performance?**  
A: Use a wordlist or rainbow table for faster lookups. Specify a pattern (e.g., `[0-9]{4}`) or reduce `--max-len`. Increase `--conc` (e.g., 50) on high-core CPUs, but monitor memory usage. Run on a system with ample RAM and CPU cores.

**Q: Why does JSON output not work as expected?**  
A: Ensure the `--json` flag is used (e.g., `vox-hash enc --str test --json`). If writing to a file, verify the output path is writable. Check for syntax errors in input files for `bulk` commands.

## Community and Contact

Join the vox-hash community to connect with other users and the maintainer:

- **GitHub Discussions**: Ask questions, share ideas, or discuss features in the [Discussions](https://github.com/VoxDroid/vox-hash/discussions) section.
- **Issues Page**: Report bugs or request features at [Issues](https://github.com/VoxDroid/vox-hash/issues).
- **Email**: Contact the maintainer at [izeno.contact@gmail.com](mailto:izeno.contact@gmail.com).

We aim to respond to questions and issues within 48 hours, though community responses may be faster.

## Supporting the Project

vox-hash is free and open-source, but your support helps maintain and improve it! Here’s how you can contribute:

- **Contribute**: Help improve the code, documentation, or community by following the [Contributing Guidelines](CONTRIBUTING.md).
- **Star the Repository**: Show your support by starring the project on [GitHub](https://github.com/VoxDroid/vox-hash).
- **Donate**: Support development via [Ko-fi](https://ko-fi.com/O4O6LO7Q1).
- **Spread the Word**: Share vox-hash with security researchers, developers, or on social media.
- **Provide Feedback**: Report bugs or suggest features to enhance the tool.

Thank you for your support and for using vox-hash!