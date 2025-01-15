use std::fmt::Debug;
use bevy::prelude::*;
use mevy::*;

pub fn main() {
    let mut app = App::new();
    app.add_plugins(DefaultPlugins)
       .add_systems(Startup,startup);
    app.run();
}

fn startup(mut world:Commands){
    spawn!{Camera2d}
    spawn!{
        BackgroundColor(#ff0000);
        BorderColor(#00ffff);
        Node{ width:80px, height:80px, margin:[>16px], !};
        .observe(destroy_on::<Click>(e1));
        .observe(destroy_on::<Click>(named_child));
        [ // auto-called 'e1'
            Node{ width:20%, height:20%, !};
            BackgroundColor(#00ff00);
        ]
        [
            Node{ width:20%, height:20%, !};
            BackgroundColor(#00aaff);
            .observe(destroy_self_on::<Click>);
        ]
        [named_child][
            Node{ width:20%, height:20%, !};
            BackgroundColor(#00ffaa);
            Visibility!;
        ]
    }
}

fn destroy_on <E:Debug+Clone+Reflect> (entity:Entity) -> impl Fn(Trigger<Pointer<E>>,Commands) {
    move|_,mut world|{if let Some(mut ecmd) = world.get_entity(entity){
        ecmd.despawn();
    }}
}

fn destroy_self_on <E:Debug+Clone+Reflect> (trigger:Trigger<Pointer<E>>,mut cmd:Commands) {
    cmd.entity(trigger.entity()).despawn();
}
