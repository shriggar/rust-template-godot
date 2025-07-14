use bevy::prelude::*;
use godot::prelude::*;
use godot_bevy::prelude::*;

#[bevy_app]
fn build_app(app: &mut App) {
    app.add_plugins(GodotDefaultPlugins)
        .add_systems(Startup, hello_world);
}

fn hello_world() {
    godot::prelude::godot_print!("Hello from godot-bevy!");
}
