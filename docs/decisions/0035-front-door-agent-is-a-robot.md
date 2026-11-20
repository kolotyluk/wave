# 0035. The Front Door agent is an ordinary robot

- Status: Proposed
- Date: 2026-11-19

## Context

0033 made the Wave-supplied AI agent the default Front Door. 0010 says robots
join through the same interface as people. Neither says whether the agent
behind the Front Door is a robot like any other or something more.

A robot is a participant in a Wave; a door is how a person reaches one.

## Decision

Proposed: the Front Door and its agent are two things.

**The Front Door is the conversation**: text and voice now, telephone and text
message later. It carries what the person means and what Wave answers (0009).

**The agent is an ordinary robot.** It joins the Wave as a participant, acts
for the person under delegation, signs every change as a robot acting for that
person (0029), and runs in the robot sandbox with only the capabilities the
Wave grants (0011). It has no privilege that another robot could not have.

**The default agent is special only in being the default.** A person may
choose another agent, for example one maintained and vouched for by a
legal-aid organization, with exactly the same standing.

**The model behind the agent is outside Wave.** Access to a language model is a
capability granted to the robot, not part of the record or the server. Which
model, and where it runs, is a deployment choice.

## Consequences

"No back door" holds for the agent too: it enters as every robot does.

Replacing the agent loses nothing, because the default had nothing extra.

Where the model runs, and who operates it, is a sovereignty question like the
choice of cloud (0030).

Open: where the conversation itself lives. The discussion paper promises the
person a complete transcript, which points to their private wavelet, in the
record and replayable. But a private wavelet sits on the host, under its
retention and open to discovery; the paper's sentry uses a private channel off
the Wave for that reason. Default proposed here: the conversation lives in the
person's private wavelet, and the off-Wave channel is left to the sentry.
