# 0061. Safety response is graduated, explicit, and never profiling

- Status: Proposed
- Date: 2026-11-27

## Context

People who come to Wave are often in difficulty. Some may be at risk of
harming themselves or others. The February 2026 tragedy in Tumbler Ridge,
British Columbia, shows what is at stake. Government surveillance is also a
real and reasonable fear.

A car's driver assistance gets the balance right: it flashes and beeps, then
nudges the steering, and brakes only when danger is imminent; the driver can
override. The Boeing 737 MAX's MCAS got it wrong: it acted on a single
sensor, intervened forcefully and repeatedly, was not disclosed to pilots,
and was hard to overcome. Two aircraft were lost.

Predicting violence is unreliable: it is rare, so most people any detector
flags would be wrongly suspected. People with mental illness are far more
often victims of violence than perpetrators. If people fear being flagged,
those most in need stop being candid, or stop coming.

Safety is important, but there is a limit to how far a system can go.

## Decision

Proposed:

**Respond to what is said, never to a profile.**

- If someone writes that they intend to harm themselves or someone else, the
  agent responds at once: for suicide risk, Canada's 9-8-8 crisis line; for
  imminent danger, 911 (0055).
- Wave does not monitor people's mental state, infer it from their writing
  or behaviour, or keep watch lists of individuals.
- Patterns across many people are statistics for systemic evidence, never
  targets.

**Serious threats go to a person with lawful authority.** Privacy law
generally permits disclosure to prevent imminent serious harm. A robot may
raise the alarm; an authorized person decides whether to disclose, under the
deployment's rules, and the decision is recorded. The discussion paper's
section on Charter section 7 and prevention of serious harm applies.

**Interventions are graduated** for anything inferred about people, from
fraud signals to signs of distress:

- **No single signal triggers strong action.** One flag, one pattern or one
  model's judgment may inform or nudge, never act.
- **Strength matches confidence and imminence:** inform, then nudge, then
  act.
- **People are told** what the system did and why. Nothing like an
  undisclosed MCAS.
- **People can override**, as `.admin` defers to explicit commands (0051).
- **Interventions are bounded and recorded:** they do not repeat without limit,
  and each is in the flight recorder (0059).

Rules that refuse an unauthorized disclosure or an invalid operation are not
interventions of this kind: they do not depend on fallible sensing.

**Behavioural signals protect accounts, never profile people.** Signals such
as keystroke timing can detect someone other than the account holder at the
keyboard, protecting, for example, a person whose abuser has their device.
They are:

- disclosed in the user agreement;
- used only to protect the account;
- kept in the identity service, never in the record;
- never used to infer health, mental state or anything else about the person.

**Everything is disclosed and reviewable.** These rules are in the published
rules and the user agreement, and anyone affected by a flag or intervention
has notice and appeal (0054).

## Consequences

Wave informs, nudges, refers and fails safe. It does not take over people's
judgment or lives.

Any government deployment must assess these rules against the Charter and
privacy law; the Wave Act (paper-sync) is where lawful disclosure for
imminent harm would be made explicit.
