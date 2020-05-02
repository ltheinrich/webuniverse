//! Network utils

use crate::crypto::{random, Crypter};
use aes_gcm::Aes256Gcm;
use kern::Fail;
use std::convert::TryInto;
use std::io::prelude::*;
use std::net::TcpStream;

pub struct ConnBuilder<'a> {
    stream: TcpStream,
    aead: &'a Aes256Gcm,
}

impl<'a> ConnBuilder<'a> {
    pub fn new(stream: TcpStream, aead: &'a Aes256Gcm) -> Self {
        Self { stream, aead }
    }

    pub fn init(mut self) -> Result<Connection<'a>, Fail> {
        let nonce = random(12);
        let crypt = Crypter::new(self.aead, &nonce);
        self.stream.write_all(&nonce).or_else(Fail::from)?;
        Ok(Connection {
            stream: self.stream,
            crypt,
        })
    }

    pub fn accept(mut self) -> Result<Connection<'a>, Fail> {
        let mut nonce = vec![0u8; 12];
        self.stream.read_exact(&mut nonce).or_else(Fail::from)?;
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
    /// Encrypt and write data
    pub fn write(&mut self, data: impl AsRef<[u8]>) -> Result<(), Fail> {
        let enc = self.crypt.encrypt(data.as_ref())?;
        self.stream
            .write_all(&enc.len().to_be_bytes())
            .or_else(Fail::from)?;
        self.stream.write_all(&enc).or_else(Fail::from)
    }

    /// Read and decrypt data
    pub fn read(&mut self) -> Result<Vec<u8>, Fail> {
        let mut len_buf = vec![0u8; 8];
        self.stream.read_exact(&mut len_buf).or_else(Fail::from)?;
        let len = usize::from_be_bytes(len_buf.as_slice().try_into().or_else(Fail::from)?);
        let mut buf = vec![0u8; len];
        self.stream.read_exact(&mut buf).or_else(Fail::from)?;
        self.crypt.decrypt(buf)
    }
}
