use std::collections::HashMap;
use crate::client::Client;
use crate::tea::qqtea_decrypt;

pub trait TlvDecoder {
    fn decode_t161(&mut self, data: &[u8]);
    fn decode_t119(&mut self, data: &[u8], ek: &[u8]);
    fn decode_t119r(&mut self, data: &[u8]);
    fn decode_t130(&mut self, data: &[u8]);
    fn decode_t113(&mut self, data: &[u8]);
    fn decode_t186(&mut self, data: &[u8]);
}

impl TlvDecoder for Client {
    fn decode_t161(&mut self, data: &[u8]) {
        todo!()
    }

    fn decode_t119(&mut self, data: &[u8], ek: &[u8]) {
        qqtea_decrypt(data,ek);
        todo!()
    }

    fn decode_t119r(&mut self, data: &[u8]) {
        todo!()
    }

    fn decode_t130(&mut self, data: &[u8]) {
        todo!()
    }

    fn decode_t113(&mut self, data: &[u8]) {
        todo!()
    }

    fn decode_t186(&mut self, data: &[u8]) {
        todo!()
    }
}


fn read_t125(data: &[u8]) -> (Vec<u8>, Vec<u8>) {
    todo!()
}

fn read_t11a(data: &[u8]) -> (String, u16, u16) {
    todo!()
}

fn read_t199(data: &[u8]) -> (Vec<u8>, Vec<u8>) {
    todo!()
}

fn read_t200(data: &[u8]) -> (Vec<u8>, Vec<u8>) {
    todo!()
}

fn read_t512(data: &[u8]) -> (HashMap<String, Vec<u8>>, HashMap<String, Vec<u8>>) {
    todo!()
}

fn read_t531(data: &[u8]) -> (Vec<u8>, Vec<u8>) {
    todo!()
}