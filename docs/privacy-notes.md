# Privacy Notes

Calemity is intended to be privacy first.

A few design things I thought of:

- Components should only receive the data they actually need.
- Internal Rust models should not automatically be exposed to the frontend.
- User IDs, device IDs, timestamps and similar values are metadata and should be treated deliberately.
- Private keys and other secret material must never be exposed to the frontend.
- End-to-end encryption does not automatically protect metadata.
- Logs should avoid message contents, identifiers, keys and other sensitive data unless explicitly needed for debugging.
- Future plugins should use scoped permissions rather than unrestricted access to application state.

Current limitations: Calemity is still experimental, and the local SQLite database is not encrypted. :/

When adding a field to an API, frontend model, log, protocol message or plugin interface, ask:

> Does this component actually need to know this?

If not, the front-end can play hide without the seek! :3
