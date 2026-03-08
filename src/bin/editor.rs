use bevy::app::App;
use bevy_animation_graph_book::locomotion_blend_parameters_node::LocomotionBlendParametersNode;
use bevy_animation_graph_editor::AnimationGraphEditorPlugin;

fn main() {
    let mut app = App::new();
    app.register_type::<LocomotionBlendParametersNode>();
    app.add_plugins(AnimationGraphEditorPlugin);

    app.run();
}
