# Installation

## Add to Cargo.toml

```toml
[dependencies]
bevy = { version = "0.18", features = ["ui"] }
mevy = { version = "0.3", features = ["0.18"] }
```

Then import everything at once:

```rust
use bevy::prelude::*;
use mevy::*;
```

That's all — `entity!{}`, `ui!{}`, and `code!{}` are now available.

## Why the Version Feature?

Bevy changes its APIs between minor versions. mevy handles these differences for you, but it needs to know which version you're targeting to generate the correct code. The version feature tells the macro which Bevy API to emit.

> [!IMPORTANT]
> Forgetting the version feature will cause a compile error. The error message will tell you exactly what to add.

## Selecting a Bevy Version

| Bevy Version | mevy Feature |
|---|---|
| 0.15.x | `features = ["0.15"]` |
| 0.16.x (stable) | `features = ["0.16"]` |
| 0.16.x (RC) | `features = ["0.16-rc"]` |
| 0.17.x | `features = ["0.17"]` |
| 0.18.x | `features = ["0.18"]` |

## Optional Features

| Feature | Description |
|---|---|
| `ui` (default) | Enables the `ui!{}` macro for CSS-like UI notation |
| `experimental` | Enables experimental helper macros (`gere!`, `geco!`, etc.) |

The `experimental` feature is unstable — the API may change between versions without notice. Only enable it if you need those helpers and are comfortable with potential breakage.

```toml
# With experimental features
mevy = { version = "0.3", features = ["0.18", "experimental"] }
```

## Using Crates Separately

You can also use individual crates if you only need specific macros:

```toml
[dependencies]
mevy_core = { version = "0.1.1", features = ["0.18"] }  # code!{} only
mevy_ui   = { version = "0.3.2", features = ["0.18"] }  # ui!{} only
```

```rust
use mevy_core::code;  // hex colors, Val, UiRect
use mevy_ui::ui;      // CSS-like UI notation
```

This is useful if you want to avoid the `experimental` default feature on `mevy_ecs`, or if your project only needs one macro family.
