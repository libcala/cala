// use std::sync::mpsc::{channel, Sender, Receiver};

#[doc(hidden)]
pub use window::{ShaderBuilder};

pub use window::{ShapeBuilder, Shape, Group, Transform, Graphic, Key};

/// **video** Load a generated shader from `res`.
#[macro_export(self)] macro_rules! shader {
    ($shadername: literal) => {
        $crate::Shader::new(include!(concat!(env!("OUT_DIR"), "/res/", $shadername, ".rs")));
    }
}

/// A shader.  Shaders are programs that run on the GPU that render things on
/// the screen.
pub struct Shader(usize);

struct VideoIO {
    window: window::Window,
    shader: Vec<Option<window::Shader>>,
    shadet: Vec<usize>,
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

fn toolbar(buffer: &mut [u8], width: u16) {
/*    let height = buffer.len() / (4 * width as usize);
    let size = (width, height as u16);
    let mut p = footile::Plotter::new(u32::from(size.0), u32::from(size.1));

    use footile::PathOp::*;
    use pix::el::Pixel;

    // Render Background.
    let shape = [
        Move(0.0, 0.0),
        Line(width.into(), 0.0),
        Line(width.into(), height as f32),
        Line(0.0, height as f32),
    ];

    let mut raster = Raster::<SRgba8>::with_u8_buffer(p.width(), p.height(), buffer);

    let pix = pix::Raster::<Sfootile::Rgba8::as_slice_mut(buffer);
    raster.composite_matte(p.fill(&shape, footile::FillRule::EvenOdd), pix::rgb::SRgba8::new(52, 32, 64, 255).convert(), pix /**/, pix::SrcOver);
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
    crate::icons::text(slice, width, height as u16, "Plop Grizzlyhna2");*/
}

// Initialize graphic shader.
pub(crate) fn init_toolbar(window: &mut window::Window) -> (window::Shader, Group) {
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
    let rect = ShapeBuilder::new(&mut gui).vert(&vertices).face(Transform::new()).finish();

    let mut group = window.group_new();
    group.push(&rect, &Transform::new());
    (gui, group)
}

pub(crate) fn initialize_video_io(name: &str, run: fn(nanos: u64) -> ()) {
    use window::*;

    unsafe {
        let video_io = &mut VIDEO_IO as *mut _ as *mut VideoIO;
        let shader = vec![];
        let shadet = vec![];

        let mut window = Window::new(name, run);
        // window.toolbar(toolbar);

        std::ptr::write(video_io, VideoIO {
            window, shader, shadet,
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

/// Draw a group on the screen.
pub fn draw(shader: &Shader, group: &mut Group) {
    let video_io = unsafe { &mut VIDEO_IO as *mut _ as *mut VideoIO };

    unsafe {
        (*video_io).window.draw((*video_io).shader[shader.0].as_ref().unwrap(), group);
    }
}

/// Set camera for shader.
pub fn set_camera(shader: &Shader, camera: Transform) {
    let video_io = unsafe { &mut VIDEO_IO as *mut _ as *mut VideoIO };

    unsafe {
        (*video_io).window.camera((*video_io).shader[shader.0].as_ref().unwrap(), camera);
    }
}
/// Set tint for shader.
pub fn set_tint(shader: &Shader, tint: [f32; 4]) {
    let video_io = unsafe { &mut VIDEO_IO as *mut _ as *mut VideoIO };

    unsafe {
        (*video_io).window.tint((*video_io).shader[shader.0].as_ref().unwrap(), tint);
    }
}

/// Draw a group with a texture on the screen.
pub fn draw_graphic(shader: &Shader, group: &mut Group, graphic: &Graphic) {
    let video_io = unsafe { &mut VIDEO_IO as *mut _ as *mut VideoIO };

    unsafe {
        (*video_io).window.draw_graphic((*video_io).shader[shader.0].as_ref().unwrap(), group, graphic);
    }
}

/// Load a graphic.
pub fn graphic(pixels: &[u8], width: usize, height: usize) -> Graphic {
    let video_io = unsafe { &mut VIDEO_IO as *mut _ as *mut VideoIO };

    unsafe {
        (*video_io).window.graphic(pixels, width, height)
    }
}

/// Create a new group.
pub fn group_new() -> Group {
    let video_io = unsafe { &mut VIDEO_IO as *mut _ as *mut VideoIO };

    unsafe {
        (*video_io).window.group_new()
    }
}

/// If a key is being held down.
pub fn key(key: Key) -> bool {
    let video_io = unsafe { &mut VIDEO_IO as *mut _ as *mut VideoIO };

    unsafe {
        (*video_io).window.key(key)
    }
}

/// Get the window aspect ratio.
pub fn aspect() -> f32 {
    let video_io = unsafe { &mut VIDEO_IO as *mut _ as *mut VideoIO };

    unsafe {
        (*video_io).window.aspect()
    }
}
