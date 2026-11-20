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
