pub mod info;
pub mod players;
pub mod rules;
use crate::errors::Result;
use std::io::{Cursor, Read};

pub use crate::types::{
    info::{ExtendedServerInfo, Info, INFO_REQUEST},
    players::{Player, TheShipPlayer, PLAYER_REQUEST},
    rules::{Rule, RULES_REQUEST},
};

trait ReadCString {
    fn read_cstring(&mut self) -> Result<String>;
}

impl ReadCString for Cursor<Vec<u8>> {
    fn read_cstring(&mut self) -> Result<String> {
        let end = self.get_ref().len() as u64;
        let mut buf = [0; 1];
        let mut str_vec = Vec::with_capacity(256);
        while self.position() < end {
            self.read_exact(&mut buf)?;
            if buf[0] == 0 {
                break;
            } else {
                str_vec.push(buf[0]);
            }
        }
        Ok(String::from_utf8_lossy(&str_vec[..]).into_owned())
    }
}
