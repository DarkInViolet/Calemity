# Calemity

A local first, privacy focused communication platform built with Rust, Tauri 2 and React.
Calemity aims to provide familiar community chat and messaging without mandatory dependence on a centralized, company controlled service.
The application is designed around local ownership: conversations live on the user's device, remain accessible offline, and can eventually synchronize across multiple devices.
Users will be able to connect Calemity to infrastructure they control, including a home server, NAS or VPS. Community operated and professionally managed hosting should also be possible without forcing everyone through one central Calemity service. (I know, it's grand! And I barely have any idea what I'm doing...but I believe!)

> Currently capable of displaying `hey` in a desktop window. Revolutionary stuff.

## Long-term goals
- Local-first messaging and offline access.
- Secure identities and multiple authorized devices.
- Direct messages, group chats, communities and channels.
- End-to-end encryption where appropriate.
- Multi-device message synchronization.
- Self-hosting on a PC, NAS, home server or VPS.
- Optional community-operated and managed hosting.
- Portable backups, exports and server migration.
- Voice, video and screen sharing later.
- Open protocols and community-built tools.
- No mandatory subscription tiers or artificial feature paywalls.
- Deep client customization, with themes and plugins planned later.

## Current status
Calemity is in very early development.
Currently implemented:

- Rust multi-crate workspace.
- Tauri 2 desktop application ("desktop application").
- React and TypeScript frontend (once again, maybe a little exaggeration).
- Local SQLite storage.
- Shared protocol models.
- Basic local message saving and loading.
Nothing should currently be considered production-ready, security-audited, or safe to entrust with the nuclear launch codes.

## Technology
- Rust
- Tauri 2
- React
- TypeScript
- SQLite
- SQLx
- Tokio
- Axum

## Project structure

```
Calemity/
├── apps/
│   └── desktop/
├── crates/
│   ├── calemity-api/
│   ├── calemity-protocol/
│   ├── calemity-storage/
│   ├── calemity-identity/
│   ├── calemity-sync/
│   └── calemity-server/
├── docs/
└── migrations/
```
## Hosting
Self-hosting is intended to be a first-class part of Calemity rather than an unsupported alternative to an official service.
In the future, communities should be able to run Calemity using infrastructure such as:
- A local computer or home server.
- A NAS.
- A VPS or dedicated server.
- A compatible third-party hosting provider.
- An optional officially managed Calemity hosting service.
Official Calemity hosting may be offered commercially in the future for users who prefer managed infrastructure, storage, backups, monitoring, upgrades and support.

Such services do not change the goal of keeping the core Calemity server open and independently self-hostable.

## Philosophy
Calemity is built **with the community, for the community**.

Commercial hosting, consulting and support are welcome. Improvements to the app should remain available to everyone, including improvements used by hosted services.

The goal is not to prevent people from building businesses around Calemity.

The goal is to prevent the shared platform from slowly becoming somebody else's closed garden with a premium badge attached.

## Why AGPL?
Calemity is intended to be a community driven communication platform. I welcome commercial use, hosting, and contributions. :3

The AGPL is used because I believe improvements to the platform should remain available to everyone, whether Calemity is distributed as software or offered as a hosted service.

With that said...I'm still shocked I managed to get a window to even pop up! If you've read this--have a lovely morning, noon, afternoon/evening!^^

## Contributing
Contributions are welcome!

Calemity uses the Developer Certificate of Origin (DCO) for contributions rather than requiring contributors to assign their copyright to the project.

See CONTRIBUTING.md for development, contribution and sign-off instructions.

## License
Calemity is licensed under the **GNU Affero General Public License v3.0 or later**.

The `calemity-protocol` crate is available under either the **MIT License** orthe **Apache License 2.0**, at the user's option.

The license declared in each crate's `Cargo.toml` is authoritative for that crate.


See [`LICENSE-AGPL`](LICENSE-AGPL), [`LICENSE-MIT`](LICENSE-MIT), and
[`LICENSE-APACHE`](LICENSE-APACHE) for the full license texts.
