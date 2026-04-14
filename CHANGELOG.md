# Changelog

All notable changes to this project will be documented in this file.

## - 2026-04-14
### Changed
- **Dependency Optimization**: Removed `async-stream` to reduce the dependency tree, improve compile times, and minimize binary size.
- **API Refactoring**: Consolidated `stream_sync` and `stream_async` into a single, unified `stream()` method for a cleaner developer experience.
- **Lifetime Improvements**: Updated the `Receiver` stream to return a `'static` lifetime, enabling seamless integration with `tokio::spawn` and `iced` Tasks.
- **Internal Logic**: Replaced macro-based stream generation with `futures::stream::unfold`.

## - 2026-04-14
### Added
- **Stability**: Established a standardized, stable architecture for channel communication logic.
- **Backend Support**: Integrated robust support for both `MPSC` (multi-producer, single-consumer) and `Broadcast` channel backends.
- **Dynamic Dispatch**: Implemented the `ReceiverHandler` trait to allow flexible switching between channel types at runtime.
