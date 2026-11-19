# 0020. No passwords, ever

- Status: Accepted
- Date: 2026-11-19

## Context

Passwords are the most common way accounts are lost.

## Decision

No part of Wave asks for, stores or accepts a password, and nothing stands in
for one. People authenticate with passkeys.

## Consequences

Passkeys are built on WebAuthn, which uses the same CBOR and COSE as the
record (see 0015).

Recovery and doors without a screen need their own rules (see 0021).
