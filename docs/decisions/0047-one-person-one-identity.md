# 0047. One person, one identity

- Status: Accepted
- Date: 2026-11-21

## Context

A person may have a dozen email addresses, several phone numbers and
passkeys on several devices. If each became an identity, one person could
appear as many: sock-puppet accounts in a consultation, a manufactured
groundswell, a party in a dispute under several names. That would corrupt the
record.

0021 also allows anonymous and pseudonymous participation, and the discussion
paper calls for controls against silent linkage of anonymous activity to a
known person.

## Decision

**Wave resists one person having multiple identities.** Each person has one
root identity.

**Aliases tie back to it.** Email addresses, phone numbers, passkeys and other
identifiers are aliases of the one person, never separate people.

**Pseudonyms are presentations, not identities.** A person may appear in a
Wave under a pseudonym. The pseudonym ties back to their root identity, but
who can see that tie is tightly limited: the identity service holds it, other
participants do not see it, and it is revealed only by the person's choice or
by lawful process.

**Anonymity is no identity claim, not another identity.** Anonymous actions
cannot count as distinct people where counting matters, such as votes,
endorsements or signatures.

**Resistance rises with assurance.** At low assurance, Wave resists duplicates
by matching aliases, detecting patterns, and merging identities when
duplicates are found; a merge is a recorded operation. At verified assurance,
a verified credential makes one person to one identity achievable.

**Robots and institutions follow the same rule.** A robot always ties back to
its principal (0038). An institution is one identity, with many people acting
for it.

**Every person agrees to it.** Participation requires a user agreement in
which the person undertakes to act only as themselves, or as a declared
delegate for someone else: a lawyer for a client, an attorney under a power of
attorney, a support worker for the person they help, a parent for a child. A
delegate always states that they are acting on another's behalf, and the
delegation is recorded in Wave, with evidence of the authority (such as the
power of attorney) at the assurance the action requires. What is forbidden is
undisclosed representation: posing as someone else, or as more than one
person. Accepting the agreement is a signed operation (0038), so the record
shows which version each person agreed to, and when.

**Violations have consequences, always lawful and with due process.**
Enforcement is graduated:
a warning; restrictions, such as requiring higher assurance or removing the
ability to endorse or vote; then suspension or loss of access. Each step
comes with notice of what was found and a right of appeal (moderation with
process). Losing access to a public service can cause real harm, so
proportionality matters.

**Fines need lawful authority.** That authority can come from statute, as for
the integrity offences the discussion paper proposes, or from the lawful rules
of a membership body: a housing co-operative's rules may impose fines and
levies, and unpaid amounts put a member in arrears, which is serious. Every
adverse action, from a warning to a fine to loss of access, must rest on such
authority, follow the body's own procedure, and be proportionate. Wave
records the agreement, the evidence, the notice, the response and the
decision; the lawful authority decides.

## Consequences

Linking aliases collects personal information: it needs notice and consent,
and the links are protected like the rest of the identity directory.

Open: the matching and merging methods at low assurance, and how a mistaken
merge is undone.
