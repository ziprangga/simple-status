# simple-status

A **lightweight Rust crate** for emitting and tracking status events in asynchronous applications. Supports flexible **progress tracking**, **messages**, and **custom paths**, with **async-compatible channels** for reactive status updates.

## Features

* Asynchronous and synchronous status event handling using `Emitter` and `Receiver`.
* Flexible Progress Tracking: Built-in support for `stage`, `current`, `total`, `message`, and `PathBuf`.
* Simple macros for building and emitting status events: `status!` and `status_emit!`.
* Zero-Branching Strategy: Specialized `Mpsc` and `Broadcast` implementations for maximum performance.
* Thread-safe, `Arc`-wrapped channels for safe multi-threaded usage.
* Dual-Stream Support: `stream_sync()` for local borrows and `stream_async()` for 'static lifetimes (required by Iced Tasks).
* Works with `iced` or any async runtime (e.g., Tokio).

## Example

A complete working example is included in the [`sample`](./sample) crate.

### Run the example

```bash
cd sample
cargo run
```

## Installation

Add to your `Cargo.toml`:

```toml
[dependencies]
simple-status = "0.1.2"
iced = { version = "0.14", features = ["tokio"] }  # optional if using iced
```

## Usage

### Initialize the global status emitter (example using iced gui)

```rust
use simple_status::{init_channels, ChannelKind};

let channels = init_channels(10, ChannelKind::Broadcast);
```

### Receiving events asynchronously

```rust
if let Some(emitter) = &channel.get_emitter() {
    // when use argument Option<&Emitter>
    running_function(Some(&emitter)).await
    
    // Standard async receive
    if let Some(status) = channels.recv_async().await {
        println!("{}", status); 
    }

}


// For background tasks (returns a 'static stream), look sample for more info
if let Some(mut stream) = channels.stream_async() {
    while let Some(status) = stream.next().await {
        println!("Received: {}", status);
    }
}
```

### Emit a status event

```rust
use simple_status::{status, status_emit};

let emitter = channels.get_emitter();

// Using the builder-style macro
let event = status!(
    stage: "Downloading",
    current: 3,
    total: 10,
    message: "Downloading file 3 of 10",
);

// Return Status directly
status!("message")

// Emit asynchronously (await required)
status_emit!(async, emitter.as_ref(), stage: "Downloading", current: 3, total: 10, message: "Downloading file 3 of 10");

// Emit synchronously
status_emit!(emitter.as_ref(), "All tasks completed!");
```

### Build custom status

```rust
use simple_status::status;
use std::path::PathBuf;

let s = status!(
    stage: "Processing",
    message: "Analyzing data...",
    path: PathBuf::from("/logs/app.log"),
);
```

### Channels API

* `Emitter` – used to send status events asynchronously or synchronously.
* `Receiver` – used to receive status events asynchronously or synchronously.
* `Channels` – holds both emitter and receiver and allows creating new subscribers for broadcast channels.

```rust
let new_sub = channels.subscriber(); // Option<Arc<Receiver>>
```

## License

MIT or Apache-2.0 (choose one).
