use bevy::prelude::*;
use mevy::*;

pub fn main() {
    let mut app = App::new();
    app.add_plugins(DefaultPlugins)
       .add_systems(Startup,startup);
    app.run();
}

fn startup(mut cmd:Commands){
    cmd.spawn(Camera2d::default());
    cmd.spawn(ui!((
        padding: 24px;
        column_gap: 24px;
    )))
    .with_children(|p|{
        p.spawn(neat_box());
        p.spawn(same_neat_box());
    });
}

// Bundles \\

    ui!{neat_box(
        size:          100px 100px;
        border:        5px #ff0000;
        box_shadow:    10% 10% 3px 8px #ffaa44;
        background:    #ffffff;
        border_radius: 6px;
        neat_outline;
    )?}

    ui!{neat_outline(
        outline: 3px 1px #00ff00;
    )}

    fn same_neat_box() -> impl Bundle {code!{{(
        Node{
            width:  100px,
            height: 100px,
            border: [>5px],
            ..default()
        },
        BoxShadow{
            color:         #ffaa44,
            x_offset:      10%,
            y_offset:      10%,
            blur_radius:   3px,
            spread_radius: 8px,
        },
        BackgroundColor(#ffffff),
        BorderColor(#ff0000),
        BorderRadius::all(6px),
        neat_outline()
    )}}}

// EOF \\
