# Introduction

**mevy** (pronounced "me-vee") is a growing set of Rust procedural macros that add "witchcraft" into [Bevy](https://bevyengine.org/). It provides **no plugins, resources, or systems** — only macros that make your Bevy code cleaner, faster to write, and more fun.

## What's Inside

mevy is organized into three crates:

| Crate | Purpose |
|---|---|
| `mevy_core` | The `code!{}` macro — hex colors, `Val`, `UiRect` shorthand |
| `mevy_ui` | The `ui!{}` macro — CSS-like UI notation for `bevy_ui` |
| `mevy_ecs` | The `entity!{}` macro — entity spawning, modification, and queries |

All three are re-exported under the single `mevy` crate for convenience.

## Key Features

### Simpler Entity Spawning

Spawn entities with nested hierarchy, named children, and chainable queries:

```rust
entity!{
    <world>
    Node { size: 100px 100px; ! }
    BackgroundColor(#ff0000);
    [child1][
        Node { size: 50px 50px; ! }
        BackgroundColor(#00ff00);
    ]
}
```

### CSS-like UI Notation

Write Bevy UI with CSS-inspired syntax:

```rust
cmd.spawn(ui!((
    size:          100px 100px;
    background:    #ff0000;
    border:        5px #00ff00;
    border_radius: 6px;
    box_shadow:    10% 10% 3px 8px #ffaa44;
)));
```

### Code Replacement

Magic syntax for types you use constantly:

```rust
let color = code!{#FF0000};
let val   = code!{100px};
let rect  = code!{[>10px 5px]};
```

## Supported Bevy Versions

mevy supports multiple Bevy versions through Cargo features:

| Feature | Bevy Version |
|---|---|
| `0.15` | Bevy 0.15.x |
| `0.16` | Bevy 0.16.x |
| `0.17` | Bevy 0.17.x |
| `0.18` | Bevy 0.18.x |

> **Tip**: Always specify your Bevy version feature in `Cargo.toml`. The macro behavior may differ between versions.

## Design Philosophy

> **Macros only.** No plugins, no resources, no systems. mevy gives you tools to write better Bevy code — it doesn't change how Bevy works.

- **Token-based**: Everything is compile-time token manipulation, preserving LSP autocomplete.
- **Version-aware**: Feature flags handle Bevy API changes between versions.
- **Composable**: `ui!{}` and `entity!{}` work together beautifully.
- **Extensible**: Custom fields let you define your own reusable UI components.

## Quick Links

- [Installation](installation.md) — How to add mevy to your project
- [Quick Start](quick-start.md) — Build your first mevy-powered app
- [Macros Overview](macros-overview.md) — All macro families
