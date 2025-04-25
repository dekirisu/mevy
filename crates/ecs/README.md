<p align="center">
    <img src="https://github.com/user-attachments/assets/829a86b8-8dc4-4403-9da4-536daaefbd11">
</p>
<p align="center">
    <a href="https://github.com/dekirisu/mevy" style="position:relative"><img src="https://img.shields.io/badge/github-dekirisu/mevy-ee6677"></a>
    <a href="https://crates.io/crates/mevy_ecs" style="position:relative"><img src="https://img.shields.io/crates/v/mevy_ecs"></a>
    <a href="https://discord.gg/kevWvBuPFg" style="position:relative"><img src="https://img.shields.io/discord/515100001903312898"></a>
</p>

These are ONLY proc-macros (no additional traits, structs, fns etc.).
This crate is part of [mevy](https://github.com/dekirisu/mevy) (tl;dr: more neat macros) so take a look! 🦆

## Setup
Multiple bevy versions are supported and managed by features:
```toml
# bevy 0.16
mevy_ecs = {version="0.2",features=["0.16"]}

# bevy 0.15
mevy_ecs = {version="0.2",features=["0.15"]}
```

# Rough Overview
```rust
entity!{
    <..>                  // World/Entity Selection
    Bundle::new(..);      // Insert Bundle to Selected
    .observe(..);         // Use a method
    {..}                  // Free code block w/ 'this: EntityCommands'
    > Pointer<Click>{..}  // Quick Observe w/ 'this: EntityCommands'
    >> Pointer<Click>{..} // Quick Observe w/ 'world: World', 'entity: Entity'
    [                     // Spawn a Child
        Bundle::new(..);  // Insert Bundle to Child
        .observe(..);     // Use method on Childs EnitityCommands/EntityWorldMut
        {ancestors[0]}    // ancestors: Vec<Entity>
                          // first = parent, last = 'selected root'
    ]
    [named_child][        // Spawn a Child with a name
        // ..
    ]
}
```

# Alternative Syntax
```rust 
entity!{
    <..>

    Bundle!;       // = Bundle::default();
    Bundle{a:3,!}; // = Bundle{a:3,..default()};
    Bundle: 3;     // = Bundle::new(3);
    bundle_fn: 3;  // = bundle_fn(3);
    macro!: 3, 4;  // = macro!{3,4};
    .method: 5;    // = .macro(5);
    
    // any plain Hex-Code = [Color]
    BorderColor(#ff0000);

    // css like [Val]s
    Node{
        left:   10px;
        top:    5%;
        width:  3vw;
        height: 6vmax;
    !};

}
```

# World/Entity Selection
The first entry of the macro determines which world access is used and which entity is aimed.
```rust
entity!{

    <cmd>       // SPAWN an entity using this [Commands]
                // (no entry) assumes a 'world: Commands'

    <cmd|enty>  // MODIFY an entity: pass a Commands | Entity
    <|enty>     // assumes a 'world: Commands'
    <cmd|>      // assumes a 'me: Entity'
    <|>         // assumes both

    <+world>    // SPAWN using this [World]
    <+world|..> // MODIFY using this [World]
    <+> <+|..>  // assumes a 'world: World'

    <-world>    // SPAWN using this [DeferredWorld]
    <-world|..> // MODIFY using this [DeferredWorld]
    <-> <-|..>  // assumes a 'world: DeferredWorld'

    <*this>     // MODIFY using this [EntityCommands]
    <*>         // assumes a 'world: EntityCommands'
    <>          // assumes a 'this: EntityCommands'

    <^this>     // MODIFY the parent of a [ChildBuilder]
    <^>         // assumes a 'world: ChildBuilder'

    <+*this>    // MODIFY using this [EntityWorldMut]
    <+*>        // assumes a 'world: EntityWorldMut'

    <|#Comp>         // target EVERY entity with this component
    <|#Comp.get()>   // ..or an [Option<Entity>] of the component
    <|#*Comp.all()>  // ..or any iterator over [Entity]s
    
    <|!#Comp>        // target THE ONLY (.single()) entity, enables 'leaking'
    <|!#Comp.0>      // ..or an [Entity] on the component

    <|@Comp.get()>   // target an [Option<Entity>] on a resource 
    <|@*Comp.all()>  // ..or any iterator over [Entity]s
    
    <|!@Comp.0>      // target an [Entity] on a resource, enables 'leaking'

}
```

# Redirection
After the initial selection of entities, you can redirect it to entities of a component.
This expects: `<impl Component>.<path to an impl Iterator<Item=Entity>>` or `<impl Component>.<path to a Some<Entity>!` (mind the '!').
```rust
entity!{
    <world|#Marker>             // select: every Entity with [Marker]
    <Children.get(0).cloned()!> // > select: first child, if available
    <Children.iter()>           // >> select all children
    Visibility::Hidden;         // hide all of them
    .despawn();                 // despawn all of them
}
```

# Leaking / Returning
If the selector can 'leak' entities, you can use one  of those symbold a the END of the macro:
- `>` 'leak': every spawned entity is available in this scope
- `<` 'return': returns the root entity

```rust
let enty = entity!{
    Bundle::new(5);
<};

entity!{
    Bundle::new(5);
    []
    [named][]
>}    
me;   // the spawned entity
e1;   // the unnamed child entity
named // the named child entity

entity!{
    <|!#Comp>
    [named][]
>}
named // resource/component selectors only leak children
```

## Quick Observe
A simpler way to use triggers on self, basically means goated event control:
```rust
spawn!{
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

```rust
fn startup(mut world: Commands){
    spawn!{Camera2d::default()}
}
```

# Child Names
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

## Example
Combining the things you could write thing like this:
```rust 
fn startup(mut world: Commands){
    entity!{
        Name: "Root";
        Node{ padding:10px, !};
        BackgroundColor(#ff0000);
        Marker;
        [nice_text][
            Name: "Some Name";
            Text: "Hello World";
            > Pointer<Click> {this.despawn();};
        ]
    }
}
```

And modify it by a marker:
```rust
fn update(mut world:Commands){
    entity!{
        <|#Marker>
        BackgroundColor(#00ff00);
    }
}
```

## Synergies with [mevy](https://github.com/dekirisu/mevy) 
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
