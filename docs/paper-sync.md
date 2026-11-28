# Carrying decisions back to the paper

The repository is the source of truth; the discussion paper is brought into
line with it. This list holds what the paper (v0.4, 2026-11-17) still needs.
Remove an item once the paper carries it.

- **Convergence.** Appendix G proposes building on OT first and testing a CRDT
  later. The repository chose CRDTs, with the server as the authority on
  admission, not order ([0004](decisions/0004-crdt-convergence.md),
  [0005](decisions/0005-authority-on-admission.md)). The admission sentence
  answers the paper's own objection that CRDTs "locate accountability nowhere".
- **Apache Wave as runtime.** Appendix G and section 2 say retained Apache Wave
  components may run in the skeleton. The repository uses Apache Wave as
  lineage and reference only ([0013](decisions/0013-apache-wave-lineage.md)).
- **Why Wave failed, and why now.** Section 2's "Why Wave failed then" can gain
  the Front Door framing: the interface showed the machinery; the Front Door
  hides it for convenience, never for secrecy; the Side Door keeps it one tab
  away; an AI agent is the translator that did not exist in 2009.
- **What Wave is for.** The transport metaphor: Canada Wave is the road
  network, a Wave is the vehicle, and a Wave keeps the record of the journey,
  like a flight recorder. Public line: "Collaboration has always been a
  journey. Canada Wave keeps the record of it." Plus the four-part test for
  when a journey suits a Wave. For the abstract and section 2. See also
  [messaging](messaging.md).
- **Bounds, without prejudice and agreement**
  ([0036](decisions/0036-bounds.md), [0037](decisions/0037-without-prejudice.md),
  [0038](decisions/0038-agreement.md)): new since v0.4. The X.500 lineage of
  bounds belongs beside Appendix B's directory layer.
- **Robot governance and mandated doors**
  ([0039](decisions/0039-robot-governance.md),
  [0040](decisions/0040-deployments-may-mandate-doors.md)): belongs with
  section 7's "Canadian rules for public agents".
- **An ask for section 13 (immediate program asks).** Networks the government
  runs or funds (departments, libraries, schools, community Wi-Fi) should
  allow HTTP/3 and IPv6, and government services should offer them. Small,
  cheap, and good for every service, not only Wave. Fits the sovereign
  broadband theme. ([0041](decisions/0041-http3-ground-floor.md),
  [0042](decisions/0042-ipv6-first.md))
- **Documentation** ([0044](decisions/0044-documentation.md)): four books
  (Wave Rider, Wave Shape, Wave Meister, Wave Reference) and a Principles of
  Operation as the written specification. Strengthens section 9's standards
  path: a public, normative spec with conformance tests.
- **Identity and the user agreement**
  ([0047](decisions/0047-one-person-one-identity.md)): one person, one
  identity; a user agreement to act only as oneself or through recorded
  delegation; graduated enforcement with appeal. Fines need statutory
  authority, which fits section 8's integrity offences and graduated
  liability.
- **A Canada Wave Act**, to be drafted later and offered with the paper,
  perhaps as the paper's "possible Digital Village Act" (section 6) made
  concrete. Provisions collected so far:
  - confidentiality of sentry communications, opened only on court order
    (section 7);
  - integrity offences for the gaps the Criminal Code leaves: fictitious or
    multiple identities (s. 403 covers personating a real person, not
    inventing one), undisclosed representation, robots passed off as people,
    and manipulation of counted civic processes such as consultations, where
    no one is deprived of property and fraud (s. 380) may not apply
    ([0047](decisions/0047-one-person-one-identity.md));
  - enforcement in three tiers: the user agreement and membership rules for
    warnings, restrictions and loss of access; administrative monetary
    penalties under the Act, with appeal; and the Criminal Code, unchanged,
    for impersonation, forgery, tampering, fraud and perjury;
  - an offence for knowingly false attested statements made in Wave, for
    statements not sworn before an authorized official (see "Sworn statement"
    and "Attested statement" in [features](features.md));
  - recognition of robot agreements made for a principal
    ([0038](decisions/0038-agreement.md)), and of without-prejudice protection
    for what an opposing party's agent sees
    ([0037](decisions/0037-without-prejudice.md));
  - evidentiary standing for Wave records: a presumption of integrity under
    the Canada Evidence Act's provisions for electronic records
    ([0022](decisions/0022-evidentiary-record.md));
  - scoped lawful disclosure with no standing back door;
  - HTTP/3 and IPv6 on networks the government runs or funds.
- **British Columbia's Digital Trust program as a partner**
  ([0052](decisions/0052-authentication-keys.md)). BC's Person credential
  (now in the BC Services Card app) is independently verifiable and
  revocable. In BC's justice project, lawyers combine it with a Law Society
  credential to access court materials online, and it supports legally
  binding signatures on web forms. A concrete precedent for Wave's verified
  assurance, the Wave Court and attested statements; belongs in Appendix H
  under Citizens' Services, and in section 7's identity discussion.
- **AI interfaces** ([0056](decisions/0056-ai-ready-derived-views.md),
  [0057](decisions/0057-mcp-and-a2a.md), [0058](decisions/0058-wave-dom.md)):
  Appendix B already names A2A and MCP; the repository proposes host adapters
  that preserve actual Wave signing, delegation and disclosure. The protocols
  do not automatically supply those semantics. Appendix G gains a derived
  semantic Wave DOM, versioned citation selectors and authorization-scoped
  metadata, not an assumption of access to hidden model reasoning. Shared
  semantics help accessibility but do not replace accessible UI testing.
  Keep cache reuse conditional and model-specific. See the sourced
  [comparison](google-wave-and-canada-wave.md) for corrected history and
  protocol dates, including A2A v1.0 on March 12, 2026.
- **Safety without surveillance**
  ([0059](decisions/0059-flight-recorder.md),
  [0060](decisions/0060-bounded-labelled-checks.md),
  [0061](decisions/0061-safety-response-and-graduated-intervention.md)).
  Belongs in the section on Charter section 7 and prevention of serious
  harm: respond to explicit statements, never profile; graduated, disclosed,
  overridable intervention (the driver-assistance model, not MCAS); a
  flight recorder for learning from incidents, TSB-style; checks that say
  when they are partial. A Wave Act could make lawful disclosure for
  imminent serious harm explicit.
- **Bill C-22, the Lawful Access Act, 2026**
  ([0063](decisions/0063-lawful-access-without-a-back-door.md)). The paper
  should engage with it: Wave as a worked example of lawful access without
  a systemic vulnerability. Production scoped by verifiable views, live
  access only by a declared participant, every access in the flight
  recorder, delayed notice, privilege review. Also a basis for a
  submission on the bill.
- **Courtroom horizon.** The repository records the long-term goal of
  conducting a court proceeding as a Wave
  ([0031](decisions/0031-courtroom-horizon.md)). Wording for the paper is
  still to be chosen.
- **Evidentiary record, no passwords, verifiable views, the Front Door agent as
  an ordinary robot** ([0020](decisions/0020-no-passwords.md),
  [0022](decisions/0022-evidentiary-record.md),
  [0032](decisions/0032-verifiable-views.md),
  [0035](decisions/0035-front-door-agent-is-a-robot.md)): new since v0.4.
- **Editorial.** Appendix B still says "Canada Commons"; its numbered list
  starts at 71; reference [85] points to svn.apache.org, which may be better
  given as the GitHub archive at tag `wave-0.4-rc1`.
