# 0039. Robots are governed by enforced constraints and a published charter

- Status: Proposed
- Date: 2026-11-20

## Context

AI robots take part in Waves as participants (0010, 0035). Each AI product has
its own way of telling a model how to behave: CLAUDE.md for Claude, AGENTS.md
and similar files for others. These instructions are advisory. A model is
asked to follow them; nothing makes it.

Public agents in Canada are also subject to rules of their own, such as the
Treasury Board's Directive on Automated Decision-Making.

## Decision

Proposed: robot governance has two layers.

**Enforced constraints.** What the architecture guarantees whatever the model
does: the capabilities its sandbox grants (0011), signing and delegation
(0029), flow restrictions (0037), permissions and admission (0005). Anything
that must hold is enforced here, never left to instructions.

**A published charter.** Every robot publishes a charter: its owner, its
purpose, the capabilities it requests, and its rules of conduct. The charter
is human-readable, signed and versioned. Each product translates it into its
own mechanism; Wave depends on none of them.

**The charter is part of the record.** Every operation a robot signs carries
the hash of the charter it was operating under. The record shows not only what
a robot did but what it had been told to do.

**Deployments add rules.** A deployment may require rules of every robot in it
(see [robot governance](../robot-governance.md) for required and recommended
examples).

## Consequences

A court, an auditor or a participant can read the rules any robot acted under,
at the moment it acted.

The envelope (0029) gains a charter reference.

Instructions remain fallible. The design treats them as conduct to be shown,
not guarantees to be relied on.
