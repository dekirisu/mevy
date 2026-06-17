# `entity!{}` — Entity Spawning & Modification

The `entity!{}` macro is mevy's most powerful feature. It handles entity spawning, hierarchy, modification, and queries all in one.

## Basic Spawning

```rust
entity!{
    Transform!;
    Name("Hello");
}
```

## Hierarchy with Children

Use `[...]` to create child entities:

```rust
entity!{
    Node { size: 100px 100px; ! }
    BackgroundColor(#ff0000);
    
    [child1][
        Node { size: 50px 50px; ! }
        BackgroundColor(#00ff00);
    ]
    
    [named_child][
        Node { size: 50px 50px; ! }
        BackgroundColor(#0000ff);
    ]
}
```

Children can be referenced by name **anywhere** — even before they're defined:

```rust
entity!{
    Node { entity: named_child; }  // reference before definition
    [named_child][
        // ...
    ]
}
```

## World/Entity Selection

The first entry `<...>` determines the world type and target entity:

### Spawning

```rust
entity!{ <world> }          // Commands (default)
entity!{ <commands> }       // named Commands
entity!{ <+world> }         // World
entity!{ <-world> }         // DeferredWorld
entity!{ <*this> }          // EntityCommands
entity!{ <^cbuild> }        // ChildBuilder
entity!{ <+*this> }         // EntityWorldMut
```

### Modifying

```rust
entity!{ <commands|entity> }   // Commands + specific Entity
entity!{ <|entity> }           // Commands + Entity (implicit)
entity!{ <world|> }            // Commands + new Entity (implicit)
entity!{ <|> }                 // Commands + me (implicit)
```

### Query Selectors

```rust
entity!{ <|#Marker> }          // All entities with Marker
entity!{ <|#Marker.get()> }    // Option<Entity> of a component
entity!{ <|#*Comp.all()> }     // Iterator over entities
entity!{ <|!#Marker> }         // Single entity (panics if none/multiple)
```

### Resource Selectors

```rust
entity!{ <|@Resource.get()> }   // Safe: does nothing if None
entity!{ <|!@Resource.0> }      // Risky: panics if None
entity!{ <|@*Resource.all()> }  // Iterate resource's entities
```

## Entity Redirection

Chain queries to target specific entities:

```rust
entity!{
    <world|#Marker>             // select: every Entity with Marker
    <Children.get(0).cloned()!> // > select: first child, if available
    <Children.iter()>           // >> select all children
    Visibility::Hidden;         // hide all of them
    .despawn();                 // despawn all of them
}
```

## Leaking & Returning

Symbols at the **end** of the macro control what's visible outside:

| Symbol | Behavior |
|---|---|
| `>` | Leak all spawned entities into scope |
| `<` | Return the root entity |
| `@` | Capture as closure `|mut world: Commands|` |

```rust
// Leak
entity!{
    Bundle::new(5);
    [child][]
>}
me;     // Entity
child;  // Entity (named child)

// Return
let enty = entity!{
    Bundle::new(5);
<};

// Closure
entity!{
    Bundle::new(5);
@};
```

## Quick Observer

Attach observers directly on entities:

```rust
entity!{
    // Single '>' — 'this' = EntityCommands
    > Pointer<Click> {
        this.despawn();
    }
    
    // Double '>>' — 'world' = mut World, 'entity' = Entity
    >> Pointer<Click> {
        world.queue(move |world: &mut World| {
            // do something with the world
        });
    }
}
```

## Alternative Syntax

mevy supports several shorthand notations inside `entity!{}`:

### `!` Default Shorthand

```rust
entity!{
    Bundle!;        // = Bundle::default();
    Bundle{a:3,!};  // = Bundle{a:3, ..default()};
    Bundle: 3;      // = Bundle::new(3);
    bundle_fn: 3;   // = bundle_fn(3);
    macro!: 3, 4;   // = macro!{3,4};
    .method: 5;     // = .method(5);
    
    BorderColor(#ff0000);  // Hex color
    Node{ left:10px, ! };  // CSS-like Val
}
```

### `try` Conditional Insertion

```rust
entity!{
    try SomeBundle;  // Only inserts if SomeBundle is Some
}
```

### `{..}` Free Code Block

```rust
entity!{
    {
        this.insert(Component);  // 'this' = EntityCommands
        this.observe(on_click);
    }
}
```

## Ancestors Array

Inside nested children, `ancestors[]` gives you parent entities:

```rust
entity!{
    [[[[[
        Component{ entity: ancestors[3] };
    ]]]]]
}
```

- `ancestors[0]` = direct parent
- `ancestors[n]` = root entity of the current macro call

## Complete Example

```rust
fn startup(mut world: Commands) {
    entity!{
        Camera2d;
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
