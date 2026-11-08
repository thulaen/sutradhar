# ADR-0011: Fresh start

Decided by the owner on 8 November 2026.

## Decision

- This repository starts with new history.
- Earlier work is kept in a private, read-only archive on Dell, outside
  this repository, until the condition in
  [AGENTS.md section 12](../../AGENTS.md#12-docs) is met.
- Every earlier idea, method, design and plan is rewritten for this
  repository's languages, as Phase 1 of the
  [roadmap](../roadmap.md#phase-1-forgejo-home) sets out.
- Earlier code is brought over only after it passes every check here.

## Why

A clean start with one set of languages lets every check reach 100% and
keeps the docs consistent.

## How we check it

- A scan of every file and all history refuses any language or file type
  not on the allowed list.
- The carry-over list in Forgejo tracks every earlier item until it is
  rewritten here.
