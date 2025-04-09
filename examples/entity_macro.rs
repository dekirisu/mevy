#![allow(dead_code)]
#![allow(path_statements)]
use bevy::{ecs::{relationship::RelatedSpawnerCommands, world::DeferredWorld}, prelude::*};
use mevy::*;
pub fn main() {}

// Commands: Spawning \\

    fn spawn_with_commands( mut commands: Commands ){
        entity!{
            <commands>
            Transform!;
            [spawned][Transform!]
        }
    }

    /// same as [spawn_with_commands]
    fn spawn_with_named_commands( mut world: Commands ){
        entity!{
            // implies <world>
            Transform!;
            [spawned][Transform!]
        >}  
        //^ using '>' (pointing outside) at the end will leak the entities, e.g. here:
        me;      // <- the root entity
        spawned; // <- the child named 'spawned'
    
        let _me = entity!(
            Transform!
        <);
        //^ using '<' (pointing inside) at the end will return the root entity
    }

// Commands: Modify w/ Provided Entity \\

    fn modify_with_commands( mut commands: Commands, entity: Entity ){
        entity!{
            <commands|entity>
            Transform!;
            [spawned][Transform!]
        }
    }

    fn modify_with_commands_redir( mut commands: Commands, entity: Entity ){
        entity!{
            <commands|entity>
            <Children.get(0).cloned()!>   // select first child, if available
            <Children.iter()>             // select all children of selected
            Visibility::Hidden;           // hide all of them
            .despawn();                   // despawn all of them
        }
    }

    /// same as [modify_with_commands]
    fn modify_with_named_commands( mut world: Commands, entity: Entity ){
        entity!{
            // empty before '|' implies a [Commands] names 'world'
            <|entity> 
            Transform!;
            [spawned][Transform!]
        }
    }

    /// same as [modify_with_commands]
    fn modify_with_named_entity( mut commands: Commands, me: Entity ){
        entity!{
            // empty after '|' implies an [Entity] names 'me'
            // NOTE: You have to type the '|', otherwise it's like [spawn_with_commands]
            <commands|> 
            Transform!;
            [spawned][Transform!]
        }
    }

    /// same as [modify_with_commands]
    fn modify_with_named_commands_entity( mut world: Commands, me: Entity ){
        entity!{
            // empty before '|' implies a [Commands] names 'world'
            // empty after '|' implies an [Entity] names 'me'
            <|>
            Transform!;
            [spawned][Transform!]
        }
    }

// Commands: Modify w/ Provided Resource \\

    #[derive(Resource,Component)]
    struct MultiEntities(Vec<Entity>);

    #[derive(Resource,Component)]
    struct AnEntity(Entity);

    impl AnEntity {
        fn get(&self) -> Option<Entity> {Some(self.0)}
    }

    fn modify_with_commands_resources( mut cmd: Commands ){
        entity!{
            // using '@' (SAFE MODE) tries to get an entity from a resource:
            // - use a path that returns: Option<Entity>
            // - does nothing if resource not available
            // - does nothing if return value is 'None'
            <cmd|@AnEntity.get()>
            Transform!;
            [spawned][Transform!]
        }
        // (SAFE MODE): 'spawned' entities CAN NOT be leaked here
    }

    fn modify_with_commands_resources_forced( mut cmd: Commands ){
        entity!{
            // using '@!' (RISKY MODE) to get an entity from a resource:
            // - use a path that returns: Entity
            // - panics if resource not available
            // - panics if return value is 'None'
            <cmd|@!AnEntity.0>
            Transform!;
            [spawned][Transform!]
        >}
        // (RISKY MODE): 'spawned' entities CAN be leaked here
    }

    fn modify_with_commands_resources_iter( mut cmd: Commands ){
        entity!{
            // using '@*' will target entities in a iterator of a resource:
            // - use a path that returns an Iterator over [Entity]s
            <cmd|@*MultiEntities.0.iter().cloned()>
            Transform!;
            [spawned][Transform!]
        }
    }

// Commands: Modify w/ Provided Component \\

    fn modify_with_commands_marker( mut cmd: Commands ){
        entity!{
            // using '#' without a trailing path will target EVERY entity With<Component>
            <cmd|#AnEntity>
            Transform!;
            [spawned][Transform!]
        }
    }

    fn modify_with_commands_component( mut cmd: Commands ){
        entity!{
            // using '#' (SAFE MODE) will target EVERY component in the world:
            // - use a path that returns: Option<Entity>
            <cmd|#AnEntity.get()>
            Transform!;
            [spawned][Transform!]
        }
    }

    fn modify_with_commands_component_forced( mut cmd: Commands ){
        entity!{
            // using '#!' (RISKY MODE) will target THE ONLY component in the world:
            // - use a path that returns: Option<Entity>
            // - panics if no entity has this component
            // - panics if more than one entity has this component
            <cmd|#!AnEntity.0>
            Transform!;
            [spawned][Transform!]
        }
        // (RISKY MODE): 'spawned' entities are available here
    }

    fn modify_with_commands_multi( mut cmd: Commands ){
        entity!{
            // using '#*' will target entities in a iterator of a component:
            // - use a path that returns an Iterator over [Entity]s
            <cmd|#*MultiEntities.0.iter().cloned()>
            Transform!;
            [spawned][Transform!]
        }
    }

// Other 'World Mutators' \\
// Only the first part of <..|..> changes, everything else is as described above 

    fn modify_with_entity_commands( mut ecmd: EntityCommands ){
        entity!{
            <*ecmd>
            Transform!;
            [spawned][Transform!]
        }
    }

    fn modify_with_child_builder<'a>( mut cbuild: RelatedSpawnerCommands<'a,ChildOf> ){
        entity!{
            <^cbuild>
            Transform!;
            [spawned][Transform!]
        }
    }

    fn modify_with_deferred_world( mut dworld: DeferredWorld, me: Entity ){
        entity!{
            <-dworld|>
            Transform!;
            [spawned][Transform!]
        }
    }

    fn modify_with_world( mut world: World, me: Entity ){
        entity!{
            <+world|>
            Transform!;
            [spawned][Transform!]
        }
    }

// Other 'World Mutators' Implied Name \\

    fn modify_with_entity_commands_implied_this( mut this: EntityCommands ){
        entity!{<>
            Transform!;
            [spawned][Transform!]
        }
    }

    fn modify_with_entity_commands_implied( mut world: EntityCommands ){
        entity!{<*>
            Transform!;
            [spawned][Transform!]
        }
    }

    fn modify_with_child_builder_implied<'a>( mut world: RelatedSpawnerCommands<'a,ChildOf> ){
        entity!{<^>
            Transform!;
            [spawned][Transform!]
        }
    }

    fn modify_with_deferred_world_implied( mut world: DeferredWorld, me: Entity ){
        entity!{<-|>
            Transform!;
            [spawned][Transform!]
        }
    }

    fn modify_with_world_implied( mut world: World, me: Entity ){
        entity!{<+|>
            Transform!;
            [spawned][Transform!]
        }
    }

    fn modify_with_entity_world_implied(mut world: EntityWorldMut ){
        entity!{<+*|>
            Transform!;
            [spawned][Transform!]
        }
    }

// EOF \\
