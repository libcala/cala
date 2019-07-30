// use std::sync::mpsc::{channel, Sender, Receiver};

#[doc(hidden)]
pub use barg::{ShaderBuilder};

pub use barg::{ShapeBuilder, Transform, Graphic, Key};

/// **video** Load a generated shader from `res`.
#[macro_export(self)] macro_rules! shader {
    ($shadername: literal) => {
        $crate::Shader::new(include!(concat!(env!("OUT_DIR"), "/res/", $shadername, ".rs")));
    }
}

/// A shader.  Shaders are programs that run on the GPU that render things on
/// the screen.
pub struct Shader(usize);
/// A shape.  Shapes are a list of indices into a `VertexList`.
pub struct Shape(usize);

struct VideoIO {
    window: Box<barg::Window>,
    shader: Vec<Option<barg::Shader>>,
    shadet: Vec<usize>,
    shapes: Vec<Option<barg::Shape>>,
    shapet: Vec<usize>,
}

static mut VIDEO_IO: FakeVideoIO = FakeVideoIO([0; std::mem::size_of::<VideoIO>()]);

#[repr(align(8))]
struct FakeVideoIO([u8; std::mem::size_of::<VideoIO>()]);

// // // // // //

impl Drop for Shader {
    fn drop(&mut self) {
        let video_io = unsafe { &mut VIDEO_IO as *mut _ as *mut VideoIO };

        unsafe {
            (*video_io).shadet.push(self.0);
            (*video_io).shader[self.0] = None;
        }
    }
}

impl Drop for Shape {
    fn drop(&mut self) {
        let video_io = unsafe { &mut VIDEO_IO as *mut _ as *mut VideoIO };

        unsafe {
            (*video_io).shapet.push(self.0);
            (*video_io).shapes[self.0] = None;
        }
    }
}

impl Shader {
    /// Build a shader.
    #[doc(hidden)]
    pub fn new(builder: ShaderBuilder) -> Shader {
        let video_io = unsafe { &mut VIDEO_IO as *mut _ as *mut VideoIO };

        let index = unsafe {
            let index = if let Some(index) = (*video_io).shadet.pop() {
                index
            } else {
                (*video_io).shader.len()
            };

            let shader = (*video_io).window.shader_new(builder);

            if index == (*video_io).shader.len() {
                (*video_io).shader.push(Some(shader));
            } else {
                (*video_io).shader[index] = Some(shader);
            }

            index
        };

        Shader(index)
    }
}

impl Shape {
    /// Build a shape.
    pub fn new<'a>(builder: ShapeBuilder<'a>) -> Shape {
        let video_io = unsafe { &mut VIDEO_IO as *mut _ as *mut VideoIO };

        let index = unsafe {
            let index = if let Some(index) = (*video_io).shapet.pop() {
                index
            } else {
                (*video_io).shapes.len()
            };

                    let shape = (*video_io).window.shape_new(builder);

                    if index == (*video_io).shapes.len() {
                        (*video_io).shapes.push(Some(shape));
                    } else {
                        (*video_io).shapes[index] = Some(shape);
                    }

            index
        };

        Shape(index)
    }

    /// Make instances.
    pub fn instances(&mut self, transforms: &[Transform]) {
        let video_io = unsafe { &mut VIDEO_IO as *mut _ as *mut VideoIO };

        unsafe {
            (*video_io).window.instances((*video_io).shapes[self.0].as_mut().unwrap(), transforms);
        }
    }

    /// Set transformation for an instance.
    pub fn transform(&mut self, index: u16, transform: Transform) {
        let video_io = unsafe { &mut VIDEO_IO as *mut _ as *mut VideoIO };

        unsafe {
            (*video_io).window.transform((*video_io).shapes[self.0].as_mut().unwrap(), index, transform);
        }
    }
}

pub(crate) fn initialize_video_io(name: &str, run: fn(nanos: u64) -> ()) {
    use barg::*;

    unsafe {
        let video_io = &mut VIDEO_IO as *mut _ as *mut VideoIO;
        let shader = vec![];
        let shadet = vec![];
        let shapes = vec![];
        let shapet = vec![];

        std::ptr::write(video_io, VideoIO {
            window: Window::new(name, run, init_toolbar),
            shader, shadet, shapes, shapet,
        });
    }
}

pub(crate) fn loop_video_io() -> bool {
    let video_io = unsafe { &mut VIDEO_IO as *mut _ as *mut VideoIO };

    unsafe {
        (*video_io).window.run()
    }
}

/// Set the display's background color.
pub fn background(r: f32, g: f32, b: f32) {
    let video_io = unsafe { &mut VIDEO_IO as *mut _ as *mut VideoIO };

    unsafe {
        (*video_io).window.background(r, g, b);
    }
}

/// Get a `ShapeBuilder` for a `Shader`.
pub fn shape(shader: &Shader) -> ShapeBuilder {
    let video_io = unsafe { &mut VIDEO_IO as *mut _ as *mut VideoIO };

    let shader = unsafe {
        (*video_io).shader[shader.0].as_mut().unwrap()
    };

    ShapeBuilder::new(shader)
}

/// Draw multiple instances of shapes on the screen.
pub fn draw(shader: &Shader, shape: &Shape) {
    let video_io = unsafe { &mut VIDEO_IO as *mut _ as *mut VideoIO };

    unsafe {
        (*video_io).window.draw((*video_io).shader[shader.0].as_ref().unwrap(), (*video_io).shapes[shape.0].as_ref().unwrap());
    }
}

/// Set camera for shader.
pub fn set_camera(shader: &Shader, camera: Transform) {
    let video_io = unsafe { &mut VIDEO_IO as *mut _ as *mut VideoIO };

    unsafe {
        (*video_io).window.camera((*video_io).shader[shader.0].as_ref().unwrap(), camera);
    }
}

/// Set texture coordinates for shader.
pub fn texture_coords(shader: &Shader, coords: ([f32; 2], [f32; 2])) {
    let video_io = unsafe { &mut VIDEO_IO as *mut _ as *mut VideoIO };

    unsafe {
        (*video_io).window.texture_coords((*video_io).shader[shader.0].as_ref().unwrap(), coords);
    }
}

/// Draw multiple instances of shapes on the screen.
pub fn draw_graphic(shader: &Shader, shape: &Shape, graphic: &Graphic) {
    let video_io = unsafe { &mut VIDEO_IO as *mut _ as *mut VideoIO };

    unsafe {
        (*video_io).window.draw_graphic((*video_io).shader[shader.0].as_ref().unwrap(), (*video_io).shapes[shape.0].as_ref().unwrap(), graphic);
    }
}

/// Load a graphic.
pub fn graphic(pixels: &[u8], width: usize) -> Graphic {
    let video_io = unsafe { &mut VIDEO_IO as *mut _ as *mut VideoIO };

    unsafe {
        (*video_io).window.graphic(pixels, width)
    }
}

/// Finish building shader.
pub fn build(shader: &Shader) {
    let video_io = unsafe { &mut VIDEO_IO as *mut _ as *mut VideoIO };

    unsafe {
        (*video_io).window.build((*video_io).shader[shader.0].as_mut().unwrap());
    }
}

/// If a key is being held down.
pub fn key(key: Key) -> bool {
    let video_io = unsafe { &mut VIDEO_IO as *mut _ as *mut VideoIO };

    unsafe {
        (*video_io).window.key(key)
    }
}
