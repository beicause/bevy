use bevy_utils::shader_layout;

shader_layout! {
    //~^ E0080
    pub struct MyUniform {
        a4: glam::Vec3,
    }
}
