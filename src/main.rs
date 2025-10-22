use solana_client::rpc_client::RpcClient;
use solana_pubkey::Pubkey;
use solana_client::rpc_client::GetConfirmedSignaturesForAddress2Config;
use solana_transaction_status::{
    EncodedConfirmedTransactionWithStatusMeta, EncodedTransaction, UiInstruction, UiParsedInstruction,
    UiTransactionEncoding,
};
use solana_transaction_status::option_serializer::OptionSerializer;

use solana_signature::Signature;
use std::collections::BTreeMap;


use std::str::FromStr;
use std::io;
use chrono::{DateTime, Utc, NaiveDate};




pub fn main() {
    let rpc_client = RpcClient::new("https://api.mainnet-beta.solana.com");

    const LAMPORTS_PER_SOL: u64 = 1000000000; // 0x3B9ACA00
println!("######################GET ACCOUNT BALANCE#########################");
    // Get the account you want to query from the user input
    println!("Enter the account you want to query: ");
    let mut account_queried = String::new();
    io::stdin().read_line(&mut account_queried).unwrap();
    let account_queried = account_queried.trim().to_owned(); 

    // Get the balance of the account
    let balance = rpc_client
      .get_account(&Pubkey::from_str(&account_queried).unwrap())
      .unwrap().lamports;
    println!("This account has: {} lamport", balance);
    let sol_lamport: f64 = balance as f64 / LAMPORTS_PER_SOL as f64;

    println!("Which is {} in solana", sol_lamport);



//######################GET FIRST SIGNATURE#########################
println!("######################GET FIRST SIGNATURE#########################");
    // Now lets go back in time
    println!("Let's see when was the first ever transfer to this account");
    let account_pubkey = Pubkey::from_str(&account_queried).unwrap();
    let mut cfg = GetConfirmedSignaturesForAddress2Config {
        limit: Some(1000),
        ..Default::default()
    };
    let mut signatures = rpc_client.get_signatures_for_address_with_config(&account_pubkey, cfg).unwrap();
    println!("to start off, this is the latest signature: {}", signatures[0].signature);
    let latest_signature = &signatures[0].signature;
    let latest_signature_block_time = signatures[0].block_time.unwrap();
    let latest_signature_date = DateTime::from_timestamp(latest_signature_block_time, 0).unwrap_or_default();
    println!("the latest signature was at: {}", latest_signature_date);
    println!("let's get the first signature!");

    // if there are more then 1000 signatures, we need to go back in time more to get the first signature
    // to do that, we just run the get_signatures_for_address_with_config again with the the last signature as the before signature
    // so that it will get the signatures before that signature
    // if there are less then 1000 signatures, we have found the first signature

    loop {
        if signatures.len() == 1000 {
            println!("We need to go back in time more");

            if let Some(last_entry) = signatures.last() {
               let before = Some(Signature::from_str(&last_entry.signature).unwrap());
                signatures = rpc_client
                    .get_signatures_for_address_with_config(
                        &account_pubkey,
                        GetConfirmedSignaturesForAddress2Config {
                            before: before.clone(),
                            limit: Some(1000),
                            ..Default::default()
                        },
                    )
                    .unwrap();
            } else {
                break;
            }
        } else {
            if let Some(first_entry) = signatures.last() {
                println!("we have found the first signature!");
                println!("first signature: {}", first_entry.signature);
            }
            break;
        }
    }
    let first_signature = &signatures[signatures.len() - 1].signature;
    let first_signature_block_time = signatures[signatures.len() - 1].block_time.unwrap();
    let first_signature_date = DateTime::from_timestamp(first_signature_block_time, 0).unwrap_or_default();
    println!("the first signature was at: {}", first_signature_date);
    
    println!("######################MENU#########################");
    println!("1. Get info on a transaction at specific signature");
    println!("2. Get historical account balance");
    println!("3. Get account transactions at specific date");
    println!("4. Exit");
    let mut choice = String::new();
    io::stdin().read_line(&mut choice).unwrap();
    let choice = choice.trim().to_owned();
    match choice.as_str() {
        "1" => get_info_on_transaction_at_specific_signature(&rpc_client, LAMPORTS_PER_SOL),
        "2" => get_historical_account_balance(&rpc_client, &account_pubkey, LAMPORTS_PER_SOL),
        "3" => get_account_transactions_at_specific_date(&rpc_client, &account_pubkey, LAMPORTS_PER_SOL),
        "4" => exit(),
        _ => println!("Invalid choice"),
    }

}

fn describe_transfer(tx: &EncodedConfirmedTransactionWithStatusMeta, LAMPORTS_PER_SOL: u64) {
    let tx = match tx.transaction.transaction {
        EncodedTransaction::Json(ref json_tx) => json_tx,
        _ => return,
    };

    let message = match &tx.message {
        solana_transaction_status::UiMessage::Parsed(ref parsed) => parsed,
        _ => return,
    };

    for instr in &message.instructions {
        match instr {
            UiInstruction::Parsed(UiParsedInstruction::Parsed(parsed))
                if parsed.program == "system"
                    && parsed.parsed["type"] == "transfer" =>
            {
                if let (Some(source), Some(dest), Some(lamports)) = (
                    parsed.parsed["info"]["source"].as_str(),
                    parsed.parsed["info"]["destination"].as_str(),
                    parsed.parsed["info"]["lamports"].as_u64(),
                ) {
                    let sol_lamport: f64 = lamports as f64 / LAMPORTS_PER_SOL as f64;
                    println!("{source} → {dest} : {lamports} lamports ({sol_lamport} SOL)");
                }
            }
            _ => {}
        }
    }
}

pub fn get_info_on_transaction_at_specific_signature(rpc: &RpcClient, LAMPORTS_PER_SOL: u64) {
    println!("######################GET INFO ON TRANSACTION AT SPECIFIC SIGNATURE#########################");
    println!("Enter the signature you want to get info on: ");
    let mut signature = String::new();
    io::stdin().read_line(&mut signature).unwrap();
    let signature = signature.trim().to_owned();
    let signature = Signature::from_str(&signature).unwrap();
    let transaction = rpc.get_transaction(&signature, UiTransactionEncoding::JsonParsed).unwrap();
    println!("Here is what happened in that transaction :" );
    match rpc.get_transaction(&signature, UiTransactionEncoding::JsonParsed) {
        Ok(tx) => describe_transfer(&tx, LAMPORTS_PER_SOL),
        Err(err) => eprintln!("RPC error fetching transaction: {err}"),
    }
}



fn get_account_transactions_at_specific_date(rpc: &RpcClient, account: &Pubkey, LAMPORTS_PER_SOL: u64) {
    println!("######################GET ACCOUNT TRANSACTIONS AT SPECIFIC DATE#########################");
    println!("Enter date (YYYY-MM-DD):");
    let mut input = String::new();
    io::stdin().read_line(&mut input).unwrap();
    let input = input.trim();

    let naive_date = NaiveDate::parse_from_str(input, "%Y-%m-%d").unwrap();
    let start_ts = naive_date.and_hms(0, 0, 0).and_utc().timestamp();
    let end_ts = (naive_date + chrono::Duration::days(1))
        .and_hms(0, 0, 0)
        .and_utc()
        .timestamp();
    let mut before: Option<Signature> = None;
    loop {
        let cfg = GetConfirmedSignaturesForAddress2Config {
            before: before.clone(),
            limit: Some(1000),
            ..Default::default()
        };

        const MAX_RETRIES: u8 = 3;
        let mut attempt = 0;
       
    let signatures = match rpc.get_signatures_for_address_with_config(account, cfg) {
        Ok(s) => s,
        Err(err) => {
            eprintln!("RPC error: {err}");
            break;
        }
    };
        if signatures.is_empty() {
            break;
        }

        for item in &signatures {
            if let Some(ts) = item.block_time {
                if ts >= start_ts && ts < end_ts {
                    let datetime = DateTime::from_timestamp(ts, 0).unwrap_or_default();
                    let sig = Signature::from_str(&item.signature).unwrap();
                    let transaction = rpc.get_transaction(&sig, UiTransactionEncoding::JsonParsed).unwrap();
                    println!("the following signature {} happened on the date {} and the following transactions occurred: ", item.signature, datetime);
                    describe_transfer(&transaction, LAMPORTS_PER_SOL);
                    println!("--------------------------------");
                } else if ts < start_ts {
                    // everything from here on will be older; you can stop entirely
                    return;
                }
            }
        }
        // Prepare next page
        if let Some(oldest) = signatures.last() {
            before = Some(Signature::from_str(&oldest.signature).unwrap());
        } else {
            break;
        }

        if signatures.len() < 1000 {
            break;
        }
    }
}





pub fn get_historical_account_balance(
    rpc: &RpcClient,
    account: &Pubkey,
    lamports_per_sol: u64,
) {
    println!("######################GET HISTORICAL ACCOUNT BALANCE#########################");
    println!("Here is a map of the end-of-day balance whenever this account had activity:\n");

    let account_str = account.to_string();
    let mut before: Option<Signature> = None;
    let mut day_balances: BTreeMap<NaiveDate, (String, u64)> = BTreeMap::new();
    let mut max_balance: Option<(NaiveDate, String, u64)> = None;

    loop {
        let cfg = GetConfirmedSignaturesForAddress2Config {
            before: before.clone(),
            limit: Some(1000),
            ..Default::default()
        };

        let batch = match rpc.get_signatures_for_address_with_config(account, cfg) {
            Ok(batch) => batch,
            Err(err) => {
                eprintln!("RPC error fetching signatures: {err}");
                break;
            }
        };

        if batch.is_empty() {
            break;
        }

        for status in &batch {
            let ts = match status.block_time {
                Some(ts) => ts,
                None => continue,
            };

            let datetime = match DateTime::<Utc>::from_timestamp(ts, 0) {
                Some(dt) => dt,
                None => continue,
            };

            let sig = match Signature::from_str(&status.signature) {
                Ok(sig) => sig,
                Err(_) => continue,
            };

            let tx = match rpc.get_transaction(&sig, UiTransactionEncoding::JsonParsed) {
                Ok(tx) => tx,
                Err(err) => {
                    eprintln!("Could not fetch transaction {}: {err}", status.signature);
                    continue;
                }
            };

            let meta = match tx.transaction.meta.as_ref() {
                Some(meta) => meta,
                None => continue,
            };

            // Build the account list aligned with pre/post balances.
            let mut account_keys: Vec<String> = match &tx.transaction.transaction {
                EncodedTransaction::Json(json_tx) => match &json_tx.message {
                    solana_transaction_status::UiMessage::Parsed(parsed) => parsed
                        .account_keys
                        .iter()
                        .map(|acc| acc.pubkey.clone())
                        .collect(),
                    solana_transaction_status::UiMessage::Raw(raw) => raw.account_keys.clone(),
                },
                _ => continue,
            };

            match &meta.loaded_addresses {
                OptionSerializer::Some(loaded) => {
                    account_keys.extend(loaded.writable.iter().cloned());
                    account_keys.extend(loaded.readonly.iter().cloned());
                }
                OptionSerializer::None | OptionSerializer::Skip => {}
            }

            let Some(index) = account_keys.iter().position(|k| k == &account_str) else {
                continue;
            };

            if index >= meta.post_balances.len() {
                continue;
            }

            let post_balance = meta.post_balances[index];
            let day = datetime.date_naive();

            day_balances
                .entry(day)
                .or_insert_with(|| (status.signature.clone(), post_balance));

            max_balance = match max_balance {
                Some((_, _, current_max)) if current_max >= post_balance => max_balance,
                _ => Some((day, status.signature.clone(), post_balance)),
            };
        }

        if let Some(oldest) = batch.last() {
            match Signature::from_str(&oldest.signature) {
                Ok(sig) => before = Some(sig),
                Err(_) => break,
            }
        } else {
            break;
        }

        if batch.len() < 1000 {
            break;
        }
    }

    if day_balances.is_empty() {
        println!("No historical signatures found for this account.\n");
        return;
    }

    println!("{:<12} | {:<44} | Balance", "Date", "Signature");
    println!("{}", "-".repeat(12 + 3 + 44 + 3 + 32));

    for (day, (signature, lamports)) in day_balances.iter().rev() {
        let sol = *lamports as f64 / lamports_per_sol as f64;
        println!(
            "{} | {} | {} lamports ({:.9} SOL)",
            day,
            signature,
            lamports,
            sol
        );
    }

    if let Some((day, signature, lamports)) = max_balance {
        let sol = lamports as f64 / lamports_per_sol as f64;
        println!(
            "\nHighest observed balance: {} lamports ({:.9} SOL) on {} (signature {})\n",
            lamports, sol, day, signature
        );
    } else {
        println!();
    }
}

pub fn exit() {
    println!("Exiting...");
    std::process::exit(0);
}


