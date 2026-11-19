# 0021. Identity assurance is a spectrum

- Status: Accepted
- Date: 2026-11-19

## Context

Some actions need no identity at all. Some need very high assurance. A
telephone caller has no passkey.

## Decision

**Assurance is a spectrum**, from anonymous, through pseudonymous and
verified, to the highest assurance a deployment needs, up to military grade.
Each action requires the level it needs and no more.

**Recovery is low-friction until security matters.** Regaining access is easy
at the low end and grows more demanding as the assurance required rises.

**A door without a passkey gathers evidence.** A telephone caller, for
example, is questioned until the evidence meets the standard the action
requires, and is offered only what that standard permits.

**The levels follow an existing framework.** Candidates: the Pan-Canadian
Trust Framework and NIST SP 800-63. To be confirmed.

## Consequences

Identity requirements attach to actions, not to people or doors.

Open for debate: what counts as evidence. The current text holds that evidence
is possession, documents or people who vouch, never a remembered secret.
Whether knowledge-based questions (a mother's maiden name, for example) are
acceptable at some level is to be argued, and is distinct from the rule
against passwords.
