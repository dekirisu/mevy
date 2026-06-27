# FAQ

Frequently asked questions about mevy.

## Do I need to enable all features?

No. Specify only the Bevy version feature (e.g., `features = ["0.18"]`). The `ui` feature is enabled by default. Only enable `experimental` if you need the experimental helper macros.

## Can I use mevy with any Bevy version?

mevy supports Bevy 0.15 through 0.18. You **must** pin your Bevy version in `Cargo.toml`:

```toml
mevy = { version = "0.4", features = ["0.18"] }
```

Without a version feature, compilation will fail with a `compile_error`.

## What happens if I forget the version feature?

You'll get a `compile_error!` at the macro expansion site telling you to specify the version feature. The error message will be clear about what's missing.

## Does mevy add runtime overhead?

No. mevy is purely compile-time token manipulation. All macros expand to regular Rust code — no plugins, no resources, no systems, no runtime dependency. The generated code is identical to what you'd write by hand.

## Are experimental features safe for production?

No. Experimental features are unstable and may change without notice. The core macros (`entity!{}`, `ui!{}`, `code!{}`) are stable and production-ready. Only use experimental features for prototyping.

## Can I use the individual crates separately?

Yes:

```toml
mevy_core = { version = "0.2.0", features = ["0.18"] }
mevy_ui   = { version = "0.4.0", features = ["0.18"] }
mevy_ecs  = { version = "0.3.0", features = ["0.18"] }
```

```rust
use mevy_core::code;
use mevy_ui::ui;
use mevy_ecs::entity;
```

This is useful if you only need specific macros or want to avoid the `experimental` default feature on `mevy_ecs`.

## Does `mevy_ecs` support Bevy 0.18?

Yes — all macros (`entity!{}`, `ui!{}`, `code!{}`) work with Bevy 0.18.

## Can I use variables inside `ui!{}` built-in fields?

Currently, variables can only be used in **custom fields** (via `$var`). Built-in field aliases (like `bg`, `w`, `px`) cannot reference variables directly. This is a known limitation.

## Is there LSP autocomplete support?

Yes. mevy is token-based, preserving the original token structure. LSP autocomplete works inside `entity!{}`, `ui!{}`, and `code!{}` — you get suggestions for Bevy types, methods, and components just like in regular Rust code.

## How does mevy compare to raw Bevy UI?

Significantly less boilerplate:

```rust
// Raw Bevy
cmd.spawn((Node { width: Val::Px(100.0), height: Val::Px(100.0), ..default() },
    BackgroundColor(Srgba::hex("#ff0000").unwrap().into()),
    BorderRadius::all(Val::Px(6.0))));

// mevy
cmd.spawn(ui!((size: 100px 100px; background: #ff0000; border_radius: 6px;)));
```

## When should I use `ui!{}` vs `code!{}`?

- **`ui!{}`** — when you need multiple UI components for an entity (a styled panel, button, etc.)
- **`code!{}`** — when you need a single value (a color, a `Val`, a `UiRect`)
- **`entity!{}`** — when you need to spawn or modify entities (with or without UI)

They're designed to work together: `entity!{}` nests `ui!{}`, and `code!{}` works inside both. See [Macros Overview](macros-overview.md) for more.
