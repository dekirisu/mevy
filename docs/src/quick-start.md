# Quick Start

This guide walks you through mevy's three macro families, from setup to a working example.

## Setup

Add mevy to your `Cargo.toml` with your Bevy version:

```toml
mevy = { version = "0.4", features = ["0.18"] }
```

Then import everything:

```rust
use bevy::prelude::*;
use mevy::*;
```

## Step 1: CSS-like UI with `ui!{}`

The `ui!{}` macro lets you write Bevy UI components using CSS-inspired syntax. Instead of constructing `Node`, `BackgroundColor`, `BorderRadius`, etc. by hand, you write properties:

```rust
fn startup(mut cmd: Commands) {
    cmd.spawn(Camera2d::default());
    
    cmd.spawn(ui!((
        size:         200px 150px;
        background:   #1a1a2e;
        border:       3px #ee6677;
        border_radius: 8px;
        justify_content: center;
        align_items: center;
    )));
}
```

The same result in slim mode — fewer keystrokes, same components:

```rust
cmd.spawn(ui!(
    w:200 h:150 bg:#1a1a2e
    border:3px #ee6677 round:8
    justify_content:center align_items:center
)?));
```

See [ui!{} docs](macros-ui.md) for all fields and modes.

> [!NOTE]
> The `?` at the end is optional — any token works. It's just a visual cue that the macro returns a value. Hover over the call in your IDE to see the type.

## Step 2: Inline values with `code!{}`

The `code!{}` macro replaces tedious patterns with shorthand inside regular Rust code. It works anywhere — in struct fields, function arguments, `let` bindings, etc.:

```rust
// Hex colors
let color = code!{#FF0000};

// Values with units
let width = code!{100px};
let auto  = code!{@};

// UiRect with CSS-like edge notation
let margin = code!{[>10px 5px]};
```

You can also chain methods on the result:

```rust
code!{BoxShadow{
    color: #FF1265.mix(&#F93ECA, 0.4).with_alpha(0.2),
}}
```

See [code!{} docs](macros-code.md) for all patterns.

## Step 3: Entity hierarchy with `entity!{}`

The `entity!{}` macro handles entity spawning, children, and modification in one expression. Children are created with `[name][...]` and can be referenced by name anywhere — even before they're defined:

```rust
fn startup(mut cmd: Commands) {
    entity!{
        Camera2d;
        BackgroundColor(#0a0a0a);
        
        [button][
            ui!((
                size: 120px 40px;
                background: #ee6677;
                border_radius: 6px;
                justify_content: center;
            ));
            Text::new("Click me");
            > Pointer<Click> {
                this.insert(BackgroundColor(#ff4455));
            };
        ]
    }
}
```

The `> Pointer<Click> { ... }` syntax attaches an observer directly on the entity. No need for separate observer registration.

See [entity!{} docs](macros-entity.md) for all features.

## Step 4: Entity queries

Target and modify entities using component queries — no separate system needed:

```rust
fn update(mut cmd: Commands) {
    entity!{
        <world|#*Button.all()>   // every entity with Button component
        <Children.iter()>         // then their children
        BackgroundColor(#00ff00);
    }
}
```

See [Entity Queries guide](guides-entity-queries.md) for detailed patterns.

## How they work together

All three macros are designed to nest: `entity!{}` contains `ui!{}` for UI, and `code!{}` works inside both:

```rust
entity!{
    ui!((
        background: code!{#ff0000};  // code!{} inside ui!{}
    ));
    [child][
        ui!(( size: 50px; ));
    ]
}
```

## Slim Mode

For quick UI, use the slim shorthand — inspired by TailwindCSS:

```rust
cmd.spawn(ui!(
    w:200 h:150 bg:#1a1a2e
    border:3px #ee6677 round:8px
));
```

See [ui!{} Slim Mode](macros-ui.md#2-slim-inline-mode) for all shortcuts.

## What's Next

- [entity!{} docs](macros-entity.md) — Entity spawning & modification
- [ui!{} docs](macros-ui.md) — CSS-like UI notation
- [code!{} docs](macros-code.md) — Inline value construction
- [Building a UI guide](guides-building-a-ui.md) — Complex layouts
- [Entity Queries guide](guides-entity-queries.md) — Query patterns
- [FAQ](faq.md) — Common questions
