# 0036. Bounds: visible demarcations with declared rules

- Status: Proposed
- Date: 2026-11-20

## Context

Parts of a Wave follow different rules: without-prejudice discussion, sealed
material, phases of a proceeding, retention periods, consent scopes,
jurisdictions. Recording what happened in a log is not enough. People need to
see, before and while they act, which rules apply where.

X.500's administrative model (X.501) solved this problem for directory trees,
and LDAP carried it forward as subentries (RFC 3672) and collective attributes
(RFC 3671).

## Decision

Proposed: a **bound** is a named, visible demarcation around a set of
wavelets or a span of history, with a declared purpose and rules. Its design
follows the X.501 administrative model, without its machinery.

- **Administrative point.** The wavelet where a bound begins is marked as its
  starting point.
- **The bound is an object.** Its purpose and rules live in a visible object in
  the Wave, as X.500's subentries do, not in hidden metadata.
- **Scope is specified precisely.** A base, its descendants, exclusions
  ("chop" in X.501) and filters by kind. An exit, such as a settlement
  outcome, is an exclusion.
- **Kinds of area.** An *autonomous* bound is one authority over a whole region:
  the home whose admission is final (0005), such as a court over its
  proceeding. *Specific* bounds carry one concern each (without prejudice,
  sealing, retention, phase) and overlap independently. *Inner* bounds nest
  within them.
- **Prescriptive and local rules.** Rules on a bound apply to everything in it;
  rules on a single wavelet apply to it alone. Precedence between them is
  defined, never left to chance.
- **Inherited properties.** A property declared on a bound ("governed by BC
  law", "retain seven years") applies to everything inside without being
  copied, as X.500's collective attributes do.
- **Bounds can cover history.** A bound may be drawn around a span of
  operations already recorded, by the agreement its kind requires.
- **Bounds are visible.** Everyone who can see what a bound contains can see the
  bound. The Side Door shows it as a frame; the Front Door says it in plain
  words.
- **Bounds are recorded.** Creating or changing a bound is a signed operation,
  under the agreement rules for its kind (0038), and part of the evidentiary
  record (0022).

## Consequences

Without prejudice (0037) becomes one kind of bound, not a special mechanism.
Sealing, phases, retention, consent scopes and jurisdiction follow the same
pattern.

The envelope (0029) and the admission rules (0005) must carry bound
membership, so that flow rules can be enforced.
