# `ui!{}` — CSS-like UI Notation

The `ui!{}` macro lets you write Bevy UI components with CSS-inspired syntax. Instead of constructing `Node`, `BackgroundColor`, `BorderRadius`, etc. by hand, you write properties that mevy translates into the appropriate Bevy components.

## Four Modes

The macro has four modes, selected by delimiters and naming. Each serves a different purpose:

### 1. Tuple Inline Mode — `ui!((...))`

Returns a tuple of the mentioned components, ready to be inserted into an entity:

```rust
cmd.spawn(ui!((
    size:          100px 100px;
    background:    #ff0000;
    border:        5px #00ff00;
    border_radius: 6px;
    box_shadow:    10% 10% 3px 8px #ffaa44;
)));
```

**Use this** when you need a bundle of UI components for a single entity. The order of components in the returned tuple is based on first mention and is consistent across calls.

### 2. Slim Inline Mode — `ui!(...)`

Short syntax inspired by TailwindCSS. Fields are separated by whitespace, and `:` ends the current field's value:

```rust
cmd.spawn(ui!(
    w:100 h:100 bg:#fff round:6 border:5#f00
    shadow:10%+10%+3px+8px#fa4
)?));
```

**Use this** for quick styling where you don't need the full CSS-like syntax. It's faster to type but less readable for complex layouts.

> [!NOTE]
> The trailing `?` is optional — any token works. It's just a visual cue that the macro returns a value. Hover over the call in your IDE to see the type.

### 3. Function Tuple Mode — `ui!{name(...)}`

Defines a function that returns `impl Bundle`. The function can be called like any other:

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

**Use this** when you want to define reusable UI components — like CSS classes or React components. Call them wherever you need that styled element.

### 4. Edit Function Mode — `ui!{name{...}}`

Defines a function that mutates existing components. The parameters are `&mut` references to the components that will be modified:

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

**Use this** when you want to modify existing entities — for example, adding a glow effect on hover. The `_` placeholder keeps the existing value for that field.

## CSS-like Notation

Inside `ui!{}`, you can write:

| Syntax | Result |
|---|---|
| `100px` | `Val::Px(100.)` |
| `50%` | `Val::Percent(50.)` |
| `3vw`, `1vh`, `2vmin`, `4vmax` | `Val::Vw/Vh/VMin/VMax` |
| `#ff0000` | `Color::Srgba(Srgba::hex("#ff0000").unwrap())` |
| `red`, `gray`, `cyan` | CSS color from `bevy::color::palettes::css` |
| `$my_var` | Pass a variable directly (e.g., `$handle`, `$image`) |

The `#` prefix for colors supports 3-digit (`#f00`), 4-digit (`#f00a`), 5-digit (`#f00a0`), 6-digit (`#ff0000`), and 8-digit (`#ff000080`) hex codes. CSS color names like `red`, `gray`, `cyan` are resolved from Bevy's color palette.

## Edge Selection (CSS-like)

For properties that apply to multiple edges (like `border` or `margin`), mevy follows CSS conventions:

```rust
border: 5px;           // all 4 edges
border: 5px 2px;       // vertical, horizontal
border: 5px 2px 8px;   // top, horizontal, bottom
border: 5px 2px 4px 1px; // top, right, bottom, left
```

The number of values determines the spread: 1 value = all edges, 2 = vertical/horizontal, 3 = top/horizontal/bottom, 4 = clockwise (top, right, bottom, left).

## Corner Selection (CSS-like)

For `border_radius`, mevy uses clockwise corner order:

```rust
border_radius: 5px;           // all 4 corners
border_radius: 5px 0px;       // top-left/right, bottom-left/right
border_radius: 5px 2px 8px;   // top-left, top-right, bottom
border_radius: 5px 2px 4px 1px; // clockwise: TL, TR, BR, BL
```

## Custom Fields

You can use functions returning `impl Bundle` as custom fields — like CSS classes:

```rust
fn neon_border(color: Color) -> Outline {
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
    Outline: 3px, 2px, #ff0000;  // calls Outline::new(3px, 2px, #ff0000)
)));
```

> [!NOTE]
> Variables (via `$var`) can only be used as values inside **custom fields**. Built-in field aliases (like `bg`, `w`, `px`) cannot reference variables directly. This is a known limitation.

## Complete Field Reference

See [CSS-like Fields](api-ui-fields.md) for the complete list of all supported fields.

## Slim Mode Aliases

See [CSS-like Fields](api-ui-fields.md) — each field section lists its slim alias in the second column.
