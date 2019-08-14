// use std::sync::mpsc::{channel, Sender, Receiver};

#[doc(hidden)]
pub use window::{ShaderBuilder};

pub use window::{ShapeBuilder, Transform, Graphic, Key};

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
    window: Box<window::Window>,
    shader: Vec<Option<window::Shader>>,
    shadet: Vec<usize>,
    shapes: Vec<Option<window::Shape>>,
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

fn toolbar(buffer: &mut [u8], width: u16) {
    let height = buffer.len() / (4 * width as usize);
    let size = (width, height as u16);
    let mut p = fonterator::footile::Plotter::new(size.0 as u32, size.1 as u32);
    let mut image = fonterator::footile::RasterB::new(p.width(), p.height());

    use fonterator::PathOp::*;
    use fonterator::footile::PixFmt;

    // Render Background.
    let shape = [
        Move(0.0, 0.0),
        Line(width.into(), 0.0),
        Line(width.into(), height as f32),
        Line(0.0, height as f32),
    ];
    let mut pix = fonterator::footile::Rgba8::as_slice_mut(buffer);
    image.over(p.fill(&shape, fonterator::footile::FillRule::EvenOdd), fonterator::footile::Rgba8::rgb(52, 32, 64), pix /**/);
    // 
    let length = buffer.len() / 4;
    let pointer = buffer as *mut _ as *mut _;
    let slice = unsafe { std::slice::from_raw_parts_mut(pointer, length) };

    crate::icons::menu(slice, 0, width, height as u16);
    crate::icons::zoom_out(slice, 1, width, height as u16);
    crate::icons::zoom_in(slice, 3, width, height as u16);
    crate::icons::view(slice, 5, width, height as u16);
    crate::icons::search(slice, 7, width, height as u16);
    crate::icons::fullscreen(slice, 9, width, height as u16);
    crate::icons::grid(slice, 11, width, height as u16);
    crate::icons::next(slice, 13, width, height as u16);
    crate::icons::text(slice, width, height as u16, "Plop Grizzlyhna2");
}

// Initialize graphic shader.
pub fn init_toolbar(window: &mut window::Window) -> (window::Shader, window::Shape) {
    let mut gui = window.shader_new(window::shader!("gui"));

    // Define vertices.
    #[rustfmt::skip]
    let vertices = [
        -1.0, -1.0,  0.0, 1.0,
         1.0, -1.0,  1.0, 1.0,
         1.0,  1.0,  1.0, 0.0,

        -1.0,  1.0,  0.0, 0.0,
        -1.0, -1.0,  0.0, 1.0,
         1.0,  1.0,  1.0, 0.0,
    ];

    // Build cube Shape
    let mut rect = window.shape_new(ShapeBuilder::new(&mut gui).vert(&vertices).face(Transform::new()));
    window.instances(&mut rect, &[Transform::new()]);
    window.build(&mut gui);

    (gui, rect)
}

pub(crate) fn initialize_video_io(name: &str, run: fn(nanos: u64) -> ()) {
    use window::*;

    unsafe {
        let video_io = &mut VIDEO_IO as *mut _ as *mut VideoIO;
        let shader = vec![];
        let shadet = vec![];
        let shapes = vec![];
        let shapet = vec![];

        let mut window = Window::new(name, run, init_toolbar);
        window.toolbar(toolbar);

        std::ptr::write(video_io, VideoIO {
            window, shader, shadet, shapes, shapet,
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

/// Set tinit for shader.
pub fn set_tint(shader: &Shader, tint: [f32; 4]) {
    let video_io = unsafe { &mut VIDEO_IO as *mut _ as *mut VideoIO };

    unsafe {
        (*video_io).window.tint((*video_io).shader[shader.0].as_ref().unwrap(), tint);
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
pub fn graphic(pixels: &[u8], width: usize, height: usize) -> Graphic {
    let video_io = unsafe { &mut VIDEO_IO as *mut _ as *mut VideoIO };

    unsafe {
        (*video_io).window.graphic(pixels, width, height)
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
