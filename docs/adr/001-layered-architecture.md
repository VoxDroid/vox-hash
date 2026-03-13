# ADR 001: Layered Architecture for Vox-Hash Revamp

## Status
Accepted

## Context
The previous version of `vox-hash` was a monolithic single-file application. This made it difficult to test, maintain, and extend. As the features grew (bulk processing, patterns, rainbow tables), the complexity in `main.rs` became unmanageable.

## Decision
We decided to adopt a layered architecture consisting of:
1. **CLI Layer**: Responsible for argument parsing and command dispatch.
2. **App Layer**: Orchestrates use cases. Each command has a corresponding use case.
3. **Domain Layer**: Contains the core business logic, algorithms, and domain models. Independent of I/O.
4. **Infra Layer**: Handles file system, networking, and other external integrations.

## Consequences
- **Pros**:
  - Better testability (can unit test domain logic without CLI).
  - Clearer separation of concerns.
  - Easier to add new hashing algorithms or matching strategies via traits.
- **Cons**:
  - More boilerplate and files.
  - Initial refactoring effort.
