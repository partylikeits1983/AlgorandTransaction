use algonaut::core::{Address, MicroAlgos};
use algonaut::transaction::{BaseTransaction, Payment, Transaction, TransactionType};
use algonaut::{Algod, Kmd};
use std::error::Error;

fn main() -> Result<(), Box<dyn Error>> {
    let kmd = Kmd::new()
        .bind("http://localhost:4002")
        .auth("aaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaa")
        .client_v1()?;


        let list_response = kmd.list_wallets()?;

        let wallet_id = match list_response
            .wallets
            .into_iter()
            .find(|wallet| wallet.name == "unencrypted-default-wallet")
        {
            Some(wallet) => wallet.id,
            None => return Err("Wallet not found".into()),
        };
        println!("Wallet: {}", wallet_id);

        let init_response = kmd.init_wallet_handle(&wallet_id, "")?;
        let wallet_handle_token = init_response.wallet_handle_token;

        let algod = Algod::new()
            .bind("http://localhost:4001")
            .auth("aaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaa")
            .client_v1()?;

        let transaction_params = algod.transaction_params()?;
        let genesis_id = transaction_params.genesis_id;
        let genesis_hash = transaction_params.genesis_hash;

        let public_key = "ISNOEZ4DNDB5PZGY67QR7DBGGFT3JWVQUJPYNVOO6EUHX5RULJHU66ANEQ";
        let to_address = Address::from_string(public_key.as_ref())?;
        let from_address = Address::from_string(public_key.as_ref())?;
        println!("Receiver: {:#?}", to_address);

        let base = BaseTransaction {
            sender: from_address,
            first_valid: transaction_params.last_round,
            last_valid: transaction_params.last_round + 1000,
            note: Vec::new(),
            genesis_id,
            genesis_hash,
        };
        println!("Base: {:#?}", base);


        let payment = Payment {
            amount: MicroAlgos(100_000),
            receiver: to_address,
            close_remainder_to: None,
        };
        println!("Payment: {:#?}", payment);


        let transaction =
            Transaction::new_flat_fee(base, MicroAlgos(1_000), TransactionType::Payment(payment));
        println!("Transaction: {:#?}", transaction);


        let sign_response = kmd.sign_transaction(&wallet_handle_token, "", &transaction)?;
        println!("Signed: {:#?}", sign_response);


        let send_response = algod.raw_transaction(&sign_response.signed_transaction)?;
        println!("Transaction ID: {}", send_response.tx_id);

        Ok(())
    }
