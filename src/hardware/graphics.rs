//! **feature:graphics** - Render graphics.
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

use pix::{chan::Channel, el::Pixel};
use std::{
    cell::RefCell,
    future::Future,
    mem::MaybeUninit,
    pin::Pin,
    sync::{
        atomic::{AtomicU32, Ordering},
        Arc, Condvar, Mutex, MutexGuard, Once,
    },
    task::{Context, Poll, Waker},
};

enum GpuCmd {
    /// Set the background color on the GPU output raster.
    Background(f32, f32, f32),
    Draw(u32, Arc<Group>),
    DrawGraphic(u32, Arc<Group>, Arc<RasterId>),
    SetCamera(u32, Transform),
    SetTint(u32, [f32; 4]),
    RasterId(pix::Raster<pix::rgb::SRgba8>, u32),
    ShaderId(ShaderBuilder, u32),
    ShapeId(ShapeBuilder, u32, u32),
}

struct FrameInternal {
    waker: Option<Waker>,
    frame: Option<(f64, f64)>,
}

struct Internal {
    cmds: Mutex<Vec<GpuCmd>>,
    frame: Mutex<FrameInternal>,
    pair: Arc<(Mutex<bool>, Condvar)>,
    raster_garbage: Mutex<Vec<u32>>,
    rasters: RefCell<Vec<RasterId>>,
    shader_garbage: Mutex<Vec<u32>>,
    shaders: RefCell<Vec<window::Shader>>,
    shape_garbage: Mutex<Vec<u32>>,
    shapes: RefCell<Vec<window::Shape>>,
}
static mut INTERNAL: MaybeUninit<Internal> = MaybeUninit::uninit();
static INIT: Once = Once::new();
static NEXT_RASTER_ID: AtomicU32 = AtomicU32::new(0);
static NEXT_SHADER_ID: AtomicU32 = AtomicU32::new(0);
static NEXT_SHAPE_ID: AtomicU32 = AtomicU32::new(0);

impl Internal {
    // Get internal graphics data, lazily initializing if not used yet.
    fn new_lazy() -> &'static Self {
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
                });
            });
            &*INTERNAL.as_ptr()
        }
    }
}

struct FrameFuture;

impl Future for FrameFuture {
    type Output = (f64, f64);

    fn poll(self: Pin<&mut Self>, cx: &mut Context<'_>) -> Poll<Self::Output> {
        let internal = Internal::new_lazy();
        let mut lock = internal.frame.lock().unwrap();
        if let Some(secs) = lock.frame.take() {
            Poll::Ready(secs)
        } else {
            lock.waker = Some(cx.waker().clone());
            Poll::Pending
        }
    }
}

/// Aquire the GPU for a frame.
pub async fn frame<'a>() -> (Gpu<'a>, f64, f64) {
    let secs = FrameFuture.await;
    let internal = Internal::new_lazy();
    let cmds = internal.cmds.lock().unwrap();
    let pair = internal.pair.clone();
    let gpu = Gpu { cmds, pair };
    (gpu, secs.0, secs.1)
}

/// A raster on the GPU.
pub struct GpuRaster(u32);

impl Drop for GpuRaster {
    fn drop(&mut self) {
        // FIXME: Make GpuCmd
        let internal = Internal::new_lazy();
        internal.raster_garbage.lock().unwrap().push(self.0);
    }
}

/// A Shader.
pub struct Shader(u32);

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

/// Borrowed access to GPU instructions.
pub struct Gpu<'a> {
    // Lock on the Gpu command buffers.
    cmds: MutexGuard<'a, Vec<GpuCmd>>,
    // For when drop'd; to notify graphics thread
    pair: Arc<(Mutex<bool>, Condvar)>,
}

impl<'a> Gpu<'a> {
    /// Set the background clear color.
    pub fn background<C: pix::el::Pixel>(&mut self, color: C)
    where
        pix::chan::Ch32: From<<C as pix::el::Pixel>::Chan>,
    {
        let c: pix::rgb::SRgb32 = color.convert();
        self.cmds.push(GpuCmd::Background(
            c.one().to_f32(),
            c.two().to_f32(),
            c.three().to_f32(),
        ));
    }

    /// Draw a group on the screen.
    pub fn draw(&mut self, shader: &Arc<Shader>, group: &Arc<Group>) {
        self.cmds.push(GpuCmd::Draw(shader.0, group.clone()));
    }

    /// Set camera for shader.
    pub fn set_camera(&mut self, shader: &Arc<Shader>, camera: Transform) {
        self.cmds.push(GpuCmd::SetCamera(shader.0, camera));
    }

    /// Set tint for shader.
    pub fn set_tint(&mut self, shader: &Arc<Shader>, tint: [f32; 4]) {
        self.cmds.push(GpuCmd::SetTint(shader.0, tint));
    }

    /// Draw a group with a texture on the screen.
    pub fn draw_graphic(
        &mut self,
        shader: &Arc<Shader>,
        group: &Arc<Group>,
        graphic: &Arc<RasterId>,
    ) {
        self.cmds.push(GpuCmd::DrawGraphic(
            shader.0,
            group.clone(),
            graphic.clone(),
        ));
    }
}

impl<'a> Drop for Gpu<'a> {
    fn drop(&mut self) {
        let (lock, cvar) = &*self.pair;
        let mut started = lock.lock().unwrap();
        *started = true;
        // We notify the condvar that the value has changed.
        cvar.notify_one();
    }
}

// A function that is run on the graphics thread whenever
fn async_runner(window: &mut window::Window, nanos: f64) {
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
        lock.frame = Some((nanos, aspect.into()));
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
                window.draw(&shaders[shader as usize], &*group);
            }
            DrawGraphic(shader, group, raster) => {
                let shaders = Internal::new_lazy().shaders.borrow();
                window.draw_graphic(
                    &shaders[shader as usize],
                    &*group,
                    &raster,
                );
            }
            SetCamera(_shader, camera) => {
                // let shaders = Internal::new_lazy().shaders.borrow();
                window.camera(/*&shaders[shader as usize], */ camera);
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
        }
    }
}

pub(crate) mod __hidden {
    use window::Window;

    #[doc(hidden)]
    pub fn graphics_thread() {
        let fallback_window_title = env!("CARGO_PKG_NAME");
        let mut window =
            Window::new(fallback_window_title, super::async_runner);
        loop {
            window.run();
        }
    }
}

#[doc(hidden)]
pub use window::ShaderBuilder;

pub use window::{shader, Group, Key, RasterId, Transform};

// // // // // //

/// Set the display's background color.
pub fn background(r: f32, g: f32, b: f32) {
    let mut lock = Internal::new_lazy().cmds.lock().unwrap();
    lock.push(GpuCmd::Background(r, g, b));
}

/// Copy and send a raster to the GPU.
pub fn gpu_raster<P: pix::el::Pixel>(raster: &pix::Raster<P>) -> GpuRaster
where
    pix::chan::Ch8: From<<P as pix::el::Pixel>::Chan>,
{
    let internal = Internal::new_lazy();
    let id = if let Some(id) = internal.raster_garbage.lock().unwrap().pop() {
        id
    } else {
        NEXT_RASTER_ID.fetch_add(1, Ordering::Relaxed)
    };
    let mut lock = internal.cmds.lock().unwrap();
    lock.push(GpuCmd::RasterId(
        pix::Raster::<pix::rgb::SRgba8>::with_raster(&raster),
        id,
    ));
    GpuRaster(id)
}

/// Copy and send a shader program to the GPU.
pub fn shader(builder: ShaderBuilder) -> Shader {
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

pub use crate::timer::*;
