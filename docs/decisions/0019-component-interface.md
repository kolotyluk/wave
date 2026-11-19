# 0019. The robot and door boundary is described in WIT

- Status: Proposed
- Date: 2026-11-16

## Context

Robots and doors are WebAssembly components that others may write in other
languages.

## Decision

Proposed: the boundary between Wave and its robots and doors is described in
WIT, the WebAssembly Component Model's interface language.

## Consequences

Anyone can implement a robot or a door in any language that targets the
Component Model.

The tooling is to be verified before anything depends on it.
