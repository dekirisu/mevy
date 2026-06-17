# Quick Start

Let's build a simple Bevy app that showcases all three macro families.

## Setup

```rust
use bevy::prelude::*;
use mevy::*;

pub fn main() {
    let mut app = App::new();
    app.add_plugins(DefaultPlugins)
       .add_systems(Startup, startup);
    app.run();
}
```

## 1. Spawn UI with `ui!{}`

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

## 2. Use `code!{}` for inline values

```rust
fn startup(mut cmd: Commands) {
    // Hex colors and Val shorthand
    let shadow = code!{BoxShadow(vec![ShadowStyle{
        color:      #FF1265,
        x_offset:   100px,
        y_offset:   50%,
        blur_radius: 40px,
        spread_radius: 3vh,
    }])};
    
    let color = code!{#FF0000};
}
```

## 3. Entity hierarchy with `entity!{}`

```rust
fn startup(mut cmd: Commands) {
    entity!{
        Camera2d
        BackgroundColor(#0a0a0a);
        
        ui!((
            size: 200px 150px;
            background: #1a1a2e;
            border: 3px #ee6677;
            border_radius: 8px;
            justify_content: center;
            align_items: center;
        ));
        
        [button][
            ui!((
                size: 120px 40px;
                background: #ee6677;
                border_radius: 6px;
                justify_content: center;
                align_items: center;
            ));
            Text::new("Click me");
            > Pointer<Click> {
                this.insert(BackgroundColor(#ff4455));
            };
        ]
    }
}
```

## 4. Modify entities with queries

```rust
fn update(mut cmd: Commands) {
    entity!{
        <world|#Button>
        <Children.iter()>
        BackgroundColor(#00ff00);
    }
}
```

## Slim Mode

For quick UI, use the slim shorthand:

```rust
cmd.spawn(ui!(
    w:200 h:150 bg:#1a1a2e
    border:3px #ee6677 round:8px
    justify_content:center align_items:center
));
```

## What's Next

More documentation coming soon.
