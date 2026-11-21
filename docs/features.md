# Features

A **capability** is architecture: a mechanism the protocol must carry from the
start. A **feature** is design: a configuration of capabilities that can be
chosen, changed or added without touching the core. Decision records hold
capabilities. This document holds features, each naming the capabilities it
uses.

Capabilities and features are like policies and procedures. A capability is
the broad policy; a feature is a procedure that implements it for a
particular use. One difference matters: a policy is often only a statement of
intent, but a capability is enforced by the architecture. ***Capabilities are
policies the architecture enforces; features are the procedures that apply
them.***

## Capabilities

Every capability Wave relies on, with the decision that records it. Those
marked *not yet recorded* are capabilities the design already depends on that
still need a decision of their own.

| Capability | What it guarantees | Decision |
| --- | --- | --- |
| Shared object | Many participants change one thing at once, and it converges. | [0004](decisions/0004-crdt-convergence.md) |
| Admission | Nothing enters the record without a valid signature and permission. | [0005](decisions/0005-authority-on-admission.md) |
| Replayable history | Every change is kept, attributed, and can be played back. | [0004](decisions/0004-crdt-convergence.md), [0043](decisions/0043-data-strategy.md) |
| Evidentiary record | Integrity can be proved to a hostile expert without trusting the operator. | [0022](decisions/0022-evidentiary-record.md) |
| Acting for | A robot, or a person, acts for a principal, and the record shows both. | [0029](decisions/0029-signed-operation-envelope.md) |
| Identity assurance | From anonymous to verified, as each action needs; no one remembers a password. | [0020](decisions/0020-no-passwords.md), [0021](decisions/0021-identity-assurance-spectrum.md), [0046](decisions/0046-passwords-scope.md) |
| One person, one identity | Aliases and pseudonyms tie back to one person; no one appears as many. | [0047](decisions/0047-one-person-one-identity.md) |
| Verifiable views | Each credential sees what it may, and every view still verifies. | [0032](decisions/0032-verifiable-views.md) |
| Bounds | Visible demarcations with declared rules. | [0036](decisions/0036-bounds.md) |
| Without prejudice | Seen by those inside, never used against them. | [0037](decisions/0037-without-prejudice.md) |
| Agreement | Participants, robots included, make and honour agreements for principals. | [0038](decisions/0038-agreement.md) |
| Robot participation | Robots join as participants, sandboxed, with no other way in. | [0010](decisions/0010-robots-same-interface.md), [0011](decisions/0011-robot-sandbox.md) |
| Robot governance | Enforced constraints, and a published charter recorded with every act. | [0039](decisions/0039-robot-governance.md) |
| Doors | Ways in for every person, replaceable or mandated by deployment. | [0033](decisions/0033-default-doors-and-tabbed-app.md), [0040](decisions/0040-deployments-may-mandate-doors.md) |
| Asking | Questions never change the record. | [0018](decisions/0018-asking-and-changing.md) |
| Portability | A Wave exports whole, verifiable offline, and can leave a provider. | [0022](decisions/0022-evidentiary-record.md), [0043](decisions/0043-data-strategy.md) |
| Erasure | Content can be lawfully destroyed while the record proves it existed. | [0029](decisions/0029-signed-operation-envelope.md), [0043](decisions/0043-data-strategy.md) |
| Federation | Servers share Waves, each Wave with a home whose admission is final. | [0005](decisions/0005-authority-on-admission.md), *open* |
| Time and notice | Dates that matter (deadlines, hearings, expiries) are known to the Wave, and people are told in time, through any door. | *not yet recorded* |
| Escalation | A matter passes to a person (a caseworker, a lawyer, legal aid) with its record, by the principal's choice. | *not yet recorded* |
| Referral directory | Trusted services, sources and authorities can be found and cited. The paper's X.500/LDAP directory layer. | *not yet recorded* |
| Language and accessibility | Both official languages, translation that marks which text is authoritative, and access for every ability. | *not yet recorded* |
| Systemic evidence | Individual journeys contribute, by consent and anonymously, to evidence about systemic problems. | *not yet recorded* |
| Private channel | A person's own robot reports to them outside the Wave, as the paper's sentry does. | *not yet recorded* |
| Moderation | Abuse is handled by a process with notice and appeal. | *not yet recorded* |
| Lawful disclosure | Disclosure under legal authority is scoped, recorded, and never a standing back door. | *not yet recorded* |

## Features

Each feature is a configuration of capabilities.

### The record

**Replay.** "How did we get here?" Play a Wave's history back from the start,
or from any point.
Capabilities: replayable history, verifiable views.

**Evidence export.** A self-proving package of a Wave, or of part of one, for a
court or a reviewer.
Capabilities: evidentiary record, portability, verifiable views.

**In the record.** The sheet a person pulls up to see every signed change their
agent made for them.
Capabilities: acting for, replayable history, doors.

### Views

**Open court.** The public view of a proceeding, with lawful redactions.
Capabilities: verifiable views, identity assurance (anonymous).

**Sealed material.** Exhibits only the court and named parties can see.
Capabilities: bounds, verifiable views.

**Publication ban.** Names withheld from every view outside the bound.
Capabilities: bounds, verifiable views.

### Without prejudice

**Settlement.** Parties negotiating to resolve a dispute. Set by all parties
jointly. Opens to prove that a settlement was reached, and on what terms. The
signed agreement exits into the main record; the negotiation stays inside.
Capabilities: bounds, without prejudice, agreement.

**Robot-proposed without prejudice.** A robot recognises settlement language in
an unmarked wavelet and proposes WP. The proposal and every response are
recorded. If all agree, a bound is drawn around the history already recorded.
Declining does not decide what the law protects.
Capabilities: without prejudice, agreement, bounds over history.

**Mediation.** As settlement, with a mediator who is also bound.

**Apology.** A statement of regret that is not an admission of liability, as
British Columbia's Apology Act provides. Set by one person alone.
Capabilities: without prejudice.

**Exploratory discussion.** A board or committee thinking out loud, without
being held to every idea.
Capabilities: without prejudice, bounds.

**Candid review.** A "what went wrong" discussion after an incident, on the
model of aviation safety investigation: what is said is used to learn, not to
blame.
Capabilities: without prejudice, bounds.

**Asking without admitting.** A person asks their agent "what if I refused?"
without that question becoming evidence of intent.
Capabilities: without prejudice, applied to the person's private wavelet.

### Identity

**Telephone verification.** A caller with no passkey is questioned until the
evidence meets the standard the action requires.
Capabilities: identity assurance, doors.

**Step-up.** A sensitive action asks for more assurance than the person has
shown so far, and only then.
Capabilities: identity assurance.

**User agreement.** Each person agrees to act only as themselves, or as a
declared delegate (for example under a power of attorney) through recorded
delegation. Acceptance is signed and recorded;
violations lead to graduated consequences with notice and appeal.
Capabilities: agreement, one person one identity, moderation.

**Recovery.** Regaining access: easy at the low end, demanding where security
matters.
Capabilities: identity assurance.

### Doors

**Split view.** Ask through the Front Door and watch it take effect in the
record.
Capabilities: doors, replayable history.

**Mandated Front Door.** In the Housing Advocate deployment, the government's
door and no other; the person can still see their own record.
Capabilities: doors (mandated), verifiable views.

**Bring your own door.** Where a deployment allows, any conforming shell.
Capabilities: doors.

### Agents

**Deadline flagging.** The agent marks a deadline in a letter and explains it.
Capabilities: acting for, robot governance, time and notice.

**Draft, don't send.** The agent drafts in the person's private wavelet; nothing
leaves until they choose.
Capabilities: acting for, verifiable views.

**Sentry.** A personal robot that watches on its principal's behalf and warns
them privately.
Capabilities: robot participation, private channel, robot governance.

### Help

**Deadline reminders.** "23 days until 2026-12-12", by app, text message or
telephone.
Capabilities: time and notice, doors.

**Hand off to a person.** The agent recognises a matter needs a professional and,
with the person's consent, brings one into the Wave with the record.
Capabilities: escalation, referral directory, agreement.

**Find free help.** Vetted services near the person, with the source of each.
Capabilities: referral directory.

**Contribute to systemic evidence.** By consent, a person's case counts toward
evidence about systemic problems, without exposing them.
Capabilities: systemic evidence, verifiable views.
