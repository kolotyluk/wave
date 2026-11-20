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

## Settlement

Parties negotiating to resolve a dispute.

- Capabilities: bounds ([0036](decisions/0036-bounds.md)), without prejudice
  ([0037](decisions/0037-without-prejudice.md)), agreement
  ([0038](decisions/0038-agreement.md)).
- Set by all parties jointly.
- Opens to prove that a settlement was reached, and on what terms.
- The signed agreement exits into the main record as the outcome; the
  negotiation stays inside.

## Robot-proposed without prejudice

A robot recognises settlement language in an unmarked wavelet and proposes WP.

- Capabilities: without prejudice, agreement, bounds over history.
- The proposal and every response are recorded.
- If all agree, a WP bound may be drawn around the history already recorded.
- Declining is recorded; it does not decide what the law protects.

## Mediation

As settlement, with a mediator who is also bound.

## Apology

A statement of regret that is not an admission of liability, as British
Columbia's Apology Act provides.

- Capabilities: without prejudice, set by one person alone.

## Exploratory discussion

A board or committee thinking out loud, without being held to every idea.

- Capabilities: without prejudice, bounds.

## Candid review

A "what went wrong" discussion after an incident, on the model of aviation
safety investigation: what is said is used to learn, not to blame.

- Capabilities: without prejudice, bounds.

## Asking without admitting

A person asks their agent "what if I refused?" without that question becoming
evidence of intent.

- Capabilities: without prejudice, applied to the person's private wavelet with
  their agent.
