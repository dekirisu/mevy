# Macros Overview

mevy provides three core macro families, each solving a different pain point in Bevy development. They're designed to work together — use `ui!{}` to build UI bundles, `code!{}` for inline values, and `entity!{}` to spawn and manage entities.

## Quick Reference

| Macro | Purpose | Use when you need... |
|---|---|---|
| `code!{}` | Inline value construction | Hex colors, `Val`, `UiRect` without function calls |
| `ui!{}` | CSS-like UI bundles | Multiple UI components without boilerplate |
| `entity!{}` | Entity spawning & modification | Hierarchies, queries, and observers in one expression |

## When to Use Which

- **Just a color or value?** → `code!{}`
- **A styled UI panel?** → `ui!{}`
- **A complete entity with children?** → `entity!{}`
- **All three together?** → Use them together. `entity!{}` nests `ui!{}` naturally, and `code!{}` works inside both.

## How They Work Together

```rust
entity!{
    <world>
    
    // ui!{} creates a bundle of UI components
    ui!((
        size: 100px 100px;
        background: #ff0000;
    ));
    
    // code!{} creates inline values
    BackgroundColor(code!{#00ff00});
    
    // Nested children with named entities
    [child][
        ui!(( size: 50px 50px; ));
        .observe(on_click);
    ]
}
```

## Troubleshooting

### "Missing bevy version" compile error

You need to specify the Bevy version feature in `Cargo.toml`:

```toml
mevy = { version = "0.4", features = ["0.18"] }
```

See [Installation](installation.md) for the full list of supported versions.

### Experimental features unexpectedly enabled

`mevy_ecs` has `experimental` as a default feature. To disable it:

```toml
mevy_ecs = { version = "0.3.0", default-features = false }
```

## Navigation

- [entity!{}](./macros-entity.md) — Entity spawning & modification
- [ui!{}](./macros-ui.md) — CSS-like UI notation
- [code!{}](./macros-code.md) — Code replacement macro
- [Experimental Helpers](./macros-experimental.md) — `gere!`, `geco!`, `cen!`, `den!`, `wen!`
