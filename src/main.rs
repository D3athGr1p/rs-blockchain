mod block;
mod blockchain;
mod transaction;

use std::{thread::sleep, time::Duration};

use blockchain::Blockchain;

const BLOCKTIME: u64 = 1; // 1 second
const MAX_COUNT: usize = 3;

fn main() {
    let mut blockchain = Blockchain::new();

    let mut amount = 1000;
    let mut count = 0;

    let (mut sender, mut recipient) = ("alice".to_owned(), "bob".to_owned());
    while count < MAX_COUNT {
        blockchain.create_transaction(sender.clone(), recipient.clone(), amount);
        let block = blockchain.create_new_block();
        if !block.validate(blockchain.difficulty).unwrap() {
            eprintln!("Invalid Block found");
            let block = blockchain.chain.pop();
            blockchain.pending_transactions = block.unwrap().transactions;
        }

        (sender, recipient) = (recipient, sender);
        amount /= 2;

        sleep(Duration::from_secs(BLOCKTIME));
        count += 1;
    }

    blockchain.chain.last().unwrap().print_block_json();
}
