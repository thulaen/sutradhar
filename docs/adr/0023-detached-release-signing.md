# ADR-0023: Detached release signing

## Decision

The owner approved detached signing for the Ubuntu and Debian
package and the XenForo and WordPress ZIPs in session 01a1f92b
on 21 November 2026. The signed release manifest binds the complete
unchanged package bytes and their required contents. Use the existing
Tijori release-manifest and release-record purposes.

Every supported delivery path verifies the exact bytes and preserves
their identity through use. The Windows wrapper keeps the independent
verification and unsigned warning in spec 0030.

## Reason and consequences

This preserves the bytes covered by the encryption proof and
qualification through delivery. Verification must use independently
trusted release authority. Native approval to install or activate
remains separate. Missing trusted package handoff or platform
coverage blocks the affected route.

[Spec 0005](../specs/0005-release-coordination.md) owns the binding
and tests. Specs 0030, 0031 and 0118 own installation and protection.
