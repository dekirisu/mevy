# `ui!{}` — CSS-like UI Notation

The `ui!{}` macro lets you write Bevy UI components with CSS-inspired syntax. It returns a tuple of the mentioned components, ready to be inserted into an entity.

## Modes

The macro has **four modes**, determined by delimiters and naming:

### 1. Tuple Inline Mode — `ui!((...))`

Returns a tuple of mentioned components:

```rust
cmd.spawn(ui!((
    size:          100px 100px;
    background:    #ff0000;
    border:        5px #00ff00;
    border_radius: 6px;
    box_shadow:    10% 10% 3px 8px #ffaa44;
)));
```

### 2. Slim Inline Mode — `ui!(...)`

Short syntax inspired by TailwindCSS. Use `:` after the last value to end the mode:

```rust
cmd.spawn(ui!(
    w:100 h:100 bg:#fff round:6 border:5#f00
    shadow:10%+10%+3px+8px#fa4
)?));
```

### 3. Function Tuple Mode — `ui!{name(...)}`

Defines a function that returns `impl Bundle`:

```rust
ui!{neat_box(
    size:       100px 100px;
    background: #ffffff;
)}

// Equivalent to:
// pub fn neat_box() -> impl Bundle { ui!(( size: 100px 100px; background: #ffffff; )) }

// Usage:
cmd.spawn(neat_box());
```

### 4. Edit Function Mode — `ui!{name{...}}`

Defines a function that mutates existing components:

```rust
ui!{into_red_glow{
    border: _ #ffffff;       // _ keeps existing width, changes color
    background: #ff0000;
    box_shadow: #ff0000;    // only color (no values = color only)
}}

// Equivalent to:
// pub fn into_red_glow(
//     border_color: &mut BorderColor,
//     background_color: &mut BackgroundColor,
//     box_shadow: &mut BoxShadow
// ) { ... }
```

## CSS-like Notation

Inside `ui!{}`, you can write:

| Syntax | Result |
|---|---|
| `100px` | `Val::Px(100.)` |
| `50%` | `Val::Percent(50.)` |
| `3vw`, `1vh`, `2vmin`, `4vmax` | `Val::Vw/Vh/VMin/VMax` |
| `#ff0000` | `Color::Srgba(Srgba::hex("#ff0000").unwrap())` |
| `red`, `gray`, `cyan` | CSS color from `bevy::color::palettes::css` |
| `$my_var` | Pass a variable directly |

## Edge Selection (CSS-like)

```rust
border: 5px;           // all 4 edges
border: 5px 2px;       // vertical, horizontal
border: 5px 2px 8px;   // top, horizontal, bottom
border: 5px 2px 4px 1px; // top, right, bottom, left
```

## Corner Selection (CSS-like)

```rust
border_radius: 5px;           // all 4 corners
border_radius: 5px 0px;       // top-left/right, bottom-left/right
border_radius: 5px 2px 8px;   // top-left, top-right, bottom
border_radius: 5px 2px 4px 1px; // clockwise: TL, TR, BR, BL
```

## Custom Fields

You can use functions returning `impl Bundle` as custom fields:

```rust
fn neat_outline(color: Color) -> Outline {
    Outline {
        width: Val::Px(3.0),
        offset: Vec2::splat(1.0),
        color,
    }
}

cmd.spawn(ui!((
    neat_outline: #00ff00;   // calls neat_outline(#00ff00)
)));
```

Or call a struct's `new` method:

```rust
cmd.spawn(ui!((
    Outline: 5px, 2px, #ff0000;  // calls Outline::new(5px, 2px, #ff0000)
)));
```

> **Tip**: Custom fields can only use variables (not built-in field aliases) in the current version.

## Complete Field Reference

See [CSS-like Fields](./macros-ui.md#complete-field-reference) for the complete list of all supported fields.

## Slim Mode Aliases

See [CSS-like Fields & Slim Aliases](./macros-ui.md#slim-mode-aliases) for the complete list of slim mode shortcuts.
