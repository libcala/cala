//! **feature:pixels** - Display graphics onto the screen, usually via a window.
//!
//! # Getting Started
//! ```rust
//! // TODO
//! ```

use crate::prelude::*;

use super::draw::*;
use pix::chan::Channel;
use std::{
    future::Future,
    pin::Pin,
    sync::{
        atomic::{AtomicU32, Ordering},
        Arc, Condvar, Mutex,
    },
    task::{Context, Poll},
};

static BACKGROUND_RED: AtomicU32 = AtomicU32::new(0);
static BACKGROUND_GREEN: AtomicU32 = AtomicU32::new(0);
static BACKGROUND_BLUE: AtomicU32 = AtomicU32::new(0);

struct FrameFuture;

impl Future for FrameFuture {
    type Output = (std::time::Duration, f32, bool);

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

/// Get a canvas for the screen.
pub async fn canvas<P: pix::el::Pixel>(color: P) -> impl Canvas
where
    pix::chan::Ch32: From<<P as pix::el::Pixel>::Chan>,
{
    Frame::new(color).await
}

/// A Canvas to draw on.
struct Frame {
    // For when drop'd; to notify graphics thread
    pair: Arc<(Mutex<bool>, Condvar)>,
    // Delta time since previous frame
    elapsed: std::time::Duration,
    // Aspect ratio
    aspect: f32,
    // If resized
    resized: bool,
}

impl Frame {
    /// Wait for the GPU to request a new frame.
    #[allow(clippy::useless_let_if_seq)] // Clippy doesn't understand
    pub async fn new<P: pix::el::Pixel>(color: P) -> Frame
    where
        pix::chan::Ch32: From<<P as pix::el::Pixel>::Chan>,
    {
        let color: pix::rgb::SRgb32 = color.convert();
        let red = color.one().to_f32();
        let green = color.two().to_f32();
        let blue = color.three().to_f32();
        let red_u32 = u32::from_ne_bytes(red.to_ne_bytes());
        let green_u32 = u32::from_ne_bytes(green.to_ne_bytes());
        let blue_u32 = u32::from_ne_bytes(blue.to_ne_bytes());

        let mut bg_changed = false;
        if BACKGROUND_RED.load(Ordering::Relaxed) != red_u32 {
            BACKGROUND_RED.store(red_u32, Ordering::Relaxed);
            bg_changed = true;
        }
        if BACKGROUND_GREEN.load(Ordering::Relaxed) != blue_u32 {
            BACKGROUND_GREEN.store(green_u32, Ordering::Relaxed);
            bg_changed = true;
        }
        if BACKGROUND_BLUE.load(Ordering::Relaxed) != green_u32 {
            BACKGROUND_BLUE.store(blue_u32, Ordering::Relaxed);
            bg_changed = true;
        }
        let secs = FrameFuture.await;
        let internal = Internal::new_lazy();
        let mut cmds = internal.cmds.lock().unwrap();
        let pair = internal.pair.clone();
        if bg_changed {
            cmds.push(GpuCmd::Background(red, green, blue));
        }
        Frame {
            pair,
            elapsed: secs.0,
            aspect: secs.1,
            resized: secs.2,
        }
    }
}

impl Canvas for Frame {
    fn draw(&mut self, shader: &Shader, group: &Group) {
        let internal = Internal::new_lazy();
        let mut cmds = internal.cmds.lock().unwrap();
        cmds.push(GpuCmd::Draw(shader.0, group.0));
    }

    fn set_camera(&mut self, camera: Transform) {
        let internal = Internal::new_lazy();
        let mut cmds = internal.cmds.lock().unwrap();
        cmds.push(GpuCmd::SetCamera(camera));
    }

    fn set_tint<P: pix::el::Pixel>(&mut self, shader: &Shader, tint: P)
    where
        pix::chan::Ch32: From<<P as pix::el::Pixel>::Chan>,
    {
        let internal = Internal::new_lazy();
        let mut cmds = internal.cmds.lock().unwrap();
        let color: pix::rgb::SRgba32 = tint.convert();
        let red = color.one().to_f32();
        let green = color.two().to_f32();
        let blue = color.three().to_f32();
        let alpha = color.four().to_f32();
        cmds.push(GpuCmd::SetTint(shader.0, [red, green, blue, alpha]));
    }

    fn draw_graphic(
        &mut self,
        shader: &Shader,
        group: &Group,
        graphic: &Texture,
    ) {
        let internal = Internal::new_lazy();
        let mut cmds = internal.cmds.lock().unwrap();
        cmds.push(GpuCmd::DrawGraphic(shader.0, group.0, graphic.0));
    }

    fn elapsed(&self) -> std::time::Duration {
        self.elapsed
    }

    fn height(&self) -> f32 {
        self.aspect
    }

    fn resized(&self) -> bool {
        self.resized
    }
}

impl Drop for Frame {
    fn drop(&mut self) {
        let (lock, cvar) = &*self.pair;
        let mut started = lock.lock().unwrap();
        *started = true;
        // We notify the condvar that the value has changed.
        cvar.notify_one();
    }
}
