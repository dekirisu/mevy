# Entity Selectors

Complete reference for `entity!{}` world and entity selectors.

## How Selectors Work

Every `entity!{}` call starts with a selector in `<...>`. The selector has two parts:

1. **World type** — which Bevy API to use (`Commands`, `World`, `DeferredWorld`, etc.)
2. **Entity target** — which entity to operate on (new, specific, or queried)

The syntax is: `<world_type|entity_target>`. The pipe `|` separates the two parts. If either part is omitted, sensible defaults apply.

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

The prefix character indicates the world type: `+` = `World`, `-` = `DeferredWorld`, `*` = `EntityCommands`, `^` = `ChildBuilder`. No prefix = `Commands`.

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

When modifying, the part before `|` is the world type and the part after is the entity. If either is empty, a default is used: empty before `|` = `Commands`, empty after `|` = `me: Entity`.

## Component Query Selectors

For targeting entities by component:

| Selector | Behavior |
|---|---|
| `!#Marker` | Single entity (panics if none/multiple) |
| `!#Marker.0` | Entity field on component (panics if None) |
| `#*Comp.all()` | Iterator over entities from component |

The `#` prefix means "find entities by component". The `!` prefix (placed before `#`) means "single entity" (panics if none/multiple). The `*` suffix (placed after `#`) means "iterator over entities".

> [!WARNING]
> Plain `#Marker` (without `!` or `*`) is **not** supported. Use `!#Marker` for a single entity or `#*Comp.all()` for an iterator.

## Resource Query Selectors

For targeting entities stored in resources:

| Selector | Behavior |
|---|---|
| `@Resource.get()` | Safe: does nothing if resource or value is `None` |
| `@*Resource.all()` | Iterate over entity iterator from resource |
| `!@Resource.0` | Risky: panics if resource or value is `None` |

The `@` prefix means "find entities from a resource". Safe selectors (`get()`) silently skip if the resource or value is missing. Risky selectors (`!`) panic.

> [!WARNING]
> `#Marker.get()` is **not** supported as a component query. It will be treated as a named entity, not a component selector.

## Redirection

After the initial selector, chain redirections to drill down:

```rust
entity!{
    <world|#*Marker.all()>       // initial: every Entity with Marker
    <Children.get(0).cloned()!>  // redirect: first child
    <Children.iter()>            // redirect: all children
    .despawn();                  // apply to all
}
```

Each `<...>` in the chain redirects the target. The final operations apply to whatever the last redirection selected.

## Leaking Symbols

At the **end** of the macro:

| Symbol | Behavior |
|---|---|
| `>` | Leak entities into scope |
| `<` | Return root entity |
| `@` | Capture as closure |

> [!WARNING]
> Resource/component selectors (`@`, `#`) only leak child entities, not the root.
