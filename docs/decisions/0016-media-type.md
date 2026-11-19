# 0016. Wave's media type is application/wave+cbor

- Status: Accepted
- Date: 2026-11-19

## Context

A response should say that it is a Wave object, not just some CBOR.

## Decision

Wave objects are served as `application/wave+cbor`. JSON is offered by content
negotiation (`Accept: application/json`) as a view.

Nothing is ever verified or signed from the JSON view.

## Consequences

The record and its signatures are always CBOR bytes. The JSON view serves
robots, web developers and people reading bug reports.

The media type is to be registered with IANA when the protocol stabilizes.
