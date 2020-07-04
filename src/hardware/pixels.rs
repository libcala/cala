//! **feature:pixels** - Display graphics onto the screen, usually via a window.

use crate::prelude::*;

use pix::chan::Channel;
use std::{sync::{atomic::{Ordering, AtomicU32}, Condvar, Arc, Mutex, MutexGuard}, task::{Context, Poll}, pin::Pin, future::Future};
use super::draw::*;

static BACKGROUND_RED: AtomicU32 = AtomicU32::new(0);
static BACKGROUND_GREEN: AtomicU32 = AtomicU32::new(0);
static BACKGROUND_BLUE: AtomicU32 = AtomicU32::new(0);

struct FrameFuture;

impl Future for FrameFuture {
    type Output = (std::time::Duration, f32);

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
struct Frame<'a> {
    // Lock on the Gpu command buffers.
    cmds: MutexGuard<'a, Vec<GpuCmd>>,
    // For when drop'd; to notify graphics thread
    pair: Arc<(Mutex<bool>, Condvar)>,
    // Delta time since previous frame
    elapsed: std::time::Duration,
    // Aspect ratio
    aspect: f32,
}

impl<'a> Frame<'a> {
    /// Wait for the GPU to request a new frame.
    pub async fn new<P: pix::el::Pixel>(color: P) -> Frame<'a>
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
        Frame { cmds, pair, elapsed: secs.0, aspect: secs.1 }
    }
}

impl<'a> Canvas for Frame<'a> {
    fn draw(&mut self, shader: &Shader, group: &Arc<Group>) {
        self.cmds.push(GpuCmd::Draw(shader.0, group.clone()));
    }

    fn set_camera(&mut self, shader: &Shader, camera: Transform) {
        self.cmds.push(GpuCmd::SetCamera(shader.0, camera));
    }

    fn set_tint<P: pix::el::Pixel>(&mut self, shader: &Shader, tint: P)
    where
        pix::chan::Ch32: From<<P as pix::el::Pixel>::Chan>,
    {
        let color: pix::rgb::SRgba32 = tint.convert();
        let red = color.one().to_f32();
        let green = color.two().to_f32();
        let blue = color.three().to_f32();
        let alpha = color.four().to_f32();
        self.cmds.push(GpuCmd::SetTint(shader.0, [red, green, blue, alpha]));
    }

    fn draw_graphic(
        &mut self,
        shader: &Shader,
        group: &Arc<Group>,
        graphic: &Texture,
    ) {
        self.cmds.push(GpuCmd::DrawGraphic(
            shader.0,
            group.clone(),
            graphic.0,
        ));
    }
    
    fn elapsed(&self) -> std::time::Duration {
        self.elapsed
    }
    
    fn aspect(&self) -> f32 {
        self.aspect
    }
}

impl<'a> Drop for Frame<'a> {
    fn drop(&mut self) {
        let (lock, cvar) = &*self.pair;
        let mut started = lock.lock().unwrap();
        *started = true;
        // We notify the condvar that the value has changed.
        cvar.notify_one();
    }
}
