#![feature(proc_macro_hygiene, decl_macro)]

#[macro_use]
extern crate rocket;
#[macro_use]
extern crate rocket_contrib;

mod block;
mod blockchain;
mod transaction;

use rocket::serde::json::{json, Value};
use rocket::State;
use rocket_contrib::json::Json;

use std::{
    sync::{Arc, Mutex},
    thread,
    time::Duration,
};

use blockchain::Blockchain;

const BLOCKTIME: u64 = 1; // 1 second

#[post("/transaction", data = "<tx>")]
fn create_transaction(
    blockchain: &State<Arc<Mutex<Blockchain>>>,
    tx: Json<transaction::Transaction>,
) -> Value {
    let mut blockchain = blockchain.lock().unwrap();
    blockchain.create_transaction(tx.sender.clone(), tx.recipient.clone(), tx.amount);
    json!({"message": "Transaction added to pending transactions"})
}

fn main() {
    let blockchain = Arc::new(Mutex::new(Blockchain::new()));

    rocket::build()
        .manage(blockchain.clone())
        .mount("/", routes![create_transaction])
        .launch();

    let create_block = Arc::clone(&blockchain);

    let create_block_handle = thread::spawn(move || {
        let mut blockchain = create_block.lock().unwrap();

        loop {
            let block = blockchain.create_new_block();

            if !block.validate(blockchain.difficulty).unwrap() {
                eprintln!("Invalid Block found");
                let block = blockchain.chain.pop();
                blockchain.pending_transactions = block.unwrap().transactions;
            }
            blockchain.chain.last().unwrap().print_block_json();
            thread::sleep(Duration::from_secs(BLOCKTIME));
        }
    });

    create_block_handle.join().unwrap();
}
