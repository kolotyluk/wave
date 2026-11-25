# 0053. Person identifiers stay private; aliases are proven, never assumed

- Status: Proposed
- Date: 2026-11-24
- Refines: [0028](0028-identifiers.md), [0047](0047-one-person-one-identity.md)

## Context

0047 gives each person one root identity, with aliases tied to it, and left
open how aliases are matched and merged. A person may have a dozen email
addresses, such as `eric.kolotyluk@gmail.com`, `eric.kolotyluk@ztuph.com` and
`eric@kolotyluk.net`.

Canada's Social Insurance Number shows the danger of a universal identifier:
everyone asks for it, it links everything, and stealing it unlocks
everything.

## Decision

Proposed:

**A person's root identity is a random UUID (version 4).** It is globally
unique without a prefix. UUIDv7 stays for records (0028), but its timestamp
would reveal when a person's identity was created, so it is not used for
people or robots.

**The root identity is private.** It is known to the person's home identity
service and to no one else. Each Wave or deployment sees a pairwise
identifier derived for that context, so a person's activity cannot be
correlated across contexts by identifier. Knowing a root identity grants
nothing: only the person's keys prove who they are (0052).

**Identity is independent of the server.** A person's home server vouches
for them; it is an attribute of the identity, not part of it. Moving home is
a signed operation made with the person's keys and acknowledged by both
servers. A form such as `person@server` may serve as an address for routing
and display, but it is not the identity. No server can claim to speak for an
identity whose keys it does not hold.

**Aliases have three states.**

- **Linked:** added by the person while authenticated, after proving control:
  a code sent to the address, or a DNS record for a domain. Only linked
  aliases are part of the identity.
- **Suspected:** inferred by Wave from names, domains, devices or keys. Kept
  privately in the identity service, never shown to other participants, and
  used only to suggest a link to the person and as a signal for fraud review
  (0048): a signal, never a finding.
- **Rejected:** the person has said "not me". Wave remembers, and stops
  asking or treating it as suspicious.

**Nothing is linked automatically.** People share names, families share
domains and mailboxes, and addresses are recycled. A wrong merge is worse
than a missed link. Merges happen by proof of control, or by a recorded
decision of a person with the authority to make it, and can be undone.

**Normalization only where a provider defines it.** Some providers define
equivalent addresses (Gmail ignores dots and anything after `+` in the local
part); such rules apply per provider, never generally.

**Email is for contact and recovery**, not strong authentication: it proves
control of a mailbox, not a person. Keys authenticate.

## Consequences

Inference is processing of personal information: it needs notice in the
deployment's privacy terms, and suspected links are protected like the rest
of the identity directory.

Open: how pairwise identifiers are derived and kept stable across a change of
home, and how federated servers resolve them for routing without learning the
root.
