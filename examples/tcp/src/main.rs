// Networking example

use std::sync::{Arc, Condvar, Mutex};

use cala::*;
use net::{ServerEvent, TcpConnection, TcpServer};

exec!(init);

async fn init() {
    let pair = Arc::new((Mutex::new(false), Condvar::new()));
    let pair2 = pair.clone();
    [server(pair).fut(), client(pair2).fut()].select().await;
}

async fn server(pair: Arc<(Mutex<bool>, Condvar)>) {
    println!("Starting Server…");
    let mut server = TcpServer::new("127.0.0.1:21135").unwrap();
    // Notify client thread that server is up and running.
    let (lock, cvar) = &*pair;
    {
        let mut started = lock.lock().unwrap();
        *started = true;
        cvar.notify_one();
    }
    // Handle connections.
    let mut connections = Vec::new();
    loop {
        match [server.fut(), connections.select().fut()].select().await.1 {
            (_, ServerEvent::Connect(connection)) => {
                println!("Got connection!");
                connections.push(connection.unwrap());
            }
            (id, ServerEvent::Receive) => {
                let mut buffer = Vec::new();
                connections[id].recv(&mut buffer).await; // Non-blocking
                println!("New data on {}: {:?}", id, buffer);
                connections[id].send(true, &[1, 2, 3, 4]).await; // Blocking.
            }
        }
    }
}

async fn client(pair: Arc<(Mutex<bool>, Condvar)>) {
    println!("Waiting for server to start…");
    let (lock, cvar) = &*pair;
    let mut started = lock.lock().unwrap();
    while !*started {
        started = cvar.wait(started).unwrap();
    }
    println!("Starting Client…");
    let mut client = TcpConnection::new("127.0.0.1:21135").unwrap();
    println!("Sending Data…");
    client.send(true, &[42]).await;
    let mut buffer = Vec::new();
    client.recv(&mut buffer).await;
    println!("Received {:?}, exiting…", buffer);
}
