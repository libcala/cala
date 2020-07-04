//! **feature:draw** - Render (draw) graphics using the GPU and/or SIMD.
//!
//! # Getting Started
//! This API is designed to be high-level without sacrificing optimization.
//! Graphics are complicated though, so before you start, a few things need
//! to be defined.
//!
//! ## Shader
//! A Shader is a program that runs on the GPU for the purpose of drawing
//! Shapes.  When you make your program, start by creating a shader.
//! Shaders are built at compile time, so you'll need to make a build.rs and
//! depend on the [`res`](https://crates.io/crates/res) crate.  Calling
//! `generate()` in your build.rs will generate your shaders.
//!
//! ## Shape
//! A shape is a collection of vertices that when connected make a 2D or 3D
//! shape.  Shapes can only be used with one Shader because they may have
//! shader-specific additional information attached to them like color or
//! graphic coordinates.
//!
//! ## Group
//! Shapes themselves can't be drawn, first you must make put them into a Group.
//! When putting a shape into a group you may attach a transform and optionally
//! texture coordinates to it.
//!
//! # Example
//! ```rust
//! // TODO
//! ```

use std::{
    cell::RefCell,
    mem::MaybeUninit,
    sync::{
        atomic::{AtomicU32, Ordering},
        Arc, Condvar, Mutex, Once,
    },
    task::{Waker},
};

/// A 2D rectangular image.
///
/// ---
pub use pix::Raster;
/// Location and dimensions of a rectangular region within a `Raster`.
///
/// ---
pub use pix::Region;

/// Color Models and types.
///
/// These are re-exported from the `pix` crate.
pub mod color {
    pub use pix::bgr::*;
    pub use pix::rgb::*;
    pub use pix::cmy::*;
    pub use pix::ycc::*;
    pub use pix::hsl::*;
    pub use pix::hsv::*;
    pub use pix::hwb::*;
    pub use pix::gray::*;
    pub use pix::matte::*;
}

pub(super) enum GpuCmd {
    /// Set the background color on the GPU output raster.
    Background(f32, f32, f32),
    Draw(u32, u32),
    DrawGraphic(u32, u32, u32),
    SetCamera(Transform),
    SetTint(u32, [f32; 4]),
    RasterId(pix::Raster<pix::rgb::SRgba8>, u32),
    ShaderId(ShaderBuilder, u32),
    ShapeId(ShapeBuilder, u32, u32),
    GroupId(u32),
    GroupPush(u32, u32, Transform),
    GroupPushTex(u32, u32, Transform, ([f32; 2], [f32; 2])),
}

pub(super) struct FrameInternal {
    pub(super) waker: Option<Waker>,
    pub(super) frame: Option<(std::time::Duration, f32)>,
}

pub(super) struct Internal {
    pub(super) cmds: Mutex<Vec<GpuCmd>>,
    pub(super) frame: Mutex<FrameInternal>,
    pub(super) pair: Arc<(Mutex<bool>, Condvar)>,
    raster_garbage: Mutex<Vec<u32>>,
    rasters: RefCell<Vec<window::RasterId>>,
    shader_garbage: Mutex<Vec<u32>>,
    shaders: RefCell<Vec<window::Shader>>,
    shape_garbage: Mutex<Vec<u32>>,
    shapes: RefCell<Vec<window::Shape>>,
    group_garbage: Mutex<Vec<u32>>,
    groups: RefCell<Vec<window::Group>>,
}
static mut INTERNAL: MaybeUninit<Internal> = MaybeUninit::uninit();
static INIT: Once = Once::new();
static NEXT_RASTER_ID: AtomicU32 = AtomicU32::new(0);
static NEXT_SHADER_ID: AtomicU32 = AtomicU32::new(0);
static NEXT_SHAPE_ID: AtomicU32 = AtomicU32::new(0);
static NEXT_GROUP_ID: AtomicU32 = AtomicU32::new(0);

impl Internal {
    // Get internal graphics data, lazily initializing if not used yet.
    pub(super) fn new_lazy() -> &'static Self {
        // It's in the Condvar docs, so this is the recommended way to do it.
        #[allow(clippy::mutex_atomic)]
        unsafe {
            INIT.call_once(|| {
                INTERNAL = MaybeUninit::new(Internal {
                    cmds: Mutex::new(Vec::new()),
                    frame: Mutex::new(FrameInternal {
                        waker: None,
                        frame: None,
                    }),
                    pair: Arc::new((Mutex::new(false), Condvar::new())),
                    raster_garbage: Mutex::new(Vec::new()),
                    rasters: RefCell::new(Vec::new()),
                    shader_garbage: Mutex::new(Vec::new()),
                    shaders: RefCell::new(Vec::new()),
                    shape_garbage: Mutex::new(Vec::new()),
                    shapes: RefCell::new(Vec::new()),
                    group_garbage: Mutex::new(Vec::new()),
                    groups: RefCell::new(Vec::new()),
                });
            });
            &*INTERNAL.as_ptr()
        }
    }
}

/// `Raster` stored on the GPU.
pub struct Texture(pub(super) u32);

impl Texture {
    /// Create a `Texture` by copying a `Raster` to the GPU.
    pub fn new<P: pix::el::Pixel>(raster: &pix::Raster<P>) -> Self
    where
        pix::chan::Ch8: From<<P as pix::el::Pixel>::Chan>,
    {
        let internal = Internal::new_lazy();
        let id = if let Some(id) = internal.raster_garbage.lock().unwrap().pop() {
            id
        } else {
            NEXT_RASTER_ID.fetch_add(1, Ordering::Relaxed)
        };
        let raster = pix::Raster::<pix::rgb::SRgba8>::with_raster(&raster);
        let mut lock = internal.cmds.lock().unwrap();
        lock.push(GpuCmd::RasterId(
            raster,
            id,
        ));
        Texture(id)
    }
}

impl Drop for Texture {
    fn drop(&mut self) {
        // FIXME: Make GpuCmd
        let internal = Internal::new_lazy();
        internal.raster_garbage.lock().unwrap().push(self.0);
    }
}

/// A Shader.
pub struct Shader(pub(super) u32);

impl Shader {
    /// Copy and send a shader program to the GPU.
    pub fn new(builder: ShaderBuilder) -> Shader {
        let internal = Internal::new_lazy();
        let id = if let Some(id) = internal.shader_garbage.lock().unwrap().pop() {
            id
        } else {
            NEXT_SHADER_ID.fetch_add(1, Ordering::Relaxed)
        };
        let mut lock = internal.cmds.lock().unwrap();
        lock.push(GpuCmd::ShaderId(builder, id));
        Shader(id)
    }
}

impl Drop for Shader {
    fn drop(&mut self) {
        // FIXME: Make GpuCmd
        let internal = Internal::new_lazy();
        internal.shader_garbage.lock().unwrap().push(self.0);
    }
}

/// A Shape.
pub struct Shape(u32);

impl Drop for Shape {
    fn drop(&mut self) {
        // FIXME: Make GpuCmd
        let internal = Internal::new_lazy();
        internal.shape_garbage.lock().unwrap().push(self.0);
    }
}

/// A Group.
pub struct Group(pub(crate) u32);

impl Group {
    /// Create a new Group of Shapes.
    pub fn new() -> Self {
        let internal = Internal::new_lazy();
        let id = if let Some(id) = internal.group_garbage.lock().unwrap().pop() {
            id
        } else {
            NEXT_GROUP_ID.fetch_add(1, Ordering::Relaxed)
        };
        let mut lock = internal.cmds.lock().unwrap();
        lock.push(GpuCmd::GroupId(id));
        Group(id)
    }
    
    /// Push a shape into the group.
    pub fn push(&mut self, shape: &Shape, transform: &Transform) {
        let internal = Internal::new_lazy();
        let mut lock = internal.cmds.lock().unwrap();
        lock.push(GpuCmd::GroupPush(self.0, shape.0, transform.clone()));
    }

    /// Push a shape into the group.
    pub fn push_tex(
        &mut self,
        shape: &Shape,
        transform: &Transform,
        tex_coords: ([f32; 2], [f32; 2])
    )
    {
        let internal = Internal::new_lazy();
        let mut lock = internal.cmds.lock().unwrap();
        lock.push(GpuCmd::GroupPushTex(self.0, shape.0, transform.clone(), tex_coords));
    }
}

impl Drop for Group {
    fn drop(&mut self) {
        // FIXME: Make GpuCmd
        let internal = Internal::new_lazy();
        internal.group_garbage.lock().unwrap().push(self.0);
    }
}

// A function that is run on the graphics thread whenever
fn async_runner(window: &mut window::Window, elapsed: std::time::Duration) {
    // Get the aspcet ratio
    let aspect = window.aspect();

    // Reset condvar
    let pair = {
        let internal = Internal::new_lazy();
        internal.pair.clone()
    };
    let (lock, cvar) = &*pair;
    *lock.lock().unwrap() = false;

    // Wake async thread
    {
        let internal = Internal::new_lazy();
        let mut lock = internal.frame.lock().unwrap();
        lock.waker.take().unwrap().wake();
        lock.frame = Some((elapsed, aspect));
    }

    // Wait for async thread to finish writing to the command buffer.
    let mut started = lock.lock().unwrap();
    while !*started {
        started = cvar.wait(started).unwrap();
    }

    // Process commands in the command buffer.
    let mut lock = Internal::new_lazy().cmds.lock().unwrap();
    for cmd in lock.drain(..) {
        use GpuCmd::*;
        match cmd {
            Background(r, g, b) => window.background(r, g, b),
            Draw(shader, group) => {
                let shaders = Internal::new_lazy().shaders.borrow();
                let groups = Internal::new_lazy().groups.borrow();
                window.draw(&shaders[shader as usize], &groups[group as usize]);
            }
            DrawGraphic(shader, group, raster) => {
                let shaders = Internal::new_lazy().shaders.borrow();
                let groups = Internal::new_lazy().groups.borrow();
                window.draw_graphic(
                    &shaders[shader as usize],
                    &groups[group as usize],
                    &Internal::new_lazy().rasters.borrow()[raster as usize],
                );
            }
            SetCamera(camera) => {
                window.camera(camera);
            }
            SetTint(shader, tint) => {
                let shaders = Internal::new_lazy().shaders.borrow();
                window.tint(&shaders[shader as usize], tint);
            }
            RasterId(raster, id) => {
                let gpu_raster = window.graphic(
                    raster.as_u8_slice(),
                    raster.width() as usize,
                    raster.height() as usize,
                );
                let mut rasters = Internal::new_lazy().rasters.borrow_mut();
                if id as usize == rasters.len() {
                    rasters.push(gpu_raster);
                } else {
                    rasters[id as usize] = gpu_raster;
                }
            }
            ShaderId(shader, id) => {
                let shader = window.shader_new(shader);
                let mut shaders = Internal::new_lazy().shaders.borrow_mut();
                if id as usize == shaders.len() {
                    shaders.push(shader);
                } else {
                    shaders[id as usize] = shader;
                }
            }
            ShapeId(shape_builder, id, shader) => {
                let mut shapes = Internal::new_lazy().shapes.borrow_mut();
                let mut shaders = Internal::new_lazy().shaders.borrow_mut();
                let mut shape =
                    window::ShapeBuilder::new(&mut shaders[shader as usize]);
                for face in shape_builder.faces {
                    if let Some(vertices) = face.vertices {
                        shape = shape.vert(vertices.as_slice());
                    }
                    if let Some(transform) = face.transform {
                        shape = shape.face(transform);
                    }
                }
                if id as usize == shapes.len() {
                    shapes.push(shape.finish());
                } else {
                    shapes[id as usize] = shape.finish();
                }
            }
            GroupId(id) => {
                let mut groups = Internal::new_lazy().groups.borrow_mut();
                if id as usize == groups.len() {
                    groups.push(window.group_new());
                } else {
                    groups[id as usize] = window.group_new();
                }
            }
            GroupPush(group, shape, transform) => {
                let mut groups = Internal::new_lazy().groups.borrow_mut();
                let shapes = Internal::new_lazy().shapes.borrow();
                groups[group as usize].push(&shapes[shape as usize], &transform);
            }
            GroupPushTex(group, shape, transform, texcoords) => {
                let mut groups = Internal::new_lazy().groups.borrow_mut();
                let shapes = Internal::new_lazy().shapes.borrow();
                groups[group as usize].push_tex(&shapes[shape as usize], &transform, texcoords);
            }
        }
    }
}

pub(crate) mod __hidden {
    use window::Window;

    #[doc(hidden)]
    pub fn draw_thread() {
        let fallback_window_title = env!("CARGO_PKG_NAME");
        let mut window =
            Window::new(fallback_window_title, super::async_runner);
        loop {
            window.run();
        }
    }
}

pub use window::{shader, Transform, ShaderBuilder};

// // // // // //

struct Face {
    vertices: Option<Vec<f32>>,
    transform: Option<Transform>,
}

/// Builder for a shape.
pub struct ShapeBuilder {
    faces: Vec<Face>,
}

impl Default for ShapeBuilder {
    fn default() -> Self {
        ShapeBuilder::new()
    }
}

impl ShapeBuilder {
    /// Create a new `ShapeBuilder`.
    pub fn new() -> Self {
        ShapeBuilder { faces: Vec::new() }
    }

    /// Set vertices for the following faces.
    pub fn vert(mut self, vertices: &[f32]) -> Self {
        self.faces.push(Face {
            vertices: Some(vertices.to_vec()),
            transform: None,
        });
        self
    }

    /// Add a face to the shape using the last set vertices.
    pub fn face(mut self, transform: Transform) -> Self {
        if let Some(mut face) = self.faces.pop() {
            if face.transform.is_none() {
                face.transform = Some(transform);
                self.faces.push(face);
            } else {
                self.faces.push(face);
                self.faces.push(Face {
                    vertices: None,
                    transform: Some(transform),
                });
            }
        } else {
            panic!("Can't have a face without vertices!");
        }
        self
    }

    /// Finish building the shape.
    pub fn finish(self, shader: &Shader) -> Shape {
        let internal = Internal::new_lazy();
        let id = if let Some(id) = internal.shape_garbage.lock().unwrap().pop()
        {
            id
        } else {
            NEXT_SHAPE_ID.fetch_add(1, Ordering::Relaxed)
        };
        let mut lock = internal.cmds.lock().unwrap();
        lock.push(GpuCmd::ShapeId(self, id, shader.0));
        Shape(id)
    }
}
