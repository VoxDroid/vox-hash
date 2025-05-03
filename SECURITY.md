# Security Policy

## Supported Versions

The following versions of **vox-hash** are currently supported with security updates:

| Version | Supported          |
|---------|--------------------|
| 1.3     | ✅                 |
| Future  | ✅ (Latest release) |

We recommend using the latest version from the [repository](https://github.com/VoxDroid/vox-hash) to ensure you have the most recent security fixes and improvements.

## Reporting a Vulnerability

If you discover a security vulnerability in vox-hash, we appreciate your help in disclosing it responsibly. Please follow these steps:

1. **Do Not Disclose Publicly**: Avoid sharing details of the vulnerability in public forums, such as GitHub issues, social media, or other platforms, until it has been addressed.
2. **Contact the Maintainer Privately**:
   - Email [izeno.contact@gmail.com](mailto:izeno.contact@gmail.com) with a detailed description of the vulnerability, steps to reproduce, and potential impact.
   - Alternatively, create a private issue or discussion on the [GitHub repository](https://github.com/VoxDroid/vox-hash).
3. **Response Time**:
   - You can expect an initial response within 48 hours.
   - We will work with you to validate and address the issue promptly.
4. **Disclosure**:
   - Once the vulnerability is fixed, we will coordinate with you on public disclosure, if appropriate.
   - Credit will be given for your discovery in release notes, unless you prefer anonymity.

## Security Best Practices

To keep your use of vox-hash secure:

- **Use Trusted Sources**: Download or clone the project only from the official [GitHub repository](https://github.com/VoxDroid/vox-hash).
- **Secure Dependencies**: Regularly update Rust dependencies via `cargo update`. Ensure dependencies (`clap`, `sha1`, `md5`, etc.) are from trusted sources.
- **Input Validation**: vox-hash validates hash formats and file inputs, but avoid processing untrusted wordlists, hash files, or rainbow tables to prevent potential injection or parsing issues.
- **File System Access**: vox-hash reads and writes to user-specified paths. Ensure input/output directories are secure to prevent unauthorized access. Avoid running as root unless necessary.
- **Resource Management**: Brute-forcing with large charsets or lengths can consume significant CPU and memory. Monitor system resources and use `--conc` and `--batch-size` appropriately.
- **Ethical Use**: Use vox-hash only for legal and ethical purposes, such as security testing with permission. Hash cracking may be restricted in some jurisdictions.
- **Data Privacy**: Avoid including sensitive data in input files (e.g., real passwords in wordlists), as vox-hash does not encrypt inputs or outputs.

## Known Dependencies

vox-hash relies on the following third-party dependencies, which may have their own security policies:

- **Rust Libraries** (via Cargo):
  - `clap`: CLI argument parsing.
  - `sha1`, `md5`: Hashing algorithms.
  - `rayon`: Parallel processing.
  - `indicatif`: Progress bars.
  - `chrono`: Timestamp logging.
  - `serde_json`: JSON handling.
  - `regex`: Pattern parsing.

Check the respective project pages for security advisories and ensure you’re using the versions specified in `Cargo.toml` or their latest secure releases.

Thank you for helping keep vox-hash secure!