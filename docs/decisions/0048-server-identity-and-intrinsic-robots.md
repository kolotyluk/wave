# 0048. Servers have identities; intrinsic robots are not shown by default, never hidden

- Status: Proposed
- Date: 2026-11-21

## Context

Some robots act for a deployment rather than an individual person: robots
that coordinate sealing, deliver to federated servers, or watch for fraud or
abuse. They need a principal and a record like any other robot (0035, 0039).
An operator is not itself a robot, and operating the default Front Door
agent does not replace the person as that agent's represented principal.

Some should not clutter what people see, like hidden files in a directory
listing. But a robot no one can see would be a back door (0010).

## Decision

Proposed:

**Each server, or each deployment, has its own identity**, as an institution
does (0047). Robots that act for no person act for it, and sign as robots for
it, for example "robot, for `wave1.ztuph.com`". The default Front Door agent
acts for the person and runs under its deployment's charter.
The identity must identify its accountable operator and its relationship to
the operating institution. A hostname is a label, not proof of identity or
a grant of authority. Stable identifiers, server/deployment scope, operator
delegation, and key rotation across failover remain to be specified.

**Intrinsic robots** are robots a deployment places in its Waves on its own
behalf. They are named like hidden files, with a leading dot: `.fraud`,
`.moderator`, `.archivist` (sealing), `.timekeeper` (deadlines and notice),
`.courier` (federation delivery).

A leading dot is a display convention, never a grant of capabilities or
proof that a robot belongs to the operator. Admission checks the robot's
identity, principal and delegated permissions like any other participant.

Backend workers are not automatically participant robots. A host's storage
maintenance and delivery machinery remains behind the host interface;
robot acts that change a Wave use signed operations through the ordinary
participant interface. The maintenance/participant boundary needs explicit
specification, not a privileged robot bypass.

**Not shown by default, never hidden.**

- They are recorded as participants in the scopes in which they act. To a
  viewer entitled to see that participation, they are collapsed by default
  ("System robots (3)"); the Side Door exposes the full authorized list.
  No setting hides a robot from an otherwise entitled viewer.
- The deployment's robot roles and purposes are disclosed in its published
  rules and user agreement. Disclosure of specific participation and counts
  respects authorized views and protected bounds; it must not reveal a
  sealed matter or its participants to an outsider.
- They follow every rule other robots follow: the sandbox (0011), a published
  charter recorded with every act (0039), signed operations, and views limited
  to what they may see (0032).

**They respect bounds.** Intrinsic status never grants permission to carry
content or derived flags out of a without-prejudice or other bound (0037).
A suspicion of fraud is not itself disclosure authority. Any permitted
outflow requires the applicable agreement or a scoped decision by a person
with authority granted by law, under the bound's rules; the authority and
exercise are recorded. Merely being a deployment administrator is not
enough.

**They flag; people decide.** A robot such as `.fraud` never decides anything
that affects a person's rights. Its flags go to an authorized person
through moderation with notice and appeal, and are never treated as
findings. Keep them within the source bound or another restricted
moderation scope only when that transfer is authorized. Being a moderator
does not automatically grant access to the source evidence.

**Witnesses are never intrinsic.** A witness must be independent of the
operator (0022), so it cannot be the operator's own robot.

## Consequences

Every act in a Wave has a principal, including the system's own.

Monitoring robots belong in each deployment's privacy impact assessment.
They remain subject to resource limits and attack-resistance requirements
([0050](0050-attack-resistance.md)).

The Side Door needs a "show system robots" view; the Front Door can mention
them in plain words when asked ("Who else is here?").
