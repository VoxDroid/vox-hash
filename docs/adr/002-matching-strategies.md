# ADR 002: Matching Strategy Abstraction via MatchProvider Trait

## Status
Accepted

## Context
Decryption in `vox-hash` involves multiple strategies: rainbow tables, common patterns, wordlists, and brute force. Hardcoding the order and logic in use cases made them complex and hard to modify.

## Decision
We introduced a `MatchProvider` trait and a `MatchingOrchestrator`. Each strategy implements `MatchProvider`. The orchestrator executes them in sequence.

## Consequences
- **Pros**:
  - Strategy order is easily adjustable.
  - New strategies can be added without modifying core use case logic.
  - Providers can be shared or pre-loaded for bulk processing.
- **Cons**:
  - Small overhead due to trait objects (dispatching).
