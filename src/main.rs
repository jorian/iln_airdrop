use komodo_rpc_client::{Client, Chain, KomodoRpcApi};
use std::path::Path;
use std::fs::File;
use std::io::{self, BufRead, BufReader};

fn main() {
    let client = Client::new_assetchain_client(&Chain::Custom(String::from("ILN")))
        .expect("an ILN client was expected");

    println!("{}", client.get_info().unwrap().balance);
    airdrop_fixed_amount(Path::new("./testdata.txt"), 0.2);
}

fn airdrop_fixed_amount(addies: &Path, amount: f32) -> io::Result<()> {
    let file = File::open(addies)?;
    let reader = BufReader::new(file);

    let vec = reader
        .lines()
        .map(|line| line.unwrap() )
        .collect::<Vec<String>>();

    // println!("{:?}", vec);

    vec.chunks(777).for_each(|addy| println!("{:?}", addy));

    // read addies
    // take chunks of 777
    //prepare sendmany
    // send sendmany
    Ok(())
}

// data is a csv
fn airdrop_csv(data: &Path) {

}

fn create_send_many() {

}

