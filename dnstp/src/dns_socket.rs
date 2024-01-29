use std::net::{SocketAddr, UdpSocket};
use std::ptr::read;
use std::thread;
use std::thread::{JoinHandle};
use log::{debug, error, info};

use std::str;
use std::sync::mpsc;
use std::sync::mpsc::{Receiver, Sender, TryRecvError};
use crate::dns_header::HEADER_SIZE;
use crate::raw_request::{NetworkMessage, NetworkMessagePtr};

pub struct DNSSocket {
    addresses: Vec<SocketAddr>,
    socket: Option<Box<UdpSocket>>,
    rx_thread: Option<JoinHandle<()>>,
    rx_thread_killer: Option<Sender<()>>,
    tx_thread: Option<JoinHandle<()>>,
    tx_message_channel: Option<Sender<NetworkMessagePtr>>,
    tx_thread_killer: Option<Sender<()>>
}

impl DNSSocket {
    pub fn new(addresses: Vec<SocketAddr>) -> DNSSocket
    {
        DNSSocket {
            addresses,
            socket: None,
            rx_thread: None,
            rx_thread_killer: None,
            tx_thread: None,
            tx_message_channel: None,
            tx_thread_killer: None
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

    pub fn bind(&mut self)
    {
        match UdpSocket::bind(&self.addresses[..]) {
            Ok(s) => {
                self.socket = Option::from(Box::from(s));
            },
            Err(_) => {}
        };
    }

    fn get_socket_clone(&mut self) -> Option<Box<UdpSocket>>
    {
        match &self.socket {
            Some(s) => Option::from(Box::from(s.try_clone().unwrap())),
            None => None
        }
    }

    pub fn run_rx(&mut self, message_sender: Sender<NetworkMessagePtr>)
    {
        let (tx, rx): (Sender<()>, Receiver<()>) = mpsc::channel();
        self.rx_thread_killer = Some(tx);
        let socket = self.get_socket_clone();

        self.rx_thread = Some(thread::spawn(move || {

            match socket {
                None => {
                    error!("no socket created, failed to bind to address")
                }
                Some(s) => {
                    let mut cancelled = false;
                    while !cancelled {
                        let mut buf = Box::new([0; 512]);
                        let res = s.recv_from(&mut (*buf));

                        match res {
                            Ok((read_count, peer)) => {
                                let res_str = str::from_utf8(&(*buf)).unwrap();
                                info!("received [{}] from [{}]", res_str, peer);

                                if read_count > HEADER_SIZE {
                                    message_sender.send(Box::new(NetworkMessage {
                                        buffer: buf,
                                        peer
                                    }));
                                }
                                else {
                                    debug!("skipping processing message from [{}], message isn't longer than standard header", peer);
                                }
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

            info!("socket rx thread finishing")
        }));
    }

    pub fn run_tx(&mut self)
    {
        let (tx, rx): (Sender<()>, Receiver<()>) = mpsc::channel();
        self.tx_thread_killer = Some(tx);

        let (msg_tx, msg_rx): (Sender<NetworkMessagePtr>, Receiver<NetworkMessagePtr>) = mpsc::channel();
        self.tx_message_channel = Option::from(msg_tx);

        let socket = self.get_socket_clone();

        self.tx_thread = Some(thread::spawn(move || {

            match socket {
                None => {
                    error!("no socket created, failed to bind to address")
                }
                Some(s) => {
                    let mut cancelled = false;
                    while !cancelled {

                        for m in &msg_rx {
                            info!("sending [{}] to [{}]", str::from_utf8(&(*(*m).buffer)).unwrap(), (*m).peer);
                            if let Err(e) = s.send_to(&(*m.buffer), m.peer){
                                error!("error sending response to [{}], {}", m.peer, e);
                            }
                        }

                        cancelled = match rx.try_recv() {
                            Ok(_) | Err(TryRecvError::Disconnected) => true,
                            _ => false
                        }
                    }
                }
            }

            info!("socket tx thread finishing")
        }));
    }

    pub fn get_tx_message_channel(&mut self) -> Option<Sender<NetworkMessagePtr>>
    {
        self.tx_message_channel.clone()
    }

    pub fn stop(&mut self)
    {
        // if let Some(t) = &mut self.thread {
            if let Some(k) = &self.rx_thread_killer {
                k.send(());
                // t.join();
            }
            if let Some(k) = &self.tx_thread_killer {
                k.send(());
                // t.join();
            }
        // }
    }
}