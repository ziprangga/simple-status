# simple-status

A **lightweight Rust crate** for emitting and tracking status events in asynchronous applications. Supports flexible **progress tracking**, **messages**, and **custom paths**, with **async-compatible channels** for reactive status updates.

## Features

* Asynchronous status event handling using `StatusReceiver`.
* Progress tracking with `stage`, `current`, and `total`.
* Custom messages with optional path and separator.
* Simple macros for emitting status events (`status!` and `status_emit!`).
* Thread-safe global emitter using `OnceLock<Arc<StatusEmitter>>`.
* Works out of the box in `iced` without requiring Tokio in your dependencies.

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
use simple_status::setup_status;
use iced::Task;

let mut rx = setup_status(10);

// Task to listen for status events
let emit_task = Task::perform(
    async move {
        while let Some(se) = rx.recv().await {
            se // handle StatusEvent
        }
    },
    |se| AppMessage::StatusEmit(se),
);
```

### Emit a status event

```rust
use simple_status::{status, status_emit};

// Using macro to build a custom event
let event = status!(
    stage: "Downloading",
    current: 3,
    total: 10,
    message: "Downloading file 3 of 10",
);

// Emit the event globally
status_emit!(stage: "Downloading", current: 3, total: 10, message: "Downloading file 3 of 10");

// Simple message
status_emit!("All tasks completed!");
```

### StatusEvent Builder

All fields are optional:

```rust
use simple_status::StatusEvent;

let event = StatusEvent::new()
    .with_stage("Processing")
    .with_current(2)
    .with_total(5)
    .with_message("Step 2 of 5")
    .with_path("/tmp/output")
    .with_separator("->");
```

## License

MIT or Apache-2.0 (choose one).
