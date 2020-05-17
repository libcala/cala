//! TCP and UDP(**WIP**) network communication.  Enable with the `net` feature.
//!
//! "Aren't TCP and UDP in the standard library?", you might ask.  Yeah, but
//! this is asynchronous.
//!
//! # Getting Started
//!
//! ## Example

use smelling_salts::{Device, Watcher};
use std::{
    io::{Result as IoResult, ErrorKind, Read, Write},
    net::{ToSocketAddrs, TcpListener, TcpStream},
    os::unix::io::AsRawFd,
    task::{Context, Poll},
    pin::Pin,
    future::Future,
};

const DEFAULT_BUF_SIZE: usize = 8 * 1024;

/// Which type of event happenned on the server.
pub enum ServerEvent {
    /// A new packet is ready to be read.
    Receive,
    /// A new client is trying to connect.
    Connect(IoResult<TcpConnection>),
}

/// TCP socket server, listening for connections.
pub struct TcpServer {
    std: TcpListener,
    dev: Device,
}

impl TcpServer {
    /// Try creating a new `TcpServer` at the specified `address`.
    pub fn new<A: ToSocketAddrs>(address: A) -> Option<Self> {
        let std = TcpListener::bind(address).ok()?;
        std.set_nonblocking(true).expect("Failed to set non-blocking");

        let dev = Device::new(std.as_raw_fd(), Watcher::new().input());

        Some(TcpServer { std, dev })
    }
}

impl Future for TcpServer {
    type Output = (usize, ServerEvent);

    fn poll(self: Pin<&mut Self>, cx: &mut Context<'_>) -> Poll<Self::Output> {
        match self.std.accept() {
            Ok(stream) => {
                // Send task to selected thread.
                let std = stream.0;
                std.set_nonblocking(true).expect("Couldn't set unblocking!");
                let dev = Device::new(std.as_raw_fd(), Watcher::new().input());

                Poll::Ready((usize::MAX, ServerEvent::Connect(Ok(TcpConnection { std, dev }))))
            }
            Err(ref e) if e.kind() == ErrorKind::WouldBlock => {
                self.dev.register_waker(cx.waker());
                Poll::Pending
            }
            Err(e) => Poll::Ready((usize::MAX, ServerEvent::Connect(Err(e)))),
        }
    }
}

impl Drop for TcpServer {
    fn drop(&mut self) {
        self.dev.old();
    }
}

/// TCP stream between a local and remote socket.
pub struct TcpConnection {
    std: TcpStream,
    dev: Device,
}

impl TcpConnection {
    /// Try creating a new client `TcpConnection` with the specified server
    /// `address`.
    pub fn new<A: ToSocketAddrs>(address: A) -> Option<Self> {
        let std = TcpStream::connect(address).ok()?;
        std.set_nonblocking(true).expect("Failed to set non-blocking");

        let dev = Device::new(std.as_raw_fd(), Watcher::new().input());

        Some(TcpConnection { std, dev })
    }

    /// Send data on the stream.
    pub async fn send(&mut self, immediate: bool, data: &[u8]) {
        TcpSender(self, immediate, data).await;
        if immediate {
            TcpFlush(self).await;
        }
    }
    
    /// Receive data from the stream.
    pub async fn recv(&mut self, data: &mut Vec<u8>) {
        TcpReceiver(self, data).await;
    }
}

impl Future for TcpConnection {
    type Output = ServerEvent;

    fn poll(self: Pin<&mut Self>, cx: &mut Context<'_>) -> Poll<Self::Output> {
        let this = self.get_mut();
        let mut buffer = [0; DEFAULT_BUF_SIZE];
        loop {
            match this.std.peek(&mut buffer) {
                Ok(bytes) if bytes != 0 => {
                    if bytes != DEFAULT_BUF_SIZE {
                        return Poll::Ready(ServerEvent::Receive)
                    }
                }
                Err(ref e) if e.kind() != ErrorKind::WouldBlock => {
                    panic!("Stream Read IO Error {}!", e)
                }
                _ => {
                    this.dev.register_waker(cx.waker());
                    return Poll::Pending
                }
            }
        }
    }
}

impl Drop for TcpConnection {
    fn drop(&mut self) {
        self.dev.old();
    }
}

struct TcpSender<'a>(&'a mut TcpConnection, bool, &'a [u8]);

impl Future for TcpSender<'_> {
    type Output = ();

    fn poll(self: Pin<&mut Self>, cx: &mut Context<'_>) -> Poll<()> {
        let this = self.get_mut();
        match this.0.std.write(&mut this.2) {
            Ok(_) => Poll::Ready(()),
            Err(ref e) if e.kind() == ErrorKind::WouldBlock => {
                this.0.dev.register_waker(cx.waker());
                Poll::Pending
            }
            Err(e) => panic!("Stream Write IO Error {}!", e),
        }
    }
}

struct TcpFlush<'a>(&'a mut TcpConnection);

impl Future for TcpFlush<'_> {
    type Output = ();

    fn poll(mut self: Pin<&mut Self>, cx: &mut Context<'_>) -> Poll<()> {
        match self.0.std.flush() {
            Ok(_) => Poll::Ready(()),
            Err(ref e) if e.kind() == ErrorKind::WouldBlock => {
                self.0.dev.register_waker(cx.waker());
                Poll::Pending
            }
            Err(e) => panic!("Stream Write IO Error {}!", e),
        }
    }
}

struct TcpReceiver<'a>(&'a mut TcpConnection, &'a mut Vec<u8>);

impl Future for TcpReceiver<'_> {
    type Output = ();
    
    fn poll(self: Pin<&mut Self>, cx: &mut Context<'_>) -> Poll<()> {
        let this = self.get_mut();
        let mut buffer = [0; DEFAULT_BUF_SIZE];
        loop {
            match this.0.std.read(&mut buffer) {
                Ok(bytes) if bytes != 0 => {
                    this.1.extend(&buffer[..bytes]);
                    if bytes != DEFAULT_BUF_SIZE {
                        return Poll::Ready(())
                    }
                }
                Err(ref e) if e.kind() != ErrorKind::WouldBlock => {
                    panic!("Stream Read IO Error {}!", e)
                }
                _ => {
                    this.0.dev.register_waker(cx.waker());
                    return Poll::Pending
                }
            }
        }
    }
}
