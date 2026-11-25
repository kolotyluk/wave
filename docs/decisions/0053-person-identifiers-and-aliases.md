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

Illustration, in Rust, of how the states make a link without proof
impossible to construct:

```rust
/// How an alias relates to a person's root identity.
pub enum AliasState {
    /// Proven by the person while authenticated.
    Linked { proof: Proof, linked_at: Timestamp },
    /// Inferred by Wave. Private to the identity service; never a finding.
    Suspected { signals: Vec<Signal>, since: Timestamp },
    /// The person said "not me". Remembered so Wave stops asking.
    Rejected { by: Principal, at: Timestamp },
}

/// What proved control of an alias. Each refers to its record.
pub enum Proof {
    EmailCode(OperationId),
    DnsRecord(OperationId),
    AuthorizedDecision { by: Principal, record: OperationId },
}
```

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

## Open questions

- **Pairwise identifiers.** How they are derived and kept stable across a
  change of home, and how federated servers resolve them for routing without
  learning the root.
- **People with no ID, phone or fixed address.** A person with a housing
  problem may be using a library computer, with no smartphone and no
  government ID to hand. Knowledge factors (eye colour, hair colour, date of
  birth, high school) are easy for them to give, but are known to exactly the
  people a vulnerable person may need to keep out, such as an abusive partner
  or relative, and often appear online or in breached data. Proposed: such
  factors are one signal among several and can unlock low-risk actions
  (continuing one's own conversation, reading general advice), never
  sensitive content or acting on the person's behalf. Stronger options for
  this person: a FIDO security key issued by a trusted intermediary (library,
  shelter, legal aid, Service BC), which works as a passkey on a shared
  computer with nothing stored there; a printed card with a one-time setup
  code issued in person, like BC's Token; vouching by an intermediary at the
  assurance their role allows; and shared-computer hygiene (no saved login,
  sessions that end with the browser).
- **Minors and guardians.** Who acts for a child, and how and when the child
  takes over their own identity.
- **Incapacity and death.** When a power of attorney takes effect, what
  becomes of an identity when its person dies, and who may act for an
  estate. The record outlives the person.
- **Name changes.** After a legal name change or a change of gender identity,
  the identity is the same and the displayed name changes; earlier signed
  statements stay verifiable without exposing the former name where it
  should not be seen.
- **Key compromise.** Every key has a recorded revocation time, so that what a
  person genuinely signed before a theft stays valid, and nothing signed with
  the stolen key afterwards is accepted.
