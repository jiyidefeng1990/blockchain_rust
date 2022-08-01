use crate::block;

pub struct Blockchain {
    pub blocks: Vec<block::Block>,
}

impl Blockchain {
    pub fn add(&mut self, data:String) {
        let block = block::Block::new_block(data,self.blocks[self.blocks.len()-1].hash.clone());
        self.blocks.push(block);
    }

    fn new_genisis_block() -> block::Block{
        block::Block::new_block("This is a genisis block".to_string(), String::from(""))
    }

    pub fn new_blockchain() -> Self{
        Blockchain{blocks:vec![Self::new_genisis_block()]}
    }
}