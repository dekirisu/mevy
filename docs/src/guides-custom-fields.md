# Custom Fields

Custom fields let you define reusable UI components in `ui!{}`. They're the mevy equivalent of CSS classes or React components — define a styled element once, use it everywhere.

## How Custom Fields Work

A custom field is any function that returns `impl Bundle`. When you write `my_field: value;` inside `ui!{}`, mevy resolves the field name:

- **Lowercase** field name → calls `my_field(value)` (function call)
- **Capitalized** field name → calls `MyField::new(value)` (struct `new()` method)

This means your custom field function name determines how it's called.

## Function-based Custom Fields

Any function returning `impl Bundle` can be used as a field:

```rust
fn neon_border(color: Color) -> Outline {
    Outline {
        width: Val::Px(3.0),
        offset: Vec2::splat(2.0),
        color,
    }
}

fn glow_box(color: Color) -> BoxShadow {
    BoxShadow(vec![ShadowStyle {
        color,
        x_offset: Val::Px(0.0),
        y_offset: Val::Px(0.0),
        blur_radius: Val::Px(20.0),
        spread_radius: Val::Px(5.0),
    }])
}

fn spawn_glowy_card(mut cmd: Commands) {
    cmd.spawn(ui!((
        size: 200px 150px;
        background: #1a1a2e;
        neon_border: #00ff00;   // calls neon_border(#00ff00)
        glow_box: #00ff00;       // calls glow_box(#00ff00)
        justify_content: center;
        align_items: center;
    )));
}
```

The function name becomes the field name. The value after `:` is passed as the argument.

## Struct `new()` Method

You can also target a struct's `new` method:

```rust
// Outline::new(width, offset, color)
cmd.spawn(ui!((
    Outline: 3px, 2px, #ff0000;
)));

// Any struct with `pub fn new(...)` works:
cmd.spawn(ui!((
    MyCustomBundle: arg1, arg2, #ff0000;
)));
```

The field name (capitalized) is treated as a type name, and `:` followed by values calls its `new()` method. This is useful for Bevy's built-in bundles like `Outline`, `BoxShadow`, etc.

## Edit Function Mode

For mutating existing components, use the edit function mode:

```rust
ui!{
    // This defines a function that takes &mut references
    into_glow{
        box_shadow: _ _ 10px 5px #ff0000;  // _ keeps existing values
        background: #ff0000;
    }
}

// Usage on existing entities:
entity!{
    <world|#*HoverTarget.all()>
    into_glow();  // applies the edits
}
```

The edit function mode (`ui!{name{...}}`) defines a function that takes `&mut` references to the components it modifies. The `_` placeholder keeps the existing value for that field.

## Prefab Pattern

Combine function mode with named components for full prefab support:

```rust
// Define prefabs
ui!{
    mevy_button(
        size: 120px 40px;
        background: #ee6677;
        border_radius: 6px;
        justify_content: center;
        align_items: center;
        gap: 8px;
    )?
}

ui!{
    mevy_card(
        size: 200px 150px;
        background: #1a1a2e;
        border: 2px #333;
        border_radius: 12px;
        padding: 12px;
        gap: 8px;
        flex_direction: column;
    )?
}

ui!{
    mevy_input(
        size: 100% 40px;
        background: #16213e;
        border: 1px #444;
        border_radius: 6px;
        padding_x: 12px;
    )?
}

// Use them
fn spawn_form(mut cmd: Commands) {
    entity!{
        [card][
            mevy_card();
            Text::new("Login");
            [input][
                mevy_input();
                Text::new("Username");
            ]
            [btn][
                mevy_button();
                Text::new("Submit");
            ]
        ]
    }
}
```

This is the prefab pattern: define a styled element once, call it like a function anywhere. It's the mevy equivalent of CSS classes or UI component libraries.

## Custom Field with Variables

Use `$var` to pass variables directly (single identifier only):

```rust
let my_image = handle;

cmd.spawn(ui!((
    image: $my_image;        // passes the variable directly
    image_color: #ff0000;    // hex color
)));
```

The `$` prefix tells mevy to pass the value as-is, without parsing it as a hex color or CSS name.

> [!NOTE]
> Only single identifiers work with `$` — `$path.to.image` is **not** supported. Use the full path directly: `image: path.to.image;`.

## Limitations

> [!NOTE]
> Variables can only be used in **custom fields**. Built-in field aliases (like `bg`, `w`, `px`) cannot reference variables directly. This is a known limitation that may be addressed in future versions.

## See Also

- [ui!{} Documentation](macros-ui.md) — All modes including edit function mode
- [CSS-like Fields](api-ui-fields.md) — Complete field reference
- [Building a UI](guides-building-a-ui.md) — UI composition patterns
