# `code!{}` — Code Replacement Macro

The `code!{}` macro replaces shorthand patterns inside regular Rust code. It works anywhere you'd write a regular expression — in function arguments, struct fields, `let` bindings, etc.

## Hex Colors

Write `#hexcode` and it becomes a `Color`:

```rust
let color = code!{#FF0000};
// → Color::Srgba(Srgba::hex("#FF0000").unwrap())

let color = code!{#f00};
// → 3-digit shorthand: #ff0000

let color = code!{#ff000080};
// → 8-digit: includes alpha
```

### Chaining Methods

Since the result is a `Color`, you can chain methods:

```rust
code!{BoxShadow{
    color: #FF1265.mix(&#F93ECA, 0.4).with_alpha(0.2),
}}
```

## Values (`Val`)

Write numbers with units and they become `Val`:

| Syntax | Result |
|---|---|
| `100px` | `Val::Px(100.)` |
| `50%` | `Val::Percent(50.)` |
| `3vw` | `Val::Vw(3.)` |
| `1vh` | `Val::Vh(1.)` |
| `2vmin` | `Val::VMin(2.)` |
| `4vmax` | `Val::VMax(4.)` |
| `@` | `Val::Auto` |

```rust
let width = code!{100px};
let height = code!{50%};
let auto = code!{@};
```

## `UiRect`

Write `[>val1 val2 ...]` for CSS-like edge notation:

```rust
// Single value → all edges
code!{[>10px]}
// → UiRect { top: 10px, right: 10px, bottom: 10px, left: 10px }

// Two values → vertical, horizontal
code!{[>10px 5px]}
// → top/bottom: 10px, left/right: 5px

// Three values → top, horizontal, bottom
code!{[>10px 5px 8px]}

// Four values → top, right, bottom, left
code!{[>10px 5px 8px 3px]}
```

## Default Shorthand

Use `!` to insert `Default::default()`:

```rust
code!{Node{
    width: 100px,
    height: 100px,
    !        // ..Default::default()
}}
```

## Multiple Expressions

Write multiple statements inside the macro:

```rust
code!{
    let color1 = #FF0000;
    let color2 = #00FF00;
    let color3 = #0000FF;
}
```

## Complete Example

```rust
use bevy::prelude::*;
use mevy::*;

pub fn main() {
    println!("{:#?}", code!{BoxShadow(vec![ShadowStyle{
        color:         #FF1265.mix(&#F93ECA, 0.4).with_alpha(0.2),
        x_offset:      100px,
        y_offset:      50%,
        spread_radius: 3.1vh,
        blur_radius:   40.23vmax,
    }])});
}
```
