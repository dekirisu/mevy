# Entity Selectors

Complete reference for `entity!{}` world and entity selectors.

## World Type Selectors

### Spawn Selectors

| Selector | World Type | Entity |
|---|---|---|
| *(none)* | `Commands` | new (spawned) |
| `world` | `Commands` (named) | new |
| `commands` | `Commands` (named) | new |
| `+world` | `World` | new |
| `-world` | `DeferredWorld` | new |
| `*this` | `EntityCommands` | new |
| `^cbuild` | `ChildBuilder` | new |
| `+*this` | `EntityWorldMut` | new |

### Modify Selectors

| Selector | World Type | Entity |
|---|---|---|
| `commands\|entity` | `Commands` | specific Entity |
| `\|entity` | `Commands` | specific Entity |
| `commands\|` | `Commands` | `me: Entity` |
| `\|` | `Commands` | `me: Entity` |
| `+world\|entity` | `World` | specific Entity |
| `-world\|entity` | `DeferredWorld` | specific Entity |
| `*this\|entity` | `EntityCommands` | specific Entity |
| `^cbuild\|entity` | `ChildBuilder` | specific Entity |
| `+*this\|entity` | `EntityWorldMut` | specific Entity |

## Component Query Selectors

| Selector | Behavior |
|---|---|
| `#Marker` | Every entity with `Marker` |
| `#Marker.get()` | `Option<Entity>` of a component |
| `#*Comp.all()` | Iterator over entities |
| `!#Marker` | Single entity (panics if none/multiple) |
| `!#Marker.0` | Entity field on component (panics if None) |

## Resource Query Selectors

| Selector | Behavior |
|---|---|
| `@Resource.get()` | Safe: does nothing if `None` |
| `@*Resource.all()` | Iterate resource's entities |
| `!@Resource.0` | Risky: panics if `None` |

## Redirection

After the initial selector, chain redirections:

````
entity!{
    <world|#Marker>              // initial: every Entity with Marker
    <Children.get(0).cloned()!>  // redirect: first child
    <Children.iter()>            // redirect: all children
    .despawn();                  // apply to all
}
````

## Leaking Symbols

At the **end** of the macro:

| Symbol | Behavior |
|---|---|
| `>` | Leak entities into scope |
| `<` | Return root entity |
| `@` | Capture as closure |

::: warning
Resource/component selectors (`@`, `#`) only leak child entities, not the root.
:::
