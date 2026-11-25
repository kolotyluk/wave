# 0055. Wave is a mission-critical service

- Status: Accepted
- Date: 2026-11-24

## Context

Telephone engineering long held a principle: when telephone systems fail,
people can die. The same holds for Wave, in specific ways:

- **Exposure.** A leak reveals where a person fleeing abuse is living.
- **Silence.** A missed deadline becomes an eviction, and the eviction
  homelessness.
- **Loss.** Evidence disappears from a case where someone's liberty or safety
  is at stake.

## Decision

***When Wave fails, people can die.*** Wave is a mission-critical service,
and is engineered as one.

**It fails safe, and safe depends on what is at risk.**

- **Confidentiality fails closed.** When the system cannot be sure a
  disclosure is authorized, it refuses. An outage harms; a leak can kill.
- **The record fails preserved.** Faults and overload never lose admitted
  evidence (0050).
- **Availability degrades gracefully.** If the agent or a door is down, a
  person's own record and deadlines stay reachable another way.

**It is not an emergency service.** Wave must never be mistaken for one. The
Front Door agent's charter directs anyone in danger to 911 at once, and every
door says so where it matters (0039).

**Mission-critical engineering applies throughout:** failure-mode analysis
before features ship; no single point of failure in production; tested
recovery; change control for the Admin Wave (0051); incident response with
honest disclosure; and availability, durability and confidentiality targets
stated and measured, never assumed.

## Consequences

The skeleton is not mission-critical, and must not be presented as if it
were. Any deployment serving real people must meet this standard first.

Where confidentiality and availability conflict, confidentiality wins for
protected information; the person is told plainly that something is
temporarily unavailable, never shown something they should not see.
