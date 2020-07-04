//! Automatically import traits with `use cala::prelude::*;`.

#[cfg(feature = "draw")]
/// **feature:draw** - 
pub use pix::{ops::Blend as PixelBlend, el::Pixel};

#[cfg(feature = "draw")]
/// **feature:draw** - Something that can be drawn on.
pub trait Canvas {
    /// Draw a group on the screen.
    fn draw(&mut self, shader: &crate::draw::Shader, group: &crate::draw::Group);
    /// Set camera for shader.
    fn set_camera(&mut self, camera: crate::draw::Transform);
    /// Set tint for shader.
    fn set_tint<P: pix::el::Pixel>(&mut self, shader: &crate::draw::Shader, tint: P)
    where
        pix::chan::Ch32: From<<P as pix::el::Pixel>::Chan>;
    /// Draw a group with a texture on the screen.
    fn draw_graphic(
        &mut self,
        shader: &crate::draw::Shader,
        group: &crate::draw::Group,
        graphic: &crate::draw::Texture,
    );
    /// Returns the amount of time elapsed since the previous frame. 
    fn elapsed(&self) -> std::time::Duration;
    /// Return the aspect ratio of the `Canvas`.
    fn aspect(&self) -> f32;
}

#[cfg(feature = "pasts")]
/// **feature:pasts** - 
pub use pasts::{DynFut as IntoDynFut, Join as JoinFut, Select as SelectFut};

#[cfg(feature = "pasts")]
/// **feature:pasts** - Trait for spawning tasks in a thread pool to run
/// closures as a `Future`.
pub trait SpawnBlocking<T> {
    /// Turn closure into a future.
    fn spawn_blocking(self) -> Box<dyn std::future::Future<Output = T>>;
}

#[cfg(feature = "pasts")]
impl<T, F> SpawnBlocking<T> for F
where
    F: FnOnce() -> T,
    F: Send + 'static,
    T: Send + 'static,
{
    fn spawn_blocking(self) -> Box<dyn std::future::Future<Output = T>> {
        Box::new(pasts::spawn_blocking(self))
    }
}
