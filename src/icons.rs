#![allow(unused)] // FIXME: remove this

use footile::{Plotter, Transform};
use pix::chan::Ch8;
use pix::chan::Linear;
use pix::chan::Premultiplied;
use pix::{
    el::{Pix1, Pixel},
    matte::Matte8,
    ops::SrcOver,
    rgb::SRgba8,
    Raster,
};
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

pub fn text<P>(
    raster: &mut Raster<P>,
    plotter: &mut Plotter<Matte8>,
    size: f32,
    text: &str,
) where
    P::Chan: From<Ch8>,
    P: Pixel<Gamma = Linear, Alpha = Premultiplied>,
{
    plotter.set_transform(Transform::with_scale(size, size));
    let font = fonterator::normal_font();

    // Render
    let path: Vec<_> = font
        .render(
            text,                         /*text*/
            raster.width() as f32 / size, /*bbox*/
            fonterator::TextAlign::Center,
        )
        .0
        .collect();

    let temp_raster =
        Raster::<Pix1<P::Chan, _, pix::chan::Premultiplied, _>>::with_raster(
            plotter.fill(footile::FillRule::NonZero, &path, Matte8::new(255)),
        );

    raster.composite_matte(
        (),
        &temp_raster,
        (),
        SRgba8::new(200, 200, 200, 255).convert(), /*color*/
        SrcOver,
    );
}

pub struct Icons {
    back: Graphic,
    exit: Graphic,
    fullscreen: Graphic,
    grid: Graphic,
    hide: Graphic,
    menu: Graphic,
    more: Graphic,
    new: Graphic,
    next: Graphic,
    search: Graphic,
    view: Graphic,
    zoom_in: Graphic,
    zoom_out: Graphic,
}

impl Icons {
    fn new() -> Self {
        Icons {
            back: Graphic::load(BACK).unwrap(),
            exit: Graphic::load(EXIT).unwrap(),
            fullscreen: Graphic::load(FULLSCREEN).unwrap(),
            grid: Graphic::load(GRID).unwrap(),
            hide: Graphic::load(HIDE).unwrap(),
            menu: Graphic::load(MENU).unwrap(),
            more: Graphic::load(MORE).unwrap(),
            new: Graphic::load(NEW).unwrap(),
            next: Graphic::load(NEXT).unwrap(),
            search: Graphic::load(SEARCH).unwrap(),
            view: Graphic::load(VIEW).unwrap(),
            zoom_in: Graphic::load(ZOOM_IN).unwrap(),
            zoom_out: Graphic::load(ZOOM_OUT).unwrap(),
        }
    }

    pub fn back<P>(&self, r: &mut Raster<P>, x: u16)
    where
        P::Chan: From<Ch8>,
        P: Pixel<Gamma = Linear, Alpha = Premultiplied>,
    {
        half(r, &self.back, x);
    }

    pub fn next<P>(&self, r: &mut Raster<P>, x: u16)
    where
        P::Chan: From<Ch8>,
        P: Pixel<Gamma = Linear, Alpha = Premultiplied>,
    {
        half(r, &self.next, x);
    }

    pub fn menu<P>(&self, r: &mut Raster<P>, x: u16)
    where
        P::Chan: From<Ch8>,
        P: Pixel<Gamma = Linear, Alpha = Premultiplied>,
    {
        half(r, &self.menu, x);
    }

    pub fn exit<P>(&self, r: &mut Raster<P>, x: u16)
    where
        P::Chan: From<Ch8>,
        P: Pixel<Gamma = Linear, Alpha = Premultiplied>,
    {
        half(r, &self.exit, x);
    }

    pub fn create<P>(&self, r: &mut Raster<P>, x: u16)
    where
        P::Chan: From<Ch8>,
        P: Pixel<Gamma = Linear, Alpha = Premultiplied>,
    {
        full(r, &self.new, x);
    }

    pub fn more<P>(&self, r: &mut Raster<P>, x: u16)
    where
        P::Chan: From<Ch8>,
        P: Pixel<Gamma = Linear, Alpha = Premultiplied>,
    {
        full(r, &self.more, x);
    }

    pub fn search<P>(&self, r: &mut Raster<P>, x: u16)
    where
        P::Chan: From<Ch8>,
        P: Pixel<Gamma = Linear, Alpha = Premultiplied>,
    {
        full(r, &self.search, x);
    }

    pub fn grid<P>(&self, r: &mut Raster<P>, x: u16)
    where
        P::Chan: From<Ch8>,
        P: Pixel<Gamma = Linear, Alpha = Premultiplied>,
    {
        full(r, &self.grid, x);
    }

    pub fn hide<P>(&self, r: &mut Raster<P>, x: u16)
    where
        P::Chan: From<Ch8>,
        P: Pixel<Gamma = Linear, Alpha = Premultiplied>,
    {
        full(r, &self.hide, x);
    }

    pub fn fullscreen<P>(&self, r: &mut Raster<P>, x: u16)
    where
        P::Chan: From<Ch8>,
        P: Pixel<Gamma = Linear, Alpha = Premultiplied>,
    {
        full(r, &self.fullscreen, x);
    }

    pub fn zoom_out<P>(&self, r: &mut Raster<P>, x: u16)
    where
        P::Chan: From<Ch8>,
        P: Pixel<Gamma = Linear, Alpha = Premultiplied>,
    {
        full(r, &self.zoom_out, x);
    }

    pub fn zoom_in<P>(&self, r: &mut Raster<P>, x: u16)
    where
        P::Chan: From<Ch8>,
        P: Pixel<Gamma = Linear, Alpha = Premultiplied>,
    {
        full(r, &self.zoom_in, x);
    }

    pub fn view<P>(&self, r: &mut Raster<P>, x: u16)
    where
        P::Chan: From<Ch8>,
        P: Pixel<Gamma = Linear, Alpha = Premultiplied>,
    {
        full(r, &self.view, x);
    }
}

fn half<P>(raster: &mut Raster<P>, graphic: &Graphic, x: u16)
where
    P::Chan: From<Ch8>,
    P: Pixel<Gamma = Linear, Alpha = Premultiplied>,
{
    rvg::render(raster, graphic, (32.0 * x as f32, 0.0, 32.0, 64.0))
}

fn full<P>(raster: &mut Raster<P>, graphic: &Graphic, x: u16)
where
    P::Chan: From<Ch8>,
    P: Pixel<Gamma = Linear, Alpha = Premultiplied>,
{
    rvg::render(raster, graphic, (32.0 * x as f32, 0.0, 64.0, 64.0))
}
