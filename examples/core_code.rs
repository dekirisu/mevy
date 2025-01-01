use bevy::prelude::*;
use mevy::*;

pub fn main() {
    println!{"{:#?}",code!{BoxShadow{
        // use #... is replaced with Color, meaning you can e.g. use methods 
        color: #FF1265.mix(&#F93ECA,0.4).with_alpha(0.2),
        x_offset: 100px,
        y_offset: 50%,
        spread_radius: 3.1vh,
        blur_radius: 40.23vmax,
    }}};
}
