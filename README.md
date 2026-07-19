# simple-status

A lightweight Rust crate for creating, emitting, and receiving status events.

The crate provides a simple event model (`StatusEvent`) together with flexible
channel abstractions for transporting events between components. Applications
can either use independent channels or store channels in a shared
`ChannelsBus` for application-wide access.

## Features

- Independent Channels: Create isolated event pipelines with `create_channels()`.
- Shared Channels: Store channels in a `ChannelsBus` and initialize them once with `init_channels()`.
- Synchronous and asynchronous event emission and reception.
- Built-in status event model with progress tracking, messages, paths, and optional identifiers.
- Optional event identifiers through `Id` (`u64`, `String`, or custom types).
- `status!` and `status_emit!` macros for concise event creation and emission.
- MPSC and Broadcast channel implementations.
- Broadcast subscriptions via `subscribe()`.
- Stream support through `stream()`.
- Thread-safe and async-runtime friendly.
- Works with Tokio, Iced, and other async ecosystems.

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
simple-status = "0.1.10"
iced = { version = "0.14", features = ["tokio"] }  # optional if using iced
```

## Usage

### Independent channels

Use independent channels when communication should remain local to a component,
subsystem, or test.

```rust
use simple_status::{
    create_channels,
    ChannelKind,
    status,
};

let channels = create_channels(10, ChannelKind::Broadcast);

channels.emit_sync(
    status!(
        action: "Build",
        current: 1,
        total: 5,
        message: "Compiling project",
    )
);

if let Some(event) = channels.recv_sync() {
    println!("{}", event);
}
```

### Shared channels with `ChannelsBus`

Use a `ChannelsBus` when channels should be shared across an application.

```rust
use simple_status::{
    ChannelKind,
    ChannelsBus,
    init_channels,
};

static STATUS_BUS: ChannelsBus = ChannelsBus::new();

fn main() {
    init_channels(&STATUS_BUS, 32, ChannelKind::Broadcast);
}
```
After initialization:

```rust
use simple_status::{emit_sync, recv_sync};

emit_sync(
    &STATUS_BUS,
    status!("Application started")
);

if let Some(event) = recv_sync(&STATUS_BUS) {
    println!("{}", event);
}
```

### Generic channels

`Channels`, `Emitter`, and `Receiver` are generic and default to `StatusEvent`.

```rust
use simple_status::{Channels, StatusEvent};

// Default type
let channels: Channels;
let channels: Channels<StatusEvent>;
```

```rust
use simple_status::Channels;

// Custom value types
let string_channels: Channels<String>;
let number_channels: Channels<u64>;
```

All channel types follow the same pattern:

```rust
use simple_status::{Channels, Emitter, Receiver};

let channels: Channels<String>;
let emitter: Emitter<String>;
let receiver: Receiver<String>;
```

The generic type must implement:

```rust
T: Send + Sync + Clone + 'static
```


### Event identifiers

Status events may optionally contain an identifier.

```rust
use simple_status::status;

let event = status!(
    id: 42u64,
    message: "Compiling",
);

assert_eq!(event.id().as_u64(), Some(42));

let event = status!(
    id: "build-01".to_string(),
    message: "Running",
);

assert_eq!(event.id().as_string(), Some("build-01"));
```

### Creating events

Use the `status!` macro to construct a `StatusEvent`.

```rust
use simple_status::status;

let event = status!(
    action: "Build",
    current: 2,
    total: 10,
    message: "Compiling project",
);
```

Message-only form:

```rust
use simple_status::status;

let event = status!("Build completed");
```

Formatting is also supported:

```rust
use simple_status::status;

let file = "main.rs";

let event = status!("Compiling {}", file);
```

### Emitting events with `status_emit!`

The `status_emit!` macro combines event construction and emission into a
single call.

#### Emit through a shared `ChannelsBus`

Using the `STATUS_BUS` defined in the previous section:

```rust
static STATUS_BUS: ChannelsBus = ChannelsBus::new();

init_channels(&STATUS_BUS, 32, ChannelKind::Broadcast);
```
Emit synchronously:

```rust
use simple_status::status_emit;

status_emit!(
    bus STATUS_BUS,
    "Application started"
);
```

Emit asynchronously:

```rust
# async {
use simple_status::status_emit;

status_emit!(
    async,
    bus STATUS_BUS,
    action: "Download",
    current: 5,
    total: 10,
);
# }
```

#### Emit through an emitter

Emit asynchronously:

```rust
use simple_status::{status_emit, StatusEmitter};

fn emit_sync_message(emitter: &StatusEmitter) {
    status_emit!(
        emitter,
        "{}",
        "this is EMIT SYNC INDEPENDENT".to_string()
    );
}
```

Emit asynchronously:

```rust
use simple_status::status_emit;

async fn emit_async_message(emitter: &StatusEmitter) {
    status_emit!(
        async,
        emitter,
        "{}",
        "this is EMIT ASYNC INDEPENDENT".to_string()
    );
}
```

### Receiving events asynchronously

```rust
if let Some(event) = channels.recv_async().await {
    println!("{}", event);
}
```

### Streaming events

```rust
use futures::StreamExt;

if let Some(mut stream) = channels.stream() {
    while let Some(event) = stream.next().await {
        println!("{}", event);
    }
}
```

### Broadcast subscriptions

Broadcast channels can create additional receivers.

```rust
if let Some(receiver) = channels.subscribe() {
    if let Some(event) = receiver.recv_async().await {
        println!("{}", event);
    }
}
```

## Channel Implementations

The crate currently provides two built-in channel implementations:

- `ChannelKind::Mpsc`
- `ChannelKind::Broadcast`

## Core Types

- `StatusEvent` – event data structure.
- `Emitter` – sends events.
- `Receiver` – receives events.
- `Channels` – owns an emitter and receiver pair.
- `ChannelsBus` – stores a shared channel set initialized once.

## License

MIT or Apache-2.0 (choose one).
