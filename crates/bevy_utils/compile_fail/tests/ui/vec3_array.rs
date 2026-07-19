use bevy_utils::shader_layout;

shader_layout! {
    pub struct MyUniformVec3F {
        a4: [glam::Vec3; 1],
        //~^ E0277
    }
}

shader_layout! {
    pub struct MyUniformVec3I {
        a4: [glam::IVec3; 1],
        //~^ E0277
    }
}

shader_layout! {
    pub struct MyUniformVec3U {
        a4: [glam::UVec3; 1],
        //~^ E0277
    }
}
