<p align="center">
    <img src="https://github.com/user-attachments/assets/829a86b8-8dc4-4403-9da4-536daaefbd11">
</p>
<p align="center">
    <a href="https://github.com/dekirisu/mevy" style="position:relative"><img src="https://img.shields.io/badge/github-dekirisu/mevy-ee6677"></a>
    <a href="https://crates.io/crates/mevy_ecs" style="position:relative"><img src="https://img.shields.io/crates/v/mevy_ecs"></a>
    <a href="https://discord.gg/kevWvBuPFg" style="position:relative"><img src="https://img.shields.io/discord/515100001903312898"></a>
</p>

This crate is part of [mevy](https://github.com/dekirisu/mevy) (tl;dr: more neat macros) so take a look! 🦆



## A simpler way of spawning
The macro `spawn!{..}` allows you to spawn hierarchies with this patter:
```rust
spawn!{

    // add components:
    Node!; // '!' is short for ::default()
    Outline{
        width: 2px, // 'Val's can be written css-like
        offset: 10%,
    !}; // '!' in a struct is short for ..default()

    // using methods:
    .remove::<Node>();
    .observe(..);
    .queue(..);

    // spawn children
    [optional_child_name]
        // component;
        // .method(..);
    ]
}
```

A simpler way to use triggers on self, basically means goated event control:
```rust
spawn!{
    Node{ width:50px, height:50px, ..default()};
    BackgroundColor(#ff0000); 
    // Using '>'
    > Pointer<Click> {
        // Provided variables:
        // 'this' = EnitityCommands
        // 'event' = e.g. &Pointer<Click>
        this.despawn();
    }
    // Using '>>'
    >> Pointer<Click> {
        // Provided variables:
        // 'world' = mut World
        // 'entity' = Entity
        // 'event' = e.g. &Pointer<Click>
    }
}
```

This macro expects a `mut world: Commands` variable (or `world: &mut Commands`)
```rust
fn startup(mut world: Commands){
    spawn!{Camera2d::default()}
}
```

The 'Child Names' are variables containing the child entity.
- Inside the macro they can be used anywhere, even 'before' you wrote them
- If none is provided - one will be generated: `e{number}`: `e0`, `e1`, ...
```rust 
spawn!{
    Component{ entity: named_child };
    [named_child][
        // components & methods
    ]
}
``` 

This is 'token based', which means it preserves LSP greatness.
```rust
spawn!{
    // typing . will fire LSPs autocomplete
    .obs // would suggest 'observer'
}
```

Easy way to address `ancestors` through a provided array, created within a macro call:
- first entry: the direct parent
- last entry: root entity of the current macro call
```rust
spawn!{
    [[[[[[
        SpecificEntity(ancestors[3]);
    ]]]]]]
}
```

To use the macro on an existing entity: The first thing sould be `&your_entity;`, which can be anything that returns a nentity:
```rust
    let entity = Entity::PLACEHOLDER;
    spawn!{
        &entity; // & -> anything that returns an entity
        // ... further macro usage
    }
```

## Synergies with [mevy](https://github.com/dekirisu/mevy) 
Using `mevy_core` macro `code!{}`, you can pair it with `code!{}` to write `Color`s and `Val`s neater:
```rust
code!{spawn!{
    BackgroundColor(#ff0000);
    Node{ width:50px, height:10%, margin:[>5px], ..default() }
    [
        // child ui
    ]
}}
```

Using `mevy_ui` macro `ui!{}`, it's a bit like html/css:
```rust
spawn!{
    ui!((
        size: 5px;
        box_shadow: 1px 2px 3px 4px #ff0000;
        background: cyan;
    ));
    [inner_box][ui!((
        size: 80%;
        background: green;
    ))]
}
```




