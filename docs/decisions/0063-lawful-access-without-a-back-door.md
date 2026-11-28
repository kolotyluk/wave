# 0063. Lawful access without a back door

- Status: Proposed
- Date: 2026-11-27

## Context

Communication systems are expected to give lawful access under court
authority. Eric worked on lawful intercept in the first commercial GPRS
product at Motorola, where it was designed in and required a court order.

Canada is legislating. Bill C-22, the Lawful Access Act, 2026, passed the
House of Commons on 2026-06-18 and was in Senate committee as of
2026-10-22. Its Part 2, the Supporting Authorized Access to Information
Act, would require designated "core providers" to build the capability to
answer lawful access requests, lets the Minister extend those obligations
to any electronic service provider, allows metadata retention for up to a
year, and prohibits requiring a "systemic vulnerability", a definition
critics consider too weak. A federal Canada Wave could be designated or
ordered.

Wave promises no back door (0010, 0048). Lawful access designed in, under
authority, is better than access bolted on later under an order.

## Decision

Proposed:

**Production from the record is the normal path.** Most lawful access is to
stored information, and Wave's record is already evidence (0022). Bounds and
verifiable views (0032, 0036) disclose exactly what an order covers, and
prove that nothing else was sent.

**Live access is a declared participant.** If an order requires new
operations as they occur, access is given by an ordered, time-bounded
participant, collapsed by default and discoverable to authorized reviewers
like an intrinsic robot (0048). Never a hidden copy, a master key, or a
route around the interface.

**Authority is checked and recorded.** A person with lawful authority, not a
robot, executes each order. The order's reference, scope and time window
are recorded, and every access is in the flight recorder (0059). The
operator cannot exceed an order unseen, nor deny one.

**Notice is delayed, not denied.** Where the law forbids telling the person,
Wave keeps a sealed entry that becomes visible to them when the
prohibition lifts. Aggregate counts are published where the law allows.

**Privilege is protected.** Orders touching solicitor-client communications
go through a privilege review before anything is produced, as the Supreme
Court required for law offices in *Lavallee* (2002 SCC 61).

**Wave states what it holds in the clear.** Content sealed or protected end
to end cannot be produced as plaintext because Wave does not have it.
Publishing this makes Wave's obligations knowable.

**Retention is deliberate.** Admission receipts are metadata (0043); any
retention period is a stated policy.

**Standard interfaces at the boundary.** If a deployment must provide
handover to authorities, it uses the established lawful-intercept handover
standards rather than inventing one (0014).

## Consequences

Wave can be a worked example of lawful access without a systemic
vulnerability, which belongs in the paper and in submissions on C-22.

Which handover standards apply, and how sealed-entry notice interacts with
secrecy orders, need legal review before production.

## References

- Bill C-22, LEGISinfo: https://www.parl.ca/LegisInfo/en/bill/45-1/C-22
- Public Safety backgrounder on Part 2:
  https://www.canada.ca/en/public-safety-canada/news/2026/03/backgrounder--securing-access-to-information-in-bill-c-22.html
