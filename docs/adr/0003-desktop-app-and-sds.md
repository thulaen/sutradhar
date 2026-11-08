# ADR-0003: The desktop app and the Sutradhar Design System

Decided by the owner on 8 November 2026.

## Decision

- The owner's only screen is a Dioxus desktop app on the owner's Ubuntu
  PC, as set out in
  [AGENTS.md section 5.3](../../AGENTS.md#53-screens-and-the-service).
- A separate background service under systemd does all the work. It
  keeps running when the window is closed.
- The window drives the service over a local socket.
- Screens use the Sutradhar Design System (SDS), our own component
  library in the spirit of GOV.UK Design System and BBC GEL, with no
  copied branding.
- Delivered as a signed `.deb`.

## Why

- Running the app on the owner's PC keeps heavy work off the site's
  server, so the add-ons work on cheap shared hosting.
- With Dioxus, screens are written in Rust and share types with the
  service.
- GOV.UK and BBC GEL are tested, accessible patterns for forms, errors
  and navigation that people who are not programmers can follow.

## Trade-offs

- The Dioxus desktop renderer uses the WebKitGTK system library. It is
  listed as a system library, not our code, and counts towards the
  memory limit.
- Only one Ubuntu version is supported, which keeps testing honest and
  small.

## How we check it

- Accessibility tests at the WCAG level in AGENTS.md: keyboard use,
  focus order, screen reader names, contrast, zoom, reduced motion and
  error messages.
- A test closes the window and proves the service and its jobs carry on.
- Visual regression tests for every SDS component.
