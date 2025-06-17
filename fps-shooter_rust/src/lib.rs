use bevy::prelude::*;
use godot_bevy::prelude::{
    godot_prelude::{ExtensionLibrary, gdextension, godot_print},
    *,
};

#[bevy_app]
fn build_app(app: &mut App) {
    // Print to the Godot console:
    // (https://docs.rs/godot-core/0.3.1/godot_core/macro.godot_print.html)
    godot_print!("Hello from Godot-Bevy!");

    // This line runs the `position_system` function every Godot render frame.
    //
    // Read more about Bevy "Systems" here:
    // (https://bevy.org/learn/quick-start/getting-started/ecs/).
    //
    // The `Update` schedule parameter is provided by Godot-Bevy.
    // It runs the system during Godot's `_process` update cycle.
    //
    // Read more about other schedules provided by Godot-Bevy here:
    // (https://github.com/dcvz/godot-bevy/blob/main/docs/TIMING_AND_SCHEDULES.md).
    app.add_systems(Update, position_system);
}

// A system is a normal Rust function. This system moves all Node2Ds to the right, such as Sprite2Ds.
//
// The `transform` parameter is a Bevy `Query` that matches all `Transform2D` components.
// `Transform2D` is a Godot-Bevy-provided component that matches all Node2Ds in the scene.
// (https://docs.rs/godot-bevy/latest/godot_bevy/plugins/core/transforms/struct.Transform2D.html)
//
// For more information on Bevy Components, Systems, and Queries, see:
// (https://bevy.org/learn/quick-start/getting-started/ecs/).
fn position_system(mut transform: Query<&mut Transform2D>) {
    // For single matches, you can use `single_mut()` instead:
    // `if let Ok(mut transform) = transform.single_mut() {`
    for mut transform in transform.iter_mut() {
        // Move the node to the right.
        transform.as_godot_mut().origin.x += 1.0;
    }
}
