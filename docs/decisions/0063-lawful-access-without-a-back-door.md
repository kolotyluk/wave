# 0063. Lawful access without a back door

- Status: Proposed
- Date: 2026-11-27

## Context

Communication systems are expected to give lawful access under court
authority. Eric worked on lawful intercept in the first commercial GPRS
product at Motorola, where it was designed in and required a court order.

Canada is legislating. Bill C-22, the Lawful Access Act, 2026, passed the
House of Commons on 2026-06-18 and reached Senate report stage after a
committee report with amendments on 2026-11-26, according to LEGISinfo
(checked 2026-11-28). It is a bill, not an enacted authority. Its Part 2,
the Supporting Authorized Access to Information Act, would require
designated "core providers" to build lawful-access capabilities and allow
ministerial capability orders, subject to Intelligence Commissioner approval,
for other electronic service providers. The March backgrounder describes
regulations prescribing metadata retention of no more than a year, not
content retention; capability obligations are distinct from authorization
to obtain particular information. A federal Canada Wave could be designated or
ordered. These proposals can change: production obligations require counsel
to check the applicable enacted text, commencement, regulations and orders,
not rely on this summary or the March backgrounder.

Wave promises no back door (0010, 0048). Lawful access designed in, under
authority, is better than access bolted on later under an order.

## Decision

Proposed:

**Production from the record is the normal path.** Most lawful access is to
stored information, and Wave's record is already evidence (0022). Bounds and
verifiable views (0032, 0036) constrain the authorized production path.
A signed manifest identifies the order, source checkpoint, selection,
redactions and delivered artifacts, within protected audit bounds.
Verification establishes integrity against that committed source and the
manifest's declared scope; it does not prove the operator never made another
copy or that a legal interpretation of scope is correct.

**An order is not a blanket capability.** A designated, legally authorized
human validates issuer, jurisdiction, authenticity, target, scope, dates
and any privilege or secrecy conditions. Execution uses a narrowly scoped
grant; extensions need fresh authority. Ambiguous or conflicting demands
have a route to counsel and the issuing authority, including clarification,
challenge or a stay where available. Neither a robot nor an ordinary server
administrator may decide that a demand overrides existing bounds.

**Bounds do not disappear under an order.** This proposal creates no generic
power to override required agreements (0036, 0038, 0051). If a deployment
needs a court-authorized exception to an existing bound, that exception
needs a separately specified, legally reviewed rule before use: who can
authorize it, which protection can change, and how its grant and execution
are recorded and reviewed. Until then, do not bypass the bound; refer the
conflict for legal resolution. Legal authority does not supply missing keys.

**Live access is a declared participant.** If an order requires new
operations as they occur, access is given by an ordered, time-bounded
read-only participant under a validated grant, collapsed by default and
discoverable to authorized reviewers like an intrinsic robot (0048). Its
capability names the recipient, compartments, operation kinds, start and end
conditions, and allowed uses; it grants no administration, delegation or
onward subscription. Revalidate scope, revocation and expiry at every
delivery, including queued deliveries, and stop on expiry or suspension.
Privilege filtering applies to new operations before delivery, not just to
the initial export. Using the ordinary interface is necessary, not proof
that the grant itself is legitimate. Never a master key or unrecorded route
around authorization.

**Authority is checked and recorded.** A person with lawful authority, not a
robot, authorizes execution; mechanical delivery may then be automated
within that grant. The order's reference, scope, time window and changes are
protected audit records, with durable intent and delivery outcomes (0059).
Two-person approval and separation of authorization, production and audit
roles apply to high-risk grants. Logs support detection and review of
misuse; they do not prove a compromised host cannot exfiltrate data outside
Wave. Host controls, independent witnessing and incident response remain
necessary. Technical inability and lawful challenges are documented, not
treated as evasion or silently bypassed.

**Notice is delayed, not denied.** Where the law forbids telling the person,
Wave keeps a sealed entry that becomes visible to them when the
prohibition lifts and disclosure is legally permitted. The entry and related
participant metadata must not reveal the order to unauthorized viewers.
Track review dates, extensions and release authority; do not assume every
prohibition has a known expiry. Notice delays are periodically reviewed,
and counts are published only where lawful and safe against re-identification.

**Privilege is protected.** Orders touching solicitor-client communications
have potentially privileged responsive material segregated before
investigative recipients can inspect it. Privilege belongs to the client
and is not waived by storage in Wave. The process
provides a meaningful opportunity to assert privilege, notice where lawful,
independent review where required and judicial determination of contested
claims. Operator approval alone is not privilege adjudication. *Lavallee*
(2002 SCC 61) supplies important law-office-search safeguards; applying them
to hosted records and live delivery needs counsel, not an assumed universal
procedure.

**Wave states what it can decrypt.** Content protected end to end, whose keys
Wave does not hold or control, cannot be produced by Wave as plaintext.
A sealed compartment or encryption at rest does not by itself establish
that limitation; document server, host, model-provider and backup access.
Publishing this makes the technical limits knowable; it does not itself
determine legal obligations. No escrow, silent key substitution, compelled
client change or addition of a decrypting recipient is authorized by this
proposal. Any such change would require a separate decision and threat/legal
review, not an implementation detail of the participant interface.

**Retention is deliberate.** Admission receipts are metadata (0043); any
retention period is a stated, legally reviewed policy for its data class.
Preservation holds and any mandated retention are scoped and audited, with
release and deletion rules; a proposed statutory maximum is not a default
retention period for all Wave metadata.

**Standard interfaces at the boundary.** If a deployment must provide
handover to authorities, it uses the established lawful-intercept handover
standards where applicable rather than inventing one (0014). Such adapters
inherit Wave's grants, bounds, audit and minimization; a handover standard
does not itself grant authority or justify a standing bulk feed.

## Consequences

Wave can propose a worked example of constrained, auditable lawful access.
Whether it avoids systemic vulnerabilities needs independent security and
legal review; it is not established by naming the access a participant.

Which handover standards apply, and how sealed-entry notice interacts with
secrecy orders, need legal review before production.

Validation must reject out-of-scope, expired and revoked grants, including
queued deliveries; hold potentially privileged live content for review;
preserve secrecy in participant counts and audit views; and fail closed on
audit failure. Test that an ordinary administrator cannot override a bound
or add a decrypting recipient through this path.

## References

- Bill C-22, LEGISinfo: https://www.parl.ca/LegisInfo/en/bill/45-1/C-22
- Public Safety backgrounder on Part 2:
  https://www.canada.ca/en/public-safety-canada/news/2026/03/backgrounder--securing-access-to-information-in-bill-c-22.html
- *Lavallee, Rackel & Heintz v. Canada (Attorney General)*, 2002 SCC 61:
  https://scc-csc.lexum.com/scc-csc/scc-csc/en/2002/1/document.do
