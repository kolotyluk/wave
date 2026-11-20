# 0040. A deployment may mandate its doors

- Status: Proposed
- Date: 2026-11-20
- Amends: [0033](0033-default-doors-and-tabbed-app.md)

## Context

0033 says either default door may be replaced by any shell that conforms.
That is true of the architecture. It is not true of every deployment.

For the Automated Housing Advocate, the federal government provides the Front
Door, and in that deployment it cannot be substituted: the government is
accountable for the advice given, under its rules for automated decisions,
official languages and accessibility.

## Decision

Proposed: **replaceability is a property of the architecture; substitution is
a policy of each deployment.**

- Any conforming door can exist, and anyone may run their own deployment with
  their own doors. Another agency may offer its own housing advocate behind
  its own Front Door.
- A deployment may mandate a door, so that within it no other door may be
  used.
- A mandated door must still conform to the protocol and the design
  principles. Being mandated grants it no route into the record that another
  door would lack.
- **The floor: a person can always see their own record.** Where a Front Door
  is mandated, the person keeps a way to inspect what is recorded about them,
  through the Side Door or an export. A mandated door is never the only window.
- A deployment declares its mandates openly, as part of its published rules.

## Consequences

Open: whether a person may bring their own robots, such as a sentry, into a
deployment whose Front Door is mandated. A robot is a participant, not a door,
so 0040 does not decide it; the deployment's rules do, and they should say so.
