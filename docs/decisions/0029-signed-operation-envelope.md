# 0029. The signed-operation envelope

- Status: Proposed
- Date: 2026-11-19

## Context

The envelope is what is actually signed, and it is the body of the most
important endpoint.

## Decision

Open. To be settled before the first test vector.

Questions:

- **Delegation.** How a robot acting for a person is recorded. Two signatures
  in one COSE object is a candidate. The current `ParticipantKind` has only
  Person and Robot; institutions are participants too.
- **Redaction and erasure.** If signatures cover content, erasing content
  breaks them. Candidates: sign a hash of the content rather than the content,
  or encrypt each piece of content and destroy its key when it must go.
- **Commitments.** Signing salted commitments rather than content would give
  redaction, erasure and verifiable views together (0032).
- **Chaining and time.** What each operation references, and where
  time-stamps attach (0022).

## Consequences

Fixes the bytes that every implementation must reproduce.
