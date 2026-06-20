# Custom Fields

Create reusable UI components with custom fields in `ui!{}`.

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
        neon_border: #00ff00;
        glow_box: #00ff00;
        justify_content: center;
        align_items: center;
    )));
}
```

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
    <world|#HoverTarget>
    into_glow();  // applies the edits
}
```

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

## Custom Field with Variables

Use `$var` to pass variables directly:

```rust
let my_image = handle;

cmd.spawn(ui!((
    image: $my_image;        // passes the variable directly
    image_color: #ff0000;    // hex color
)));
```
