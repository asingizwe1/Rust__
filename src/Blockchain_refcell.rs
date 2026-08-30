//INTERIOR MUTABILITY
//with refcell we can mutate the chain(push new blocks) even if we only have a shared reference
use std::cell::RefCell;

#[derive(Debug)]
struct Block {
    index: usize,
    data: String,
}

#[derive(Debug)]
struct Blockchain {
    blocks: RefCell<Vec<Block>>, //interior mutability
}

impl Blockchain {
    fn new() -> Self {
        Blockchain {
            blocks: RefCell::new(Vec::new()),
        }
    }

    fn add_block(&self, data: String) {
        let mut chain = self.blocks.borrow_mut(); //runtime mutable borrow
        let index = chain.len();
        chain.push(Block { index, data })
    }
    fn print_block(&self) {
        for block in self.blocks.borrow().iter() {
            //immutable borrow
            println!("Block {}: {}", block.index, block.data);
        }
    }
}

fn main() {
    let blockchain = Blockchain::new();

    blockchain.add_block("Genesis Block".to_string());
    blockchain.add_block("Transaction: Louis -> Job".to_string());
    blockchain.add_block("Transaction: Job -> Klip".to_string());

    blockchain.print_block();
}
