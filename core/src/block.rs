use utils::coder;
use chrono::prelude::*;
use serde::{Serialize};

#[derive(Serialize,Debug)]
pub struct BlockHeader{
    pub timestamp: i64,      //时间戳
    pub tx_hash: String,     //merkel tree的根hash值
    pub prev_hash: String,   //前一个区块的区块头的hash值
}

#[derive(Debug)]
pub struct Block {
    pub header: BlockHeader, //区块头
    pub hash: String,        //区块头的hash值
    pub data: String,        //交易数据
}

impl Block {
    fn print(&mut self) {
        println!("+++++++++++++++++++++++++++++++++++++++++");
        println!("{:#?}", self);
        println!("");
    }

    fn set_hash(&mut self) {
        let header_serialize = coder::my_serialize(&(self.header));
        self.hash = coder::get_hash(&header_serialize[..]);
    }

    pub fn new_block(data: String, prev_hash: String) -> Self {
        let tx_serialze = coder::my_serialize(&data);
        let new_header = BlockHeader { 
            timestamp: Utc::now().timestamp(), 
            tx_hash: coder::get_hash(&tx_serialze[..]), 
            prev_hash: prev_hash };

        let mut block = Block {
            header: new_header,
            hash: "".to_string(),
            data: data,
        };
        block.set_hash();
        block.print();
        block
    }
}