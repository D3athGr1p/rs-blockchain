mod block;
mod blockchain;
mod transaction;

use blockchain::Blockchain;

fn main() {
    let mut blockchain = Blockchain::new();

    let (mut sender, mut recipient) = ("alice".to_owned(), "bob".to_owned());
    let mut amount = 1000;

    blockchain.create_transaction(sender.clone(), recipient.clone(), amount);
    blockchain.create_new_block();

    (sender, recipient) = (recipient, sender);
    amount /= 10;

    blockchain.create_transaction(sender, recipient, amount);
    blockchain.create_new_block();

    println!("{:?}", blockchain);
}
