# 0015. Operations are encoded in deterministic CBOR and signed with COSE

- Status: Proposed
- Date: 2026-11-19

## Context

Every operation is signed, and a signature is over exact bytes. Two
implementations must produce the same bytes for the same operation.

Protobuf, which Apache Wave used, is not canonical: the same message may
encode to different bytes. JSON has no single byte form without an added
canonicalization scheme.

## Decision

Proposed: operations are encoded in CBOR (RFC 8949) using its deterministic
encoding, and signed with COSE (RFC 9052). Message shapes are described in
CDDL (RFC 8610).

Wave uses a profile of CBOR that maps cleanly to JSON: string keys, integers
in a safe range, no floating point in signed operations, and one fixed JSON
form (base64url) for byte strings.

CBOR diagnostic notation (RFC 8949, section 8) is the human-readable form for
inspection and for test vectors.

## Consequences

The same data always gives the same bytes, so signatures mean something
across implementations.

COSE allows several signers on one object, which suits a robot and the person
it acts for.

The same encoding is used by the AT Protocol, ISO mobile driving licences and
C2PA content credentials, and by passkeys (WebAuthn).

Compression is applied in transport or storage, never to what is signed.

To be confirmed before the first test vector is written.
