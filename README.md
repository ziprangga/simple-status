# simple-status

A **lightweight Rust crate** for emitting and tracking status events in asynchronous applications. Supports flexible **progress tracking**, **messages**, and **custom paths**, with **async-compatible channels** for reactive status updates.

## Features

* Asynchronous and synchronous status event handling using `Emitter` and `Receiver`.
* Progress tracking with `stage`, `current`, and `total`.
* Custom messages with optional file paths.
* Simple macros for building and emitting status events: `status!` and `status_emit!`.
* Supports multiple channel types: `Mpsc` and `Broadcast`.
* Thread-safe, `Arc`-wrapped channels for safe multi-threaded usage.
* Works with `iced` or any async runtime (e.g., Tokio).

## Installation

Add to your `Cargo.toml`:

```toml
[dependencies]
simple-status = "0.1"
iced = { version = "0.12", features = ["tokio"] }  # optional if using iced
```

## Usage

### Initialize the global status emitter (example using iced gui)

```rust
use simple_status::{init_channels, ChannelKind};

let channels = init_channels(10, ChannelKind::Broadcast);
let emitter = channels.emitter();   // Arc<Emitter>
let receiver = channels.receiver(); // Arc<Receiver>
```

### Receiving events asynchronously

```rust
use simple_status::Status;

async fn listen(receiver: Arc<simple_status::Receiver>) {
    while let Some(status) = receiver.async_recv().await {
        println!("{}", status); // handle Status
    }
}
```

### Emit a status event

```rust
use simple_status::{status, status_emit};

// Using the builder-style macro
let event = status!(
    stage: "Downloading",
    current: 3,
    total: 10,
    message: "Downloading file 3 of 10",
);

// Emit asynchronously (await required)
status_emit!(async, emitter.as_ref(), stage: "Downloading", current: 3, total: 10, message: "Downloading file 3 of 10");

// Emit synchronously
status_emit!(emitter.as_ref(), "All tasks completed!");
```

### Build custom status

```rust
use simple_status::{build_status, Status};
use std::path::PathBuf;

let status: Status = build_status(
    Some("Processing".into()),
    Some(2),
    Some(5),
    Some("Step 2 of 5".into()),
    Some(PathBuf::from("/tmp/output")),
);
```

### Channels API

* `Emitter` – used to send status events asynchronously or synchronously.
* `Receiver` – used to receive status events asynchronously or synchronously.
* `Channels` – holds both emitter and receiver and allows creating new subscribers for broadcast channels.

```rust
let new_sub = channels.new_subscriber(); // Option<Arc<Receiver>>
```

## License

MIT or Apache-2.0 (choose one).
