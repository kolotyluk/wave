# 0050. Attack resistance applies to every Wave

- Status: Accepted
- Date: 2026-11-23

## Context

A small private Wave can be attacked as readily as a large public one.
Anonymous access, many subscribers, expensive queries, federation, and
robot activity introduce different opportunities for resource exhaustion
and unauthorized access. Authenticated participants can also be malicious.

Normal and Swarm participation profiles are under discussion; neither a
participant threshold nor the profiles themselves are settled by this
record.

## Decision

**Denial-of-service resistance and security are cross-cutting requirements
for every Wave and deployment**, independent of size, identity policy, or
subscription model. They are not optional Swarm features.

The requirement is accepted; concrete controls, thresholds, products and
deployment guarantees still need threat modelling, design and testing.
This record does not claim that defenses have been implemented or that
arbitrary attacks can be prevented.

The design must address:

- **Network saturation.** Application limits cannot protect a saturated
  upstream link. Deployments need appropriate provider/network mitigation
  and ingress controls compatible with their transports, including HTTP/3
  and its UDP path. No particular proxy or provider is required here.
- **Bounded work.** Bound connections, streams, request sizes, decompression,
  parsing depth, uploads, cryptographic validation, queries, exports and
  verification work. Perform cheap structural and size checks before
  expensive processing, without treating them as authorization.
- **Fair admission and backpressure.** Use explicit budgets for requests,
  concurrency, storage and computational cost, including aggregate limits
  across instances. One contribution can cause many deliveries or robot
  actions; limit amplification, not just input bytes.
- **No privileged exemption from limits.** People, anonymous sessions,
  robots and federation peers all need appropriate controls. An identity,
  valid signature or source IP is not proof of harmlessness or a unique
  person. Avoid relying on IP limits alone, particularly on shared networks.
- **Trustworthy overload behavior.** Distinguish pending, admitted and
  rejected operations; provide retry semantics and duplicate handling.
  Never silently discard admitted evidence or acknowledge durability not
  achieved. Do not prune unsealed evidence to relieve queue pressure.
- **Isolation and confidentiality.** Preserve authorization and bounds at
  every read, delivery and external side effect. Limit the blast radius of
  a compromised service, robot or tenant. Address injection, hostile files,
  credential theft, replay, request forgery and malicious model instructions;
  model output cannot grant capabilities or bypass admission checks.
- **Safe operations.** Protect management interfaces and secrets, patch
  dependencies, and define monitoring, incident response and recovery.
  Telemetry and rejection logs must themselves be bounded and must not leak
  protected content, credentials or pseudonym-root links.

## Consequences

Capacity and failure testing must include malformed input, expensive valid
requests, slow consumers, duplicate/replayed submissions, queue exhaustion,
robot feedback loops, and malicious peers. Test confidentiality and
authorization as well as availability.

Rate and resource policies must be documented and reviewable. Overload
controls must not silently change voting eligibility, moderation findings,
or the meaning of anonymous participation.

The initial PostgreSQL-backed delivery decision
([0049](0049-postgresql-backed-delivery.md)) does not remove these
requirements. A broker added later would inherit them, not solve them
automatically. Volumetric mitigation and operational resilience remain host
responsibilities; protocol validation and capability boundaries remain
Wave responsibilities.
