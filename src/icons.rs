#![allow(unused)] // FIXME: remove this

use fonterator::footile;
use rvg::*;

const BACK: &[u8] = include_bytes!("../rvg/back.svg.rvg");
const EXIT: &[u8] = include_bytes!("../rvg/exit.svg.rvg");
const FULLSCREEN: &[u8] = include_bytes!("../rvg/fullscreen.svg.rvg");
const GRID: &[u8] = include_bytes!("../rvg/grid.svg.rvg");
const HIDE: &[u8] = include_bytes!("../rvg/hide.svg.rvg");
const MENU: &[u8] = include_bytes!("../rvg/menu.svg.rvg");
const MORE: &[u8] = include_bytes!("../rvg/more.svg.rvg");
const NEW: &[u8] = include_bytes!("../rvg/new.svg.rvg");
const NEXT: &[u8] = include_bytes!("../rvg/next.svg.rvg");
const SEARCH: &[u8] = include_bytes!("../rvg/search.svg.rvg");
const VIEW: &[u8] = include_bytes!("../rvg/view.svg.rvg");
const ZOOM_IN: &[u8] = include_bytes!("../rvg/zoom_in.svg.rvg");
const ZOOM_OUT: &[u8] = include_bytes!("../rvg/zoom_out.svg.rvg");

pub fn text(
    pixels: &mut [footile::Rgba8],
    width: u16,
    graphic_h: u16,
    text: &str,
) {
    let font = fonterator::normal_font();
    let graphic_h = graphic_h / 2;

    // Render
    let mut p =
        footile::Plotter::new(u32::from(width), u32::from(graphic_h) * 2);
    let r = footile::RasterB::new(p.width(), p.height());
    let path: Vec<_> = font
        .render(
            text,                                               /*text*/
            (0.0, 0.0, f32::from(width), f32::from(graphic_h)), /*bbox*/
            (f32::from(graphic_h), f32::from(graphic_h)),       /*size*/
            fonterator::TextAlign::Center,
        )
        .0
        .collect();
    r.over(
        p.fill(&path, footile::FillRule::NonZero),
        footile::Rgba8::rgb(200, 200, 200), /*color*/
        unsafe {
            std::slice::from_raw_parts_mut(
                pixels.as_mut_ptr(),
                usize::from(width) * usize::from(graphic_h) * 2,
            )
        },
    );
}

fn half(
    pixels: &mut [footile::Rgba8],
    mut x: u16,
    width: u16,
    graphic_h: u16,
    slice: &[u8],
) {
    let margin = graphic_h / 8;
    let graphic_width = (graphic_h / 2) - (margin);
    let ad = (graphic_h / 2) - (margin);

    let offs = if x > 6 {
        x -= 6;
        width - (8 * ad)
    } else {
        0
    };
    render_from_rvg(slice, pixels, width, offs + x * ad, margin, graphic_width)
}

fn full(
    pixels: &mut [footile::Rgba8],
    mut x: u16,
    width: u16,
    graphic_h: u16,
    slice: &[u8],
) {
    let margin = graphic_h / 8;
    let graphic_width = (graphic_h) - (margin * 2);
    let ad = (graphic_h / 2) - (margin);

    let offs = if x > 6 {
        x -= 6;
        width - (8 * ad)
    } else {
        0
    };
    render_from_rvg(slice, pixels, width, offs + x * ad, margin, graphic_width)
}

pub fn back(pixels: &mut [footile::Rgba8], x: u16, width: u16, graphic_h: u16) {
    half(pixels, x, width, graphic_h, BACK);
}

pub fn next(pixels: &mut [footile::Rgba8], x: u16, width: u16, graphic_h: u16) {
    half(pixels, x, width, graphic_h, NEXT);
}

pub fn menu(pixels: &mut [footile::Rgba8], x: u16, width: u16, graphic_h: u16) {
    half(pixels, x, width, graphic_h, MENU);
}

pub fn exit(pixels: &mut [footile::Rgba8], x: u16, width: u16, graphic_h: u16) {
    half(pixels, x, width, graphic_h, EXIT);
}

pub fn new(pixels: &mut [footile::Rgba8], x: u16, width: u16, graphic_h: u16) {
    full(pixels, x, width, graphic_h, NEW);
}

pub fn more(pixels: &mut [footile::Rgba8], x: u16, width: u16, graphic_h: u16) {
    full(pixels, x, width, graphic_h, MORE);
}

pub fn search(
    pixels: &mut [footile::Rgba8],
    x: u16,
    width: u16,
    graphic_h: u16,
) {
    full(pixels, x, width, graphic_h, SEARCH);
}

pub fn grid(pixels: &mut [footile::Rgba8], x: u16, width: u16, graphic_h: u16) {
    full(pixels, x, width, graphic_h, GRID);
}

pub fn hide(pixels: &mut [footile::Rgba8], x: u16, width: u16, graphic_h: u16) {
    full(pixels, x, width, graphic_h, HIDE);
}

pub fn fullscreen(
    pixels: &mut [footile::Rgba8],
    x: u16,
    width: u16,
    graphic_h: u16,
) {
    full(pixels, x, width, graphic_h, FULLSCREEN);
}

pub fn zoom_out(
    pixels: &mut [footile::Rgba8],
    x: u16,
    width: u16,
    graphic_h: u16,
) {
    full(pixels, x, width, graphic_h, ZOOM_OUT);
}

pub fn zoom_in(
    pixels: &mut [footile::Rgba8],
    x: u16,
    width: u16,
    graphic_h: u16,
) {
    full(pixels, x, width, graphic_h, ZOOM_IN);
}

pub fn view(pixels: &mut [footile::Rgba8], x: u16, width: u16, graphic_h: u16) {
    full(pixels, x, width, graphic_h, VIEW);
}
