# Changelog

All notable changes to this project will be documented in this file.


## v0.1.10

### Changes

* Add `Id` enum support for `StatusEvent`
* Make `Emitter`, `Receiver`, `EmitterHandler`, and `ReceiverHandler` generic over the transmitted value type
* Make `Channels<T>` generic with `StatusEvent` as the default value type
* Add ID support to `status!` and `status_emit!` macros
* Simplify status event construction and ID handling
* Update documentation and README
