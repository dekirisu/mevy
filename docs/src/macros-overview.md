# Macros Overview

mevy provides three core macro families, each solving a different pain point in Bevy development.

## Quick Reference

| Macro | Purpose | Key Feature |
|---|---|---|
| `code!{}` | Inline value construction | Hex colors, `Val`, `UiRect` |
| `ui!{}` | CSS-like UI bundles | CSS properties, slim mode |
| `entity!{}` | Entity spawning & modification | Hierarchy, queries, observers |

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

## Navigation

- [entity!{}](./macros-entity.md) — Entity spawning & modification
- [ui!{}](./macros-ui.md) — CSS-like UI notation
- [code!{}](./macros-code.md) — Code replacement macro
- [Experimental Helpers](./macros-experimental.md) — `gere!`, `geco!`, `cen!`, `den!`, `wen!`
