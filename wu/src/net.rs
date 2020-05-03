//! Network utils

use crate::crypto::{random, Crypter};
use aes_gcm::Aes256Gcm;
use kern::Fail;
use std::convert::TryInto;
use std::io::prelude::*;
use std::net::{TcpStream, ToSocketAddrs};

/// Connection builder
pub struct ConnBuilder<'a> {
    stream: TcpStream,
    aead: &'a Aes256Gcm,
}

impl<'a> ConnBuilder<'a> {
    /// Create new connection builder
    pub fn new(addr: impl ToSocketAddrs, aead: &'a Aes256Gcm) -> Result<Self, Fail> {
        let stream = TcpStream::connect(addr).or_else(Fail::from)?;
        Ok(Self { stream, aead })
    }

    /// Create new connection builder from TcpStream
    pub fn from(stream: TcpStream, aead: &'a Aes256Gcm) -> Self {
        Self { stream, aead }
    }

    /// Connection initiator
    pub fn init(mut self) -> Result<Connection<'a>, Fail> {
        // generate and write nonce
        let nonce = random(12);
        self.stream.write_all(&nonce).or_else(Fail::from)?;

        // initiate crypter and return connection
        let crypt = Crypter::new(self.aead, &nonce);
        Ok(Connection {
            stream: self.stream,
            crypt,
        })
    }

    /// Connection acceptor
    pub fn accept(mut self) -> Result<Connection<'a>, Fail> {
        // read nonce
        let mut nonce = vec![0u8; 12];
        self.stream.read_exact(&mut nonce).or_else(Fail::from)?;

        // initiate crypter and return connection
        let crypt = Crypter::new(self.aead, &nonce);
        Ok(Connection {
            stream: self.stream,
            crypt,
        })
    }
}

/// Encrypted connection
pub struct Connection<'a> {
    stream: TcpStream,
    crypt: Crypter<'a>,
}

impl<'a> Connection<'a> {
    /// Reinitiate connection
    pub fn reinit(&'a mut self) -> Result<Connection<'a>, Fail> {
        let addr = self.stream.peer_addr().or_else(Fail::from)?;
        ConnBuilder::new(addr, self.crypt.aead())?.init()
    }

    /// Encrypt and write data
    pub fn write(&mut self, data: impl AsRef<[u8]>) -> Result<(), Fail> {
        // encrypt and write length
        let enc = self.crypt.encrypt(data.as_ref())?;
        self.stream
            .write_all(&enc.len().to_be_bytes())
            .or_else(Fail::from)?;

        // write encrypted data
        self.stream.write_all(&enc).or_else(Fail::from)
    }

    /// Read and decrypt data
    pub fn read(&mut self) -> Result<Vec<u8>, Fail> {
        // read length
        let mut len_buf = vec![0u8; 8];
        self.stream.read_exact(&mut len_buf).or_else(Fail::from)?;
        let len = usize::from_be_bytes(len_buf.as_slice().try_into().or_else(Fail::from)?);

        // read encrypted data
        let mut buf = vec![0u8; len];
        self.stream.read_exact(&mut buf).or_else(Fail::from)?;

        // decrypt data and return
        self.crypt.decrypt(buf)
    }

    /// Get IP address of TcpStream
    pub fn stream_ip(&self) -> String {
        self.stream.peer_addr().unwrap().ip().to_string()
    }

    /// Get Crypter AEAD
    pub fn crypter_aead(&self) -> &Aes256Gcm {
        self.crypt.aead()
    }
}
