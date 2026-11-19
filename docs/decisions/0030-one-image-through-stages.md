# 0030. One container image moves through every stage

- Status: Accepted
- Date: 2026-11-16

## Context

What is tested should be what ships.

## Decision

The same container image moves through every stage unchanged:

- **Alpha**: Docker on a developer's machine. Every change gets here.
- **Beta**: a TrueNAS server running containers and virtual machines. One server
  in a container and a second in a virtual machine, so federation is tested
  between two independent machines.
- **Production**: a commercial cloud, chosen later. Not part of the skeleton.

Unit tests and test vectors run natively on every build, before any stage.

Wave depends on nothing a single provider owns: a container, a network and
storage are all it asks for.

## Consequences

The choice of cloud stays open. Where a provider is owned decides whose law
can reach the data, wherever the machines sit; that is a policy question for
whoever deploys Wave.
