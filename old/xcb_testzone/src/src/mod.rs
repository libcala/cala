// This is an example program.

use png;
use super::adi::*;

const IMAGE: &'static [u8] = include_bytes!("WaterOcean.png");

/// Bitmap Ids
#[repr(u32)]
enum BitmapId {
    /// First bitmap.
    Water = 0u32,
}

static mut BUF: Option<Vec<u8>> = None;

/// The application context.
pub struct App {
}

/// The initial application context.
pub const APP_INIT: App = App {
};

fn png_to_rgba(file: &[u8]) -> (u32, Vec<u8>) {
    // Load the PNG file.
    let decoder = png::Decoder::new(std::io::Cursor::new(IMAGE));
    let (info, mut reader) = decoder.read_info().unwrap();
    // Allocate the output buffer.
    let mut buf = vec![0; info.buffer_size()];
    // Read the next frame. Currently this function should only called once.
    // The default options
    reader.next_frame(buf.as_mut_slice()).unwrap();

    let buf = match info.color_type {
        png::ColorType::RGBA => {
            buf
        }
        png::ColorType::RGB => {
            let mut rgba = vec![];
            let mut x = 0;
            for i in buf.iter() {
                rgba.push(buf[x]);
                if x % 3 == 2 {
                    rgba.push(255);
                }
                x += 1;
            }
            rgba
        }
        _ => {
            panic!("Unknown Color Type!");
        }
    };

    (info.width, buf)
}

/// The initial runner for this application.
pub fn run_init(sys: &mut Sys<App>) {
    let (width, buf) = png_to_rgba(IMAGE);

    sys.send(vec![
        OutputMsg::LoadBitmap(BitmapId::Water as u32, width, buf),
        OutputMsg::Clear([0x88, 0x88, 0x88, 0xFF]),
        OutputMsg::Move(0.75, 0.5, 0.0),
        OutputMsg::Line(0.25, 0.5, 0.0),
        OutputMsg::Line(0.5, 0.0, 0.0),
        OutputMsg::Line(0.75, 0.5, 0.0),
        OutputMsg::Close(),
        OutputMsg::Sync(),
        OutputMsg::DrawBitmap(
            [
                [0.25, 0.0, 0.0],
                [0.75, 0.0, 0.0],
                [0.75, 0.5, 0.0],
                [0.25, 0.5, 0.0],
            ],
            BitmapId::Water as u32
        ),
    ]);
    sys.run = run_stall;
}

pub fn run_stall(sys: &mut Sys<App>) {
    while let Some(_input) = sys.recv() {
        // TODO
    }
    std::thread::sleep(std::time::Duration::from_millis(10));
}
