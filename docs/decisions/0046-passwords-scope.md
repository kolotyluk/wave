# 0046. No one has to remember a password; infrastructure avoids them too

- Status: Accepted
- Date: 2026-11-21
- Amends: [0020](0020-no-passwords.md)

## Context

0020 said "no passwords, ever" without saying whether it covered
infrastructure. Oz's review noted that the development database login uses a
generated password.

## Decision

**People never have to remember a password.** No participant, person or
robot, authenticates to Wave with a password, a PIN, or any other remembered
secret standing in for one. There are many other ways to identify and
authenticate.

**Infrastructure in development may use passwords**, generated and kept
outside the repository.

**Infrastructure in production avoids them**, preferring certificate
authentication or workload identity (PostgreSQL supports certificate
authentication; clouds offer workload identity).

## Consequences

The development database login is acceptable as it is.

Moving to production includes replacing infrastructure passwords.
