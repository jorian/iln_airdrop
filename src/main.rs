use komodo_rpc_client::{Client, Chain, KomodoRpcApi, TransactionId};
use std::path::Path;
use std::fs::File;
use std::io::{self, BufRead, BufReader};
use std::thread::sleep;
use std::time::Duration;
use komodo_rpc_client::arguments::{
    address::Address,
    SendManyAmounts
};

fn main() {
    let client = Client::new_assetchain_client(&Chain::Custom(String::from("MORTY")))
        .expect("a client was expected");
    let file_name = Path::new("data/08-0.50.txt");
    let amount = 0.012;

    if let Ok(vec) = read_addresses_from_file(file_name) {
        for chunk in vec.chunks(777) {
            let txid = create_and_send(&client, chunk, amount);
            let mut raw_tx = client.get_raw_transaction_verbose(TransactionId::from_hex(&txid.be_hex_string()).unwrap());

            sleep(Duration::from_secs(2));

            while raw_tx.unwrap().confirmations.is_none() {
                println!("no confirmation, wait 10 sec");
                sleep(Duration::from_secs(10));
                raw_tx = client.get_raw_transaction_verbose(TransactionId::from_hex(&txid.be_hex_string()).unwrap());
                // need to sleep to prevent Hyper IncompleteMessage errors
                sleep(Duration::from_secs(2));
            }
        }
    } else {
        println!{"Something went wrong while reading the file"}
    }
}

fn read_addresses_from_file(addresses: &Path) -> io::Result<Vec<Address>>  {
    let file = File::open(addresses)?;
    let reader = BufReader::new(file);

    let mut vec = vec![];

    reader
        .lines()
        .for_each(|line| {
            let str_add = line.unwrap();
            match Address::from(&str_add) {
                Ok(address) => vec.push(address),
                Err(err) => println!("error parsing address {}: {}", &str_add, err.to_string())
            }
        });

    Ok(vec)
}

fn create_and_send(client: &Client, chunk: &[Address], amount: f64) -> TransactionId {
    let mut sendmany = SendManyAmounts::new();
    for addy in chunk {
        sendmany.add(&addy.to_string(), amount);
    }

    let tx = client.send_many(sendmany, None, None, None);

    tx.unwrap()
}

#[cfg(test)]
mod tests {
    use std::path::Path;
    use crate::read_addresses_from_file;
    use komodo_rpc_client::arguments::address::Address;

    #[test]
    fn it_works() {
        assert_eq!(2 + 2, 4);
    }

    #[test]
    fn valid_addresses() {
        let path = Path::new("data/test_valid.txt");
        let vec = read_addresses_from_file(path);

        let valid_addresses = vec!{
            Address::from("RAMVr4wrArBMM4j1J5gmCTiE5zvpBR9L3V").unwrap(),
            Address::from("RHQt6RRAKgzdvZxSPH5CxLNC9zmaN7ARvC").unwrap(),
            Address::from("RVFh5H8HuaBvAYVngoSoYPijjUvpXjsq1e").unwrap()
        };

        assert_eq!(valid_addresses, vec.unwrap());
    }
}