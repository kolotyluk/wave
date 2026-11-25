# 0051. The Admin Wave: a server is administered through a Wave

- Status: Proposed
- Date: 2026-11-24

## Context

A Wave server needs administration: policies, participation profiles and
Swarm resource grants, robot charters, mandated doors, user-agreement
versions, moderation rosters. Administration done outside Wave would be
recorded, if at all, in some other log, by some other rules.

Wave already offers signed, attributed, replayable records, permissions,
bounds and agreements. Administration can use them.

## Decision

Proposed: **each server is administered through an Admin Wave**, pinned as
the first Wave on the server, with a well-known identifier.

- **Access.** People with write access to the Admin Wave administer the
  server; others may have read access only, for example auditors. Access is
  granted, recorded and revoked like any other permission, at the assurance
  the action requires.
- **Changes are signed operations.** Every administrative change is
  attributed, recorded and replayable, like any other change.
- **The Admin Robot reviews.** An intrinsic robot, `.admin` (0048), reviews each
  proposed change before it takes effect: it explains effects, warns, and may
  ask for confirmation or a second approver for high-risk changes. It always
  defers to an explicit, lawful command from an authorized person. It never
  vetoes.
- **Invariants are enforced by the server, not the robot.** Some commands
  fail whoever gives them: deleting sealed evidence, disabling witnessing or
  time-stamping, hiding a robot, pruning an unsealed tail, lowering a bound's
  protection without its required agreement. These are the server's rules,
  as in robot governance (0039).
- **High-risk changes may need two people.** A two-person rule is a bound on
  the relevant part of the Admin Wave with an agreement requirement (0036,
  0038).
- **No secrets.** The Admin Wave is append-only, so nothing secret is ever
  written into it. Credentials and keys live in the key manager and are
  referred to by name.
- **Scope.** The Admin Wave governs Wave-level configuration. Host
  infrastructure (DNS, certificates, PostgreSQL roles, the runtime) stays in
  host tooling, behind the host boundary (0048); its changes may be recorded
  in the Admin Wave for the history.
- **Published rules come from it.** The deployment's published rules, robot
  charters and user agreements are a public view of the Admin Wave,
  verifiable like any other view (0032).
- **Break-glass.** If a change breaks the server, it cannot be repaired
  through the Admin Wave. A recovery path through the host exists (local
  console or a recovery mode). It requires host-level access, and its use is
  recorded in the Admin Wave when service resumes, and disclosed. It is not a
  back door.

## Consequences

Administration is held to the same standard of evidence as everything else:
the server eats its own cooking.

The Admin Wave depends on most of Wave's capabilities. The skeleton starts
with a plain configuration file and moves administration into the Admin Wave
once the core works.

Open: how the Admin Wave is bootstrapped on a new server, and who holds the
first write access.
