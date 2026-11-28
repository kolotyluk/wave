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
  door promptly offers help when available: for suicide risk, Canada's
  9-8-8 crisis line; for imminent danger, 911 (0055). This referral does not
  wait for optional checks or model generation (0062).
- Wave does not monitor people's mental state, infer it from their writing
  or behaviour, or keep watch lists of individuals.
- Patterns across many people are statistics for systemic evidence, never
  targets.

**Serious threats go to a person with lawful authority.** Privacy law
and privilege rules vary by jurisdiction and role; an emergency exception
is not a general permission to share. A robot may request review only within
its existing disclosure authority. Where even referral would cross a bound,
the deployment needs an explicit, legally reviewed authority and protected
route, not an assumption that safety overrides confidentiality.

A designated human reviewer assesses context, credibility, seriousness and
imminence, records the applicable legal basis, and limits any disclosure to
necessary information and authorized recipients. An explicit credible threat
may justify urgent review without waiting for a second signal. Quotation,
historical evidence, hypothetical discussion and a third party's threats
must not automatically be treated as the speaker's intent. Deployment
procedures state reviewer availability and what happens if no reviewer is
reachable; Wave must not imply that someone is monitoring or that help has
been dispatched.

**Interventions are graduated** for anything inferred about people, from
fraud signals to signs of distress:

- **Evidence matters, not signal counts.** Correlated flags or models using
  the same source are not independent corroboration. A model inference alone
  never establishes wrongdoing or authorizes punitive or irreversible action.
- **Authority matches the action.** Advice and referrals are distinct from
  temporary account protection, human escalation and external disclosure.
  Each has a declared purpose, evidence threshold, authority, duration and
  review path. Urgency may skip advisory steps, never authorization.
- **People are told** what the system did and why, with notice delayed only
  under a documented lawful or narrowly scoped security justification and
  reviewed for release. Sensitive reasons stay within their bounds.
- **Override has limits.** A person may dismiss advice and optional assistance.
  They cannot bypass server invariants, mandatory authentication or a valid
  legal restriction. Temporary protective restrictions are proportionate,
  time-limited and have accessible recovery and prompt human review; they
  are not findings about rights (0054).
- **Interventions are bounded and recorded:** they do not repeat without limit,
  and each is in the flight recorder (0059).

Rules that refuse an unauthorized disclosure or an invalid operation are not
interventions of this kind: they do not depend on fallible sensing.

**Behavioural signals protect accounts, never profile people.** Signals such
as keystroke timing might help detect account compromise, but are fallible
behavioural biometric processing, not proof of who is typing. Disability,
injury, stress, assistive technology or a changed device can alter them.
Any such feature requires a separate privacy, accessibility and abuse
assessment before deployment. It is:

- optional, with informed opt-in and an equivalent accessible alternative;
- used only for account protection, never identity merging, fraud findings,
  health inference, mental-state inference or cross-context tracking;
- minimized, preferably processed locally, and subject to short, stated
  retention and deletion of raw signals and templates;
- kept out of the Wave record and general telemetry; any server-side
  processing is isolated within the identity service;
- insufficient alone for permanent lockout or disclosure to another party.
  Step-up and recovery use independently verified methods. Notifications use
  a previously chosen safe channel, not necessarily the compromised device.

**Everything is disclosed and reviewable.** These rules are in the published
rules and the user agreement. Anyone affected by an adverse flag or
intervention has a protected review and appeal route (0054), with any notice
delay justified and periodically reviewed.

## Consequences

Wave informs, nudges, refers and fails safe. It does not take over people's
judgment or lives.

Any government deployment must assess these rules against the Charter and
privacy law; the Wave Act (paper-sync) is where lawful disclosure for
imminent harm would be made explicit.

Before production, counsel must distinguish permitted disclosure from any
duty to report, including the narrow public-safety exception to privilege.
*Smith v. Jones* is relevant, not a blanket emergency override.

Validation includes quoted threats, a credible single-source statement,
correlated false flags, unavailable reviewers, a compromised notification
device, accessibility-related authentication changes and safe recovery.
Measure false positives and disparate effects without building new
cross-context profiles; use consented or synthetic evaluation data.

## References

- *Smith v. Jones*, [1999] 1 SCR 455:
  https://decisions.scc-csc.ca/scc-csc/scc-csc/en/1689/1/document.do
