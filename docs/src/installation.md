# Installation

## Add to Cargo.toml

Add mevy to your project with the appropriate Bevy version feature:

```toml
[dependencies]
bevy = { version = "0.18", features = ["ui"] }
mevy = { version = "0.3", features = ["0.18"] }
```

### Selecting a Bevy Version

| Bevy Version | mevy Feature |
|---|---|
| 0.15.x | `features = ["0.15"]` |
| 0.16.x | `features = ["0.16"]` |
| 0.17.x | `features = ["0.17"]` |
| 0.18.x | `features = ["0.18"]` |

### Optional Features

| Feature | Description |
|---|---|
| `ui` (default) | Enables the `ui!{}` macro |
| `experimental` | Enables experimental helpers (`gere!`, `geco!`, etc.) |

```toml
# With experimental features
mevy = { version = "0.3", features = ["0.18", "experimental"] }
```

## Usage

```rust
use bevy::prelude::*;
use mevy::*;
```

That's it — all three macro families are available through the single `use mevy::*;`.

## Standalone Crates

You can also use the individual crates independently:

```toml
[dependencies]
mevy_core    = { version = "0.1", features = ["0.18"] }
mevy_ui      = { version = "0.3", features = ["0.18"] }
mevy_ecs     = { version = "0.2", features = ["0.18"] }
```

```rust
use mevy_core::code;
use mevy_ui::ui;
use mevy_ecs::entity;
```
