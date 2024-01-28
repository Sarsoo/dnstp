use std::net::{SocketAddr, UdpSocket};
use std::thread;
use std::thread::{JoinHandle};
use log::{error, info};

use std::str;
use std::sync::mpsc;
use std::sync::mpsc::{Receiver, Sender, TryRecvError};

pub struct DNSSocket {
    addresses: Vec<SocketAddr>,
    thread: Option<JoinHandle<()>>,
    thread_killer: Option<Sender<()>>
}

impl DNSSocket {
    pub fn new(addresses: Vec<SocketAddr>) -> DNSSocket
    {
        DNSSocket {
            addresses,
            thread: None,
            thread_killer: None
        }
    }

    // pub fn new<T, U: ToSocketAddrs>(addr: T) -> DNSSocket
    // where
    //     T: Iterator<Item = U>,
    // {
    //     DNSSocket {
    //         addresses: addr
    //             .map(|x| x.parse().expect("Couldn't parse address"))
    //             .collect()
    //     }
    // }

    fn bind(&mut self) -> Option<UdpSocket>
    {
        match UdpSocket::bind(&self.addresses[..]) {
            Ok(s) => Option::from(s),
            Err(e) => None
        }
    }

    pub fn run(&mut self)
    {
        let (tx, rx): (Sender<()>, Receiver<()>) = mpsc::channel();
        self.thread_killer = Some(tx);
        let socket = self.bind();

        self.thread = Some(thread::spawn(move || {

            match socket {
                None => {
                    error!("no socket created, failed to bind to address")
                }
                Some(s) => {
                    let mut cancelled = false;
                    while !cancelled {
                        let mut buf = [0; 512];
                        let res = s.recv_from(&mut buf);

                        match res {
                            Ok(r) => {
                                let res_str = str::from_utf8(&buf).unwrap();
                                info!("received: {}", res_str);
                            }
                            Err(_) => {}
                        }

                        cancelled = match rx.try_recv() {
                            Ok(_) | Err(TryRecvError::Disconnected) => true,
                            _ => false
                        }
                    }
                }
            };

            info!("socket thread finishing")
        }));
    }

    pub fn stop(&mut self)
    {
        // if let Some(t) = &mut self.thread {
            if let Some(k) = &self.thread_killer {
                k.send(());
                // t.join();
            }
        // }
    }
}