use crate::{primitives::Frustum, Camera, CameraProjection, OrthographicProjection, Projection};
use bevy_ecs::prelude::*;
use bevy_math::UVec2;
use bevy_reflect::{std_traits::ReflectDefault, Reflect, ReflectDeserialize, ReflectSerialize};
use bevy_transform::prelude::{GlobalTransform, Transform};
use serde::{Deserialize, Serialize};
use smallvec::SmallVec;
use wgpu_types::{LoadOp, TextureFormat, TextureUsages};

/// A 2D camera component. Enables the 2D render graph for a [`Camera`].
#[derive(Component, Default, Reflect, Clone)]
#[reflect(Component, Default, Clone)]
#[require(
    Camera,
    Projection::Orthographic(OrthographicProjection::default_2d()),
    Frustum = OrthographicProjection::default_2d().compute_frustum(&GlobalTransform::from(Transform::default())),
)]
pub struct Camera2d;

/// A 3D camera component. Enables the main 3D render graph for a [`Camera`].
///
/// The camera coordinate space is right-handed X-right, Y-up, Z-back.
/// This means "forward" is -Z.
#[derive(Component, Reflect, Clone)]
#[reflect(Component, Default, Clone)]
#[require(Camera, Projection)]
pub struct Camera3d {
    /// The depth clear operation to perform for the main 3d pass.
    pub depth_load_op: Camera3dDepthLoadOp,
    /// The texture usages for the depth texture created for the main 3d pass.
    pub depth_texture_usages: Camera3dDepthTextureUsage,
}

impl Default for Camera3d {
    fn default() -> Self {
        Self {
            depth_load_op: Default::default(),
            depth_texture_usages: TextureUsages::RENDER_ATTACHMENT.into(),
        }
    }
}

#[derive(Clone, Copy, Reflect, Serialize, Deserialize)]
#[reflect(Serialize, Deserialize, Clone)]
pub struct Camera3dDepthTextureUsage(pub u32);

impl From<TextureUsages> for Camera3dDepthTextureUsage {
    fn from(value: TextureUsages) -> Self {
        Self(value.bits())
    }
}

impl From<Camera3dDepthTextureUsage> for TextureUsages {
    fn from(value: Camera3dDepthTextureUsage) -> Self {
        Self::from_bits_truncate(value.0)
    }
}

/// The depth clear operation to perform for the main 3d pass.
#[derive(Reflect, Serialize, Deserialize, Clone, Debug)]
#[reflect(Serialize, Deserialize, Clone, Default)]
pub enum Camera3dDepthLoadOp {
    /// Clear with a specified value.
    /// Note that 0.0 is the far plane due to bevy's use of reverse-z projections.
    Clear(f32),
    /// Load from memory.
    Load,
}

impl Default for Camera3dDepthLoadOp {
    fn default() -> Self {
        Camera3dDepthLoadOp::Clear(0.0)
    }
}

impl From<Camera3dDepthLoadOp> for LoadOp<f32> {
    fn from(config: Camera3dDepthLoadOp) -> Self {
        match config {
            Camera3dDepthLoadOp::Clear(x) => LoadOp::Clear(x),
            Camera3dDepthLoadOp::Load => LoadOp::Load,
        }
    }
}

/// Configure the tonemap mode.
#[derive(Component, Default, Copy, Clone, Reflect, PartialEq, Eq, Hash, Debug)]
#[reflect(Component, Default, PartialEq, Hash, Debug)]
pub enum TonemapMode {
    /// Tonemapping will be applied in the fragment shader of each material,
    /// which allows rendering HDR colors to LDR format color target,
    /// but the result of transparency blending may not be physically correct.
    #[default]
    InShader,
    /// Tonemapping will be done as a post process render pass.
    /// The color target should have sufficient dynamic range (such as `Rgba16Float`, `Rg11b10Ufloat`),
    /// otherwise HDR colors will be clipped during main pass.
    PostProcess,
}

/// Color space for alpha compositing. Affects how overlapping semi-transparent layers blend.
#[derive(Component, Copy, Clone, Reflect, PartialEq, Eq, Hash, Debug, Default)]
#[reflect(Component, PartialEq, Hash, Debug, Default)]
pub enum CompositingSpace {
    /// Gamma-encoded blending. Matches most image editors. Uses default sRGB target.
    #[default]
    Srgb,
    /// Linear light blending. Physically correct.
    Linear,
    /// Perceptually uniform blending. Often smoother gradients. Requires [`Hdr`] because its value can be outside [0, 1].
    Oklab,
}

/// The intermediate color target texture (not the [`crate::RenderTarget`]) that can be used for cameras.
#[derive(Component, Clone, Reflect, PartialEq, Eq, Hash, Debug)]
#[reflect(Component, PartialEq, Hash, Debug, Default)]
pub struct ColorTarget {
    /// The label prefix of the texture.
    pub label: Option<alloc::borrow::Cow<'static, str>>,
    /// Size of the texture.
    pub size: UVec2,
    /// Sample count of the multisampled texture if this is larger than 1.
    pub sample_count: u32,
    /// Format of the texture.
    pub format: TextureFormat,
    /// Allowed usages of the texture.
    pub usage: TextureUsages,
    /// Specifies what view formats will be allowed when creating texture view on this texture.
    /// View formats of the same format as the texture are always allowed.
    /// Note: currently, only the srgb-ness is allowed to change.
    pub view_formats: SmallVec<[TextureFormat; 1]>,
}

impl Default for ColorTarget {
    fn default() -> Self {
        Self {
            label: Some("main_texture".into()),
            size: UVec2::new(1280, 720),
            sample_count: 4,
            format: TextureFormat::Rgba8UnormSrgb,
            usage: TextureUsages::RENDER_ATTACHMENT
                | TextureUsages::TEXTURE_BINDING
                | TextureUsages::COPY_SRC,
            view_formats: SmallVec::new(),
        }
    }
}

/// The color target used by this camera.
#[derive(Component, Clone, Reflect, PartialEq, Eq, Hash, Debug)]
#[reflect(Component, PartialEq, Hash, Debug, Default)]
pub enum CameraColorTarget {
    Owned(ColorTarget),
    Reference(Entity),
}

impl Default for CameraColorTarget {
    fn default() -> Self {
        Self::Owned(ColorTarget::default())
    }
}

/// Configure the texture view of the color target used by this camera.
#[derive(Component, Copy, Clone, Reflect, PartialEq, Eq, Hash, Debug, Default)]
#[reflect(Component, PartialEq, Hash, Debug, Default)]
pub struct CameraColorTargetTextureView {
    /// The format of the texture view. If None the view format will be equal to texture format.
    pub format: Option<TextureFormat>,
}
