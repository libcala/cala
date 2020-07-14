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
//! # Coordinate System
//! ![X goes from 0 to 1, Y goes from 0 to `Canvas.height()`](https://raw.githubusercontent.com/libcala/window/5205e59f0cd9f37a619f590e94218900afc2395b/res/coordinate_system.svg)
//!
//! # Example
//! Open a window with a triangle that rotates once a second:
//! ```rust
//! use cala::*;
//!
//! use cala::draw::{
//!     color::SRgb32, shader, Group, Shader, ShaderBuilder, ShapeBuilder,
//!     Transform,
//! };
//! use cala::input::{GameInput, Input, TextInput, UiInput};
//!
//! pub struct Context {
//!     colors: Shader,
//!     triangle: Group,
//!     timed: f64,
//! }
//!
//! // Initialize & set loop to `init()`.
//! cala::exec!(init);
//!
//! async fn init() {
//!     let timed = 0.0;
//!     // Load a shader.
//!     let colors = Shader::new(shader!("color"));
//!
//!     // Build triangle Shape
//!     let triangle = Group::new();
//!     let mut context = Context {
//!         colors,
//!         triangle,
//!         timed,
//!     };
//!
//!     // Game loop
//!     while [canvas(&mut context).fut(), input().fut()].select().await.1 {}
//! }
//!
//! fn animate_triangle(context: &mut Context, time: f32, aspect: f32) {
//!     #[rustfmt::skip]
//!     let vertices = [
//!          -1.0,  1.0,  1.0, 0.5, 0.0,
//!           1.0,  1.0,  0.0, 0.0, 1.0,
//!           0.0, -1.0,  1.0, 1.0, 1.0,
//!
//!           0.0, -1.0,  1.0, 0.7, 0.0,
//!           1.0,  1.0,  1.0, 0.7, 0.0,
//!          -1.0,  1.0,  1.0, 0.7, 0.0,
//!     ];
//!
//!     let triangle_shape = ShapeBuilder::new()
//!         .vert(&vertices)
//!         .face(Transform::new())
//!         .finish(&context.colors);
//!     let transform = Transform::new()
//!         .rotate(0.0, 1.0, 0.0, time)
//!         .scale(0.25, 0.25 * aspect, 0.25)
//!         .translate(0.5, 0.5 * aspect, 0.0);
//!     context.triangle.write(0, &triangle_shape, &transform);
//! }
//!
//! // Function that runs while your app runs.
//! pub async fn canvas(context: &mut Context) -> bool {
//!     // Set the background color.
//!     let mut canvas = pixels::canvas(SRgb32::new(0.0, 0.5, 0.0)).await;
//!
//!     // Update triangle
//!     context.timed = (context.timed + canvas.elapsed().as_secs_f64()) % 1.0;
//!     animate_triangle(context, context.timed as f32, canvas.aspect());
//!
//!     // Draw triangle
//!     canvas.draw(&context.colors, &context.triangle);
//!
//!     true
//! }
//!
//! async fn input<'a>() -> bool {
//!     match cala::input::input().await {
//!         Input::Ui(UiInput::Back) => return false,
//!         Input::Game(_id, GameInput::Back) => return false,
//!         Input::Text(TextInput::Back) => return false,
//!         input => println!("{:?}", input),
//!     }
//!     true
//! }
//! ```

use std::{
    cell::RefCell,
    mem::MaybeUninit,
    sync::{
        atomic::{AtomicU32, Ordering},
        Arc, Condvar, Mutex, Once,
    },
    task::Waker,
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
    pub use pix::cmy::*;
    pub use pix::gray::*;
    pub use pix::hsl::*;
    pub use pix::hsv::*;
    pub use pix::hwb::*;
    pub use pix::matte::*;
    pub use pix::rgb::*;
    pub use pix::ycc::*;
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
    GroupWrite(u32, u32, u32, Transform),
    GroupWriteTex(u32, u32, u32, Transform, ([f32; 2], [f32; 2])),
}

pub(super) struct FrameInternal {
    pub(super) waker: Option<Waker>,
    pub(super) frame: Option<(std::time::Duration, f32, bool)>,
}

type Location = Vec<(usize, usize)>;

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
    groups: RefCell<Vec<(window::Group, Location)>>,
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
        let id = if let Some(id) = internal.raster_garbage.lock().unwrap().pop()
        {
            id
        } else {
            NEXT_RASTER_ID.fetch_add(1, Ordering::Relaxed)
        };
        let raster = pix::Raster::<pix::rgb::SRgba8>::with_raster(&raster);
        let mut lock = internal.cmds.lock().unwrap();
        lock.push(GpuCmd::RasterId(raster, id));
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
        let id = if let Some(id) = internal.shader_garbage.lock().unwrap().pop()
        {
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

impl Default for Group {
    fn default() -> Self {
        Group::new()
    }
}

impl Group {
    /// Create a new Group of Shapes.
    pub fn new() -> Self {
        let internal = Internal::new_lazy();
        let id = if let Some(id) = internal.group_garbage.lock().unwrap().pop()
        {
            id
        } else {
            NEXT_GROUP_ID.fetch_add(1, Ordering::Relaxed)
        };
        let mut lock = internal.cmds.lock().unwrap();
        lock.push(GpuCmd::GroupId(id));
        Group(id)
    }

    /// Push a shape into the group.
    pub fn write(&mut self, id: u32, shape: &Shape, transform: &Transform) {
        let internal = Internal::new_lazy();
        let mut lock = internal.cmds.lock().unwrap();
        lock.push(GpuCmd::GroupWrite(self.0, id, shape.0, *transform));
    }

    /// Push a shape into the group.
    pub fn write_tex(
        &mut self,
        id: u32,
        shape: &Shape,
        transform: &Transform,
        tex_coords: ([f32; 2], [f32; 2]),
    ) {
        let internal = Internal::new_lazy();
        let mut lock = internal.cmds.lock().unwrap();
        lock.push(GpuCmd::GroupWriteTex(
            self.0, id, shape.0, *transform, tex_coords,
        ));
    }
}

impl Drop for Group {
    fn drop(&mut self) {
        // FIXME: Make GpuCmd
        let internal = Internal::new_lazy();
        internal.group_garbage.lock().unwrap().push(self.0);
    }
}

static ASPECT: AtomicU32 = AtomicU32::new(0);

// A function that is run on the graphics thread whenever
fn async_runner(window: &mut window::Window, elapsed: std::time::Duration) {
    // Get the aspect ratio
    let aspect = window.aspect();
    // Check if the window has been resized.
    let new_aspect = u32::from_ne_bytes(aspect.to_ne_bytes());
    let old_aspect = ASPECT.swap(new_aspect, Ordering::Relaxed);
    let resized = new_aspect != old_aspect;

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
        lock.frame = Some((elapsed, aspect, resized));
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
                window
                    .draw(&shaders[shader as usize], &groups[group as usize].0);
            }
            DrawGraphic(shader, group, raster) => {
                let shaders = Internal::new_lazy().shaders.borrow();
                let groups = Internal::new_lazy().groups.borrow();
                window.draw_graphic(
                    &shaders[shader as usize],
                    &groups[group as usize].0,
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
                    groups.push((window.group_new(), Vec::new()));
                } else {
                    groups[id as usize] = (window.group_new(), Vec::new());
                }
            }
            GroupWrite(group, id, shape, transform) => {
                let mut groups = Internal::new_lazy().groups.borrow_mut();
                let shapes = Internal::new_lazy().shapes.borrow();
                if id >= groups[group as usize].1.len() as u32 {
                    let location = if id == 0 {
                        (0, 0)
                    } else {
                        groups[group as usize].1[id as usize - 1]
                    };
                    let location = groups[group as usize].0.write(
                        location,
                        &shapes[shape as usize],
                        &transform,
                    );
                    groups[group as usize].1.push(location);
                } else {
                    let location = if id == 0 {
                        (0, 0)
                    } else {
                        groups[group as usize].1[id as usize - 1]
                    };
                    groups[group as usize].1[id as usize] = groups
                        [group as usize]
                        .0
                        .write(location, &shapes[shape as usize], &transform);
                }
            }
            GroupWriteTex(group, id, shape, transform, texcoords) => {
                let mut groups = Internal::new_lazy().groups.borrow_mut();
                let shapes = Internal::new_lazy().shapes.borrow();
                if id >= groups[group as usize].1.len() as u32 {
                    let location = if id == 0 {
                        (0, 0)
                    } else {
                        groups[group as usize].1[id as usize - 1]
                    };
                    let location = groups[group as usize].0.write_tex(
                        location,
                        &shapes[shape as usize],
                        &transform,
                        texcoords,
                    );
                    groups[group as usize].1.push(location);
                } else {
                    let location = if id == 0 {
                        (0, 0)
                    } else {
                        groups[group as usize].1[id as usize - 1]
                    };
                    groups[group as usize].1[id as usize] =
                        groups[group as usize].0.write_tex(
                            location,
                            &shapes[shape as usize],
                            &transform,
                            texcoords,
                        );
                }
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

pub use window::{shader, ShaderBuilder, Transform};

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
