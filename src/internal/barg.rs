// use std::sync::mpsc::{channel, Sender, Receiver};

#[doc(hidden)]
pub use barg::{ShaderBuilder};

pub use barg::{ShapeBuilder, Transform, Graphic};

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
/*/// A vertex list.  Vertex lists are a list of vertex positions, and associated
/// data.  Associated data may refer to graphic coordinates and/or colors.
pub struct VertexList(usize);*/

/*enum VideoMsg {
    Background{r:f32,g:f32,b:f32},
    ShaderNew{builder:ShaderBuilder, index:usize},
    ShaderOld{index:usize},
//    ShapeNew{builder:ShapeBuilder<'_>, index:usize},
    ShapeOld{index:usize},
//    VertexListNew{vertices:Vec<f32>, dim: u8, gradient: u8, graphic_coords: u8, index:usize},
//    VertexListOld{index:usize},
}*/

struct VideoIO {
    window: Box<barg::Window>,
//    sender: Sender<VideoMsg>,
//    recver: Receiver<VideoMsg>,
    shader: Vec<Option<barg::Shader>>,
    shadet: Vec<usize>,
    shapes: Vec<Option<barg::Shape>>,
    shapet: Vec<usize>,
//    vertls: Vec<Option<barg::VertexList>>,
//    vertlt: Vec<usize>,
}

static mut VIDEO_IO: FakeVideoIO = FakeVideoIO([0; std::mem::size_of::<VideoIO>()]);

#[repr(align(8))]
struct FakeVideoIO([u8; std::mem::size_of::<VideoIO>()]);

// // // // // //

impl Drop for Shader {
    fn drop(&mut self) {
        let video_io = unsafe { &mut VIDEO_IO as *mut _ as *mut VideoIO };

        unsafe {
//            (*video_io).sender.send(VideoMsg::ShaderOld{index:self.0}).unwrap();
            (*video_io).shadet.push(self.0);
            (*video_io).shader[self.0] = None;
        }
    }
}

/*impl Drop for VertexList {
    fn drop(&mut self) {
        let video_io = unsafe { &mut VIDEO_IO as *mut _ as *mut VideoIO };

        unsafe {
            (*video_io).sender.send(VideoMsg::VertexListOld{index:self.0}).unwrap();
            (*video_io).vertlt.push(self.0);
        }
    }
}*/

impl Drop for Shape {
    fn drop(&mut self) {
        let video_io = unsafe { &mut VIDEO_IO as *mut _ as *mut VideoIO };

        unsafe {
//            (*video_io).sender.send(VideoMsg::ShapeOld{index:self.0}).unwrap();
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

//            (*video_io).sender.send(VideoMsg::ShaderNew{builder, index}).unwrap();

            index
        };

        Shader(index)
    }

/*    /// Add a shape that uses this shader.
    pub fn add<'a>(&'a mut self) -> ShapeBuilder<'a> {
        ShapeBuilder {
            dimensions: 0,
            vertices: Vec::new(),
            ops: Vec::new(),
            shader: self,
        }

//        Shape(0) // TODO
    }*/
}

/*impl VertexList {
    /// Build a vertex list.
    pub fn new(vertices: Vec<f32>, dim: u8, gradient: u8, graphic_coords: u8) -> VertexList {
        let video_io = unsafe { &mut VIDEO_IO as *mut _ as *mut VideoIO };

        let index = unsafe {
            let index = if let Some(index) = (*video_io).vertlt.pop() {
                index
            } else {
                (*video_io).vertls.len()
            };

            (*video_io).sender.send(VideoMsg::VertexListNew{vertices, dim, gradient, graphic_coords, index}).unwrap();

            index
        };

        VertexList(index)
    }
}*/

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

// fn run(nanos: u64) {
/*    let video_io = unsafe { &mut VIDEO_IO as *mut _ as *mut VideoIO };

    unsafe {
        while let Ok(msg) = (*video_io).recver.try_recv() {
            match msg {
                VideoMsg::Background{r,g,b} => {
                    (*video_io).window.background(r, g, b);
                }
                VideoMsg::ShaderNew{builder, index} => {
                    let shader = (*video_io).window.shader_new(builder);

                    if index == (*video_io).shader.len() {
                        (*video_io).shader.push(Some(shader));
                    } else {
                        (*video_io).shader[index] = Some(shader);
                    }
                }
                VideoMsg::ShaderOld{index} => {
                    (*video_io).shader[index] = None;
                }
/*                VideoMsg::ShapeNew{builder, index} => {
                    let shape = (*video_io).window.shape_new(builder);

                    if index == (*video_io).shapes.len() {
                        (*video_io).shapes.push(Some(shape));
                    } else {
                        (*video_io).shapes[index] = Some(shape);
                    }
                }*/
                VideoMsg::ShapeOld{index} => {
                    (*video_io).shapes[index] = None;
                }
/*                VideoMsg::VertexListNew{vertices, dim, gradient, graphic_coords, index} => {
                    let vl = (*video_io).window.vertex_list_new(vertices.as_slice(), dim, gradient, graphic_coords);

                    if index == (*video_io).vertls.len() {
                        (*video_io).vertls.push(Some(vl));
                    } else {
                        (*video_io).vertls[index] = Some(vl);
                    }
                }
                VideoMsg::VertexListOld{index} => {
                    (*video_io).vertls[index] = None;
                }*/
            }
        }
    }*/
// }

pub(crate) fn initialize_video_io(name: &str, run: fn(nanos: u64) -> ()) {
    use barg::*;

    unsafe {
        let video_io = &mut VIDEO_IO as *mut _ as *mut VideoIO;
//        let (sender, recver) = channel();
        let shader = vec![];
        let shadet = vec![];
        let shapes = vec![];
        let shapet = vec![];
//        let vertls = vec![];
//        let vertlt = vec![];

        std::ptr::write(video_io, VideoIO {
            window: Window::new(name, run),
            //sender, recver,
            shader, shadet, shapes, shapet, // vertls, vertlt
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
//        (*video_io).sender.send(VideoMsg::Background{r,g,b}).unwrap();
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
pub fn draw(shader: &Shader, shape: &Shape/*, shape: &[Instance]*/) {
    let video_io = unsafe { &mut VIDEO_IO as *mut _ as *mut VideoIO };

    unsafe {
        (*video_io).window.draw((*video_io).shader[shader.0].as_ref().unwrap(), (*video_io).shapes[shape.0].as_ref().unwrap());
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
