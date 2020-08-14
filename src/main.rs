use komodo_rpc_client::{Client, Chain, KomodoRpcApi, TransactionId};
use std::path::Path;
use std::fs::File;
use std::io::{self, BufRead, BufReader};
use std::thread::sleep;
use std::time::Duration;

fn main() {
    let client = Client::new_assetchain_client(&Chain::Custom(String::from("ILN")))
        .expect("a client was expected");

    airdrop_fixed_amount(Path::new("./08-50.txt"), 14.59636933);
}

fn airdrop_fixed_amount(addies: &Path, amount: f64) -> io::Result<()> {
    let file = File::open(addies)?;
    let reader = BufReader::new(file);

    let vec = reader
        .lines()
        .map(|line| line.unwrap() )
        .collect::<Vec<String>>();

    let client = Client::new_assetchain_client(&Chain::Custom(String::from("ILN")))
        .expect("a client was expected");

    for chunk in vec.chunks(777) {
        let tx = create_and_send(chunk, amount);
        dbg!(&tx);
        let mut raw_tx = client.get_raw_transaction_verbose(TransactionId::from_hex(&tx.be_hex_string()).unwrap());

        sleep(Duration::from_secs(2));

        while raw_tx.unwrap().confirmations.is_none() {
            println!("no confirmation, wait 10 sec");
            sleep(Duration::from_secs(10));
            raw_tx = client.get_raw_transaction_verbose(TransactionId::from_hex(&tx.be_hex_string()).unwrap());
            sleep(Duration::from_secs(2));
        }
    }

    Ok(())
}

fn create_and_send(chunk: &[String], amount: f64) -> TransactionId {
    let mut sendmany = komodo_rpc_client::arguments::SendManyAmounts::new();
    for addy in chunk {
        let address = komodo_rpc_client::arguments::address::Address::from(addy).unwrap();
        sendmany.add(&address.to_string(), amount);
    }

    let client = Client::new_assetchain_client(&Chain::Custom(String::from("ILN")))
        .expect("a client was expected");

    let tx = client.send_many(sendmany, None, None, None);

    tx.unwrap()
}

