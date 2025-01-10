use bevy::prelude::*;
use mevy::*;

pub fn main() {
    let mut app = App::new();
    app.add_plugins(DefaultPlugins)
       .add_systems(Startup,startup);
    app.run();
}

fn startup(mut world:Commands){
    spawn!{Camera2d::default()}


    spawn!{
        ui!((
            size: 80px 50px;
            background: gray;
            border_radius: 8px;
            justify_self: center;
            align_self: center;
        ));
        > Pointer<Click> {
            this.despawn(); 
        };
        > Pointer<Over> {
            this.insert(ui!((
                background: red;
                border_radius: 16px;
            )));
        };
        > Pointer<Out> {
            this.insert(ui!((
                background: gray;
                border_radius: 8px;
            )));
        };
    }


}

