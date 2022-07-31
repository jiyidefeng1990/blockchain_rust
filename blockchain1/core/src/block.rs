pub struct BlockHeader{
    pub timestamp: i64,      //时间戳
    pub tx_hash: String,     //merkel tree的根hash值
    pub prev_hash: String,   //前一个区块的区块头的hash值
}

pub struct Block {
    pub header: BlockHeader, //区块头
    pub hash: String,        //区块头的hash值
    pub data: String,        //交易数据
}