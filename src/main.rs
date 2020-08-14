use komodo_rpc_client::{Client, Chain, KomodoRpcApi, TransactionId};
use std::path::Path;
use std::fs::File;
use std::io::{self, BufRead, BufReader};
use std::thread::sleep;
use std::time::Duration;

fn main() {
    let client = Client::new_assetchain_client(&Chain::Custom(String::from("MORTY")))
        .expect("a client was expected");
    let file_name = Path::new("data/08-0.50.txt");
    let amount = 0.012;

    if let Ok(vec) = read_addresses_file(file_name) {
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
    }
}

fn read_addresses_file(addresses: &Path) -> io::Result<Vec<String>>  {
    let file = File::open(addresses)?;
    let reader = BufReader::new(file);

    Ok(reader
        .lines()
        .filter_map(Result::ok) // TODO make sure this is a valid address
        .collect::<Vec<String>>())
}

fn create_and_send(client: &Client, chunk: &[String], amount: f64) -> TransactionId {
    let mut sendmany = komodo_rpc_client::arguments::SendManyAmounts::new();
    for addy in chunk {
        let address = komodo_rpc_client::arguments::address::Address::from(addy).unwrap();
        sendmany.add(&address.to_string(), amount);
    }

    let tx = client.send_many(sendmany, None, None, None);

    tx.unwrap()
}

#[cfg(test)]
mod tests {
    use std::path::Path;
    use crate::read_addresses_file;

    #[test]
    fn it_works() {
        assert_eq!(2 + 2, 4);
    }

    #[test]
    fn valid_addresses() {
        let path = Path::new("data/test_valid.txt");
        let vec = read_addresses_file(path);

        let valid_addresses = vec!{
            String::from("RAMVr4wrArBMM4j1J5gmCTiE5zvpBR9L3V"),
            String::from("RHQt6RRAKgzdvZxSPH5CxLNC9zmaN7ARvC"),
            String::from("RVFh5H8HuaBvAYVngoSoYPijjUvpXjsq1e")
        };

        assert_eq!(valid_addresses, vec.unwrap());
    }

    #[test]
    fn invalid_addresses() {
        let path = Path::new("data/test_invalid.txt");
        let vec = read_addresses_file(path);

        assert!(vec.is_err());
    }
}