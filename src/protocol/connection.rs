use std::collections::{HashMap, VecDeque};
use std::os::unix::net::UnixStream;
use std::sync::atomic::{AtomicU32, Ordering};
use std::sync::{Arc, Mutex};

use super::channel::Channel;
use super::packet::{read_packet, write_packet, Packet};

/// A connection to the Hegel server.
pub struct Connection {
    stream: Mutex<UnixStream>,
    /// Packets that arrived for channels other than the one being processed
    pending_packets: Mutex<HashMap<u32, VecDeque<Packet>>>,
    next_channel_id: AtomicU32,
    channels: Mutex<HashMap<u32, ()>>, // Track which channels exist
}

impl Connection {
    /// Create a new connection from a Unix stream.
    pub fn new(stream: UnixStream) -> Arc<Self> {
        Arc::new(Self {
            stream: Mutex::new(stream),
            pending_packets: Mutex::new(HashMap::new()),
            next_channel_id: AtomicU32::new(1), // 0 is reserved for control
            channels: Mutex::new(HashMap::new()),
        })
    }

    /// Get the control channel (channel 0).
    pub fn control_channel(self: &Arc<Self>) -> Channel {
        Channel::new(0, Arc::clone(self))
    }

    /// Create a new client-side channel with an odd ID (3, 5, 7...).
    pub fn new_channel(self: &Arc<Self>) -> Channel {
        let next = self.next_channel_id.fetch_add(1, Ordering::SeqCst);
        // Client channels use odd IDs: (next << 1) | 1 gives 3, 5, 7, ...
        let channel_id = (next << 1) | 1;
        self.channels.lock().unwrap().insert(channel_id, ());
        Channel::new(channel_id, Arc::clone(self))
    }

    /// Connect to an existing channel (created by the other side).
    pub fn connect_channel(self: &Arc<Self>, channel_id: u32) -> Channel {
        self.channels.lock().unwrap().insert(channel_id, ());
        Channel::new(channel_id, Arc::clone(self))
    }

    /// Send a packet.
    pub fn send_packet(&self, packet: &Packet) -> std::io::Result<()> {
        let mut stream = self.stream.lock().unwrap();
        write_packet(&mut *stream, packet)
    }

    /// Receive a packet for a specific channel.
    /// If a packet for a different channel arrives, it's queued for later.
    pub fn receive_packet_for_channel(&self, channel_id: u32) -> std::io::Result<Packet> {
        // First check pending packets
        {
            let mut pending = self.pending_packets.lock().unwrap();
            if let Some(queue) = pending.get_mut(&channel_id) {
                if let Some(packet) = queue.pop_front() {
                    return Ok(packet);
                }
            }
        }

        // Read from stream until we get a packet for our channel
        loop {
            let packet = {
                let mut stream = self.stream.lock().unwrap();
                read_packet(&mut *stream)?
            };

            if packet.channel == channel_id {
                return Ok(packet);
            }

            // Queue for another channel
            let mut pending = self.pending_packets.lock().unwrap();
            pending.entry(packet.channel).or_default().push_back(packet);
        }
    }

    /// Close the connection.
    pub fn close(&self) -> std::io::Result<()> {
        let stream = self.stream.lock().unwrap();
        stream.shutdown(std::net::Shutdown::Both)
    }
}
