# Calemity — Open Questions

Calemity is still early, and there are a few things I genuinely haven't figured out yet.

My approach is pretty simple:

> Build small experiments, test the scary assumptions, and keep what actually works.

If you're reading this and have ideas, criticism, or experience with any of this, feel free to open an issue/discussion. ^^

## One-Click Hosting

I'd love Calemity to eventually be:

> Install → Create Community → Pick where it runs → Done.

No Docker tutorials, reverse proxies, port forwarding, TURN setup, DNS configuration, etc.

The problem is that once I actually look at everything required to make that reliable across random networks and devices... it's kind of insane. xD

I still want to see how close I can get.

## P2P vs Relays

Direct P2P is cheap and fast, but exposes participants' network addresses.

Relay-only networking protects peer IPs, but somebody has to pay for the bandwidth.

I've already tested relay-only Iroh connectivity across two separate networks successfully, but I haven't decided what the final transport policy should be.

Current thought:

- Private relay path by default.
- Never silently weaken privacy.
- Maybe explicit direct connections between trusted users later.
- Communities could provide their own relay/media infrastructure.

## Voice / Video / Screen Sharing

This is probably the biggest scalability problem.

Text traffic is tiny. Realtime media is not.

Things I still need to figure out:

- Who supplies SFU/TURN bandwidth?
- Can communities easily run their own media node?
- What can realistically be offered for free?
- How do I keep latency good without compromising privacy?

## Security / Identity / Storage

I'm experimenting with MLS/OpenMLS for E2EE.

A basic two-device encrypted DM already works, but there are still harder problems:

- Proving a device really belongs to a user.
- Device authorization/revocation.
- Multi-device recovery.
- Crash-safe MLS state.
- Encrypted local databases.
- Secure key storage and recovery.

I don't want Calemity to rely on one giant key or something silly like storing `database.key` beside `calemity.sqlite`.

## Small Directory Service

I'm fine with Calemity having a small central service for things like:

- Community discovery.
- Public account/device information.
- MLS KeyPackages.
- Revocation records.
- Signed connection information.

But it should never hold user private keys, plaintext messages, or storage keys.

The idea is for the service to distribute public/signed information while clients verify trust locally.

## The UX Problem

All of this is complicated.

The user ideally sees:

> Install → Create / Join → Talk.

Not:

> Configure TURN → inspect key packages → repair cryptographic state → cry.

If Calemity requires understanding its architecture to use it, I consider that a UX failure.

## Got an Idea?

If you've worked with secure messaging, QUIC, WebRTC/SFUs, distributed systems, storage encryption, or identity systems, I'd genuinely love input.

Even:

> "This idea is stupid, here's why."

is useful. :3
