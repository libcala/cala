// Networking example

use std::sync::{Mutex, Condvar, Arc};

use cala::*;
use net::{TcpServer, TcpConnection, ServerEvent};

cala::start!(init);

fn init() {
    let pair = Arc::new((Mutex::new(false), Condvar::new()));
    let pair2 = pair.clone();

    let s = move || { async move { server(pair).await } };
    let c = move || { async move { client(pair2).await } };

    Page::new()
        .spawn(s)
        .spawn(c)
        .join()
}

async fn server(pair: Arc<(Mutex<bool>, Condvar)>) {
    println!("Starting Server…");
    let mut server = TcpServer::new("127.0.0.1:21135").unwrap();
    
    let (lock, cvar) = &*pair;
    {
        let mut started = lock.lock().unwrap();
        *started = true;
        cvar.notify_one();
    }

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
    println!("Waiting for server…");

    let (lock, cvar) = &*pair;
    let mut started = lock.lock().unwrap();
    while !*started {
        started = cvar.wait(started).unwrap();
    }

    println!("Starting Client…");
    let mut client = TcpConnection::new("127.0.0.1:21135").unwrap();

    client.send(true, &[42]).await;

    let mut buffer = Vec::new();
    client.recv(&mut buffer).await;

    println!("Received {:?}, exiting…", buffer);
}
