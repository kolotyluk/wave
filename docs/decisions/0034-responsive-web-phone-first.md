# 0034. One responsive web app, phone-first for the Front Door

- Status: Proposed
- Date: 2026-11-19

## Context

The first users, people working through a housing problem, are among the most
likely to reach the internet only by phone. Native phone apps usually feel
better, but they mean more codebases, and distribution that Apple and Google
control.

## Decision

Proposed: the default app is one responsive web app for every screen size.

**The Front Door is designed for the phone first.** Its usability on a phone
is the highest priority.

**The Side Door adapts but comes second.** On a phone it shows one panel at a
time (structure, document or operation), read-only, with "View as" kept.

**Gestures.** Left and right move between tabs, as a shortcut to a visible
tab bar, never the only way, and never from the screen's edge. Up and down
are time: up is earlier, in the conversation and in the history alike. On the
Front Door, swiping up from a handle opens "In the record", the signed
operations behind the conversation. No pull-to-refresh, no action that
appears only on a long press.

**System navigation is respected.** Android's Back (button or edge gesture)
closes the topmost sheet or panel before it leaves Wave: opening a sheet or an
inner panel adds a browser history entry. iPhones have no system Back, so
every sheet and inner panel also has a visible Close or Back control. Nothing
in Wave uses the screen's edges, which both systems reserve, and the tab bar
sits above the system's gesture area (safe-area insets).

**Native apps are optional shells** under 0033, built later by Wave or by any
agency that wants one.

## Consequences

One codebase, no install, no gatekeeper, reachable from a link in a text
message.

Notifications are the weak spot: on iOS, web push works only once the web app
is installed to the home screen. The telephone and text-message doors cover
people without it, and people without a smartphone at all.

Testing on real phones (iOS Safari, Android Chrome, Samsung Internet) joins the three-engine
testing in 0027.
