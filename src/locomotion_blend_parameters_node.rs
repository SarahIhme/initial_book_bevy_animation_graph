extern crate bevy;
extern crate bevy_animation_graph;

use bevy::prelude::*;
use bevy_animation_graph::core::{
    animation_node::{NodeLike, ReflectNodeLike},
    context::{new_context::NodeContext, spec_context::SpecContext},
    edge_data::{DataSpec, DataValue},
    errors::GraphError,
};

#[derive(Reflect, Clone, Debug, Default)]
#[reflect(Default, NodeLike)]
pub struct LocomotionBlendParametersNode;

// First we define the input and outputs of our node. We will take in all the values required for the calculations
// and output the result. We define the names of the input and outputs here.
impl LocomotionBlendParametersNode {
    // walk_base_speed
    pub const IN_WALK_BASE_SPEED: &'static str = "walk_base_speed";
    // run_base_speed
    pub const IN_RUN_BASE_SPEED: &'static str = "run_base_speed";
    // target_speed
    pub const IN_TARGET_SPEED: &'static str = "target_speed";
    // blend_start
    pub const IN_BLEND_START: &'static str = "blend_start";
    // blend_end
    pub const IN_BLEND_END: &'static str = "blend_end";
    // blend_factor
    pub const OUT_BLEND_FACTOR: &'static str = "blend_factor";
    // speed_factor
    pub const OUT_SPEED_FACTOR: &'static str = "speed_factor";
}

impl NodeLike for LocomotionBlendParametersNode {
    fn update(&self, mut ctx: NodeContext) -> Result<(), GraphError> {
        // We can read the inmput data. Nodes are evaluated lazily, so
        // whatever is connected to the inputs won't compute anything until we attempt to read them.
        // the distance a full walk cycle moves
        let walk_base_speed = ctx
            .data_back(Self::IN_WALK_BASE_SPEED)
            .unwrap_or(DataValue::F32(1.29))
            .into_f32()?;
        // the distance a full run cycle moves
        let run_base_speed = ctx
            .data_back(Self::IN_RUN_BASE_SPEED)
            .unwrap_or(DataValue::F32(3.54))
            .into_f32()?;
        let target_speed = ctx.data_back(Self::IN_TARGET_SPEED)?.into_f32()?;
        // blend start parameter: the speed at which to start blending with the run - experiment what feels good
        let blend_start = ctx
            .data_back(Self::IN_BLEND_START)
            .unwrap_or(DataValue::F32(1.9))
            .into_f32()?;
        // blend run parameter: the speed at which to always run - experiment what feels good
        let blend_end = ctx
            .data_back(Self::IN_BLEND_END)
            .unwrap_or(DataValue::F32(3.))
            .into_f32()?;

        // lets do the calculations described in the tutorial
        let blend_factor = ((target_speed - blend_start) / (blend_end - blend_start)).clamp(0., 1.);
        let speed_factor =
            target_speed / (walk_base_speed * (1. - blend_factor) + run_base_speed * blend_factor);

        // Publish the output pose to the corresponding output data pin
        ctx.set_data_fwd(Self::OUT_BLEND_FACTOR, blend_factor);
        ctx.set_data_fwd(Self::OUT_SPEED_FACTOR, speed_factor);

        Ok(())
    }

    fn display_name(&self) -> String {
        // This is the name that will be displayed in the editor for the node
        "Locomotion Blend Parameters node".into()
    }

    // Specify the data type for all the inputs and outputs
    fn spec(&self, mut ctx: SpecContext) -> Result<(), GraphError> {
        // Specify input data pins for this node with the correc type
        ctx.add_input_data(Self::IN_WALK_BASE_SPEED, DataSpec::F32);
        ctx.add_input_data(Self::IN_RUN_BASE_SPEED, DataSpec::F32);
        ctx.add_input_data(Self::IN_TARGET_SPEED, DataSpec::F32);
        ctx.add_input_data(Self::IN_BLEND_START, DataSpec::F32);
        ctx.add_input_data(Self::IN_BLEND_END, DataSpec::F32);

        // Specify output data pins for this node with the correct tyope
        ctx.add_output_data(Self::OUT_BLEND_FACTOR, DataSpec::F32);
        ctx.add_output_data(Self::OUT_SPEED_FACTOR, DataSpec::F32);

        Ok(())
    }
}
