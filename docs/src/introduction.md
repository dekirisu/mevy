# Introduction

**mevy** (pronounced "me-vee") is a set of Rust procedural macros that make writing [Bevy](https://bevyengine.org) code less tedious. It doesn't add plugins, resources, or systems — it just gives you better syntax for the patterns you already use.

## The Problem

Bevy's ECS and UI APIs are expressive but verbose. Consider spawning a styled panel — the same structure at three levels of verbosity:

```rust
// Raw Bevy

cmd.spawn((
    Node {
        width: Val::Px(300.0),
        height: Val::Px(200.0),
        padding: UiRect::all(Val::Px(16.0)),
        border_radius: BorderRadius::all(Val::Px(12.)),
        border: UiRect::all(Val::Px(2.0)),
        ..default()
    },
    BorderColor::all(Srgba::hex("#ee6677").unwrap().into()),
    BackgroundColor(Srgba::hex("#1a1a2e").unwrap().into()),
));
```

That's a lot of boilerplate for a simple rounded rectangle. With mevy's CSS-like mode:

```rust
// mevy — CSS-like mode
cmd.spawn(ui!((
    size: 300px 200px;
    padding: 16px;
    border_radius: 12px;
    border: 2px #ee6677;
    background: #1a1a2e;
)));  
```

For quick UI, the slim mode cuts it even further:

```rust
// mevy — slim mode
cmd.spawn(ui!(
    w:300 h:200 p:16 round:12 
    border:2px#ee6677 bg:#1a1a2e
)?));
```

Three levels. Same result. And your IDE still knows what's going on — mevy preserves the token structure so autocomplete and type inference work normally.

## What mevy Gives You

### A CSS-like language for Bevy UI

Write `border_radius: 12px` instead of `BorderRadius::all(Val::Px(12.0))`. mevy translates CSS concepts into Bevy components automatically. Edge selection (`border: 5px 2px`) and corner selection (`border_radius: 5px 2px 8px`) follow the same CSS conventions you already know.

### Nested entity spawning without callbacks

Bevy's `.with_children()` callback is the only way to build hierarchies, but it's hard to reference children by name. mevy's `[child_name][...]` syntax lets you name children and reference them anywhere - even before they're defined.

### Inline values without ceremony

Write `#ff0000` for a color, `100px` for a value, `[>10px 5px]` for a rectangle. No function calls, no type annotations, no `Srgba::hex().unwrap()` boilerplate.

## Design Philosophy

> **Macros only.** mevy doesn't change how Bevy works - it just makes the code you write cleaner.

- **Token-based**: Everything happens at compile time. Your IDE autocomplete, type inference, and error messages all work normally.
- **Version-aware**: Feature flags handle Bevy API changes between versions. You can upgrade Bevy without changing your mevy code.
- **Composable**: `ui!{}` and `entity!{}` work together. Use `ui!{}` inside `entity!{}` for nested UI, `code!{}` for inline values anywhere.
- **Extensible**: Define your own UI components with custom fields. Reuse them across your project like CSS classes.

## Supported Bevy Versions

mevy supports Bevy 0.15 through 0.18. You **must** pin your version in `Cargo.toml`:

```toml
mevy = { version = "0.3", features = ["0.18"] }
```

## Quick Links

- [Installation](installation.md) - How to add mevy to your project
- [Quick Start](quick-start.md) - Build your first mevy-powered app
- [Macros Overview](macros-overview.md) - All macro families
- [API Reference](api-reference.md) - Complete field and selector reference
- [FAQ](faq.md) - Frequently asked questions
- [Migration Guide](migration.md) - Migrate between Bevy versions
- [Changelog](changelog.md) - Release history
