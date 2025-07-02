#[cfg(test)] mod tests {
    use solana_sdk;
    #[test]
    fn keygen() {
        use solana_sdk::{signature::{Keypair, Signer}};
        
        let kp = Keypair::new();
        println!("You have generated a new Solana wallet: {}", kp.pubkey().to_string());
        println!("To save your Solana wallet, copy paste the following into a json file: {:?}", kp.to_bytes())
    }
    
    #[test]
    fn base58_to_wallet() {
        use bs58; use std::io::{self, BufRead};
        
        println!("enter your private key as base58 string");
        let stdin = io::stdin();
        let base58 = stdin.lock().lines().next().unwrap().unwrap();
        let wallet = bs58::decode(base58).into_vec().unwrap();
        println!("Your wallet file format is:{:?}", wallet)
    }
    
    #[test]
    fn wallet_to_base58() {
        use bs58; use std::io::{self, BufRead};

        println!("enter you private key as JSON byte array, ex: [23, 54,...]");

        let stdin = io::stdin();
        let wallet = stdin.lock().lines().next().unwrap().unwrap().trim_start_matches('[').trim_end_matches(']').split(',').map(|s| s.trim().parse::<u8>().unwrap()).collect::<Vec<u8>>();

        let base58 = bs58::encode(wallet).into_string();
        println!("Your Base58 encoded private key is: {:?}", base58)
    }

    #[test]
    fn airdrop() {
        use solana_client::rpc_client::RpcClient;
        use solana_sdk::{
            signature::{Signer, read_keypair_file},
        };

        const RPC_URL: &str = "https://turbine-solanad-4cde.devnet.rpcpool.com/9a9da9cf-6db1-47dc-839a-55aca5c9c80a";
        
        let keypair = read_keypair_file("dev-wallet.json").expect("Coudnt find wallet file");
        let client = RpcClient::new(RPC_URL);
        
        match  client.request_airdrop (&keypair.pubkey(), 2_000_000_000u64) {
            Ok(sig) =>{
                println!("Success! Check your txn here:\nhttps://explorer.solana.com/tx/{}?cluster=devnet", sig);
            }
            Err(err) => {
                println!("Airdrop failded: {}", err)
            }
        }
    }
    
    #[test]
    fn transfer_sol() {
        use solana_client::rpc_client::RpcClient;
        use solana_program::{pubkey::Pubkey, system_instruction::transfer};
        use solana_sdk::{
            signature::{Signer, read_keypair_file},
            transaction::Transaction,
        };
        use std::str::FromStr;
        // use std::ptr::hash;
        let keypair = read_keypair_file("dev-wallet.json").expect("Coudnt find wallet file");

        // let pubkey = keypair.pubkey();

        // let msg_bytes = b"|verify my Solana Keypair!";
        // let sig = keypair.sign_message(msg_bytes);
        // let sig_hashed = hash(sig.asref(), keypair.pvtkey());

        // match  sig.verify(&pubkey.to_bytes(), &sig_hashed.to_bytes()) {
        //     true => println!("Signature verified"),
        //     false => println!("verification failed")
        // }

        let to_pubkey = Pubkey::from_str("28FUAMHV8yfJkqVHEsnqRcp8urwGRvSMxBFMGviXa2DX").unwrap();

        const RPC_URL: &str = "https://api.devnet.solana.com";
        let rpc_client = RpcClient::new(RPC_URL);
        
        let recent_blockhash = rpc_client.get_latest_blockhash()
            .expect("Failed to get recent blockhash");
        
        let txn = Transaction:: new_signed_with_payer(
            &[transfer(&keypair.pubkey(), &to_pubkey, 1_000_000)],
            Some(&keypair.pubkey()),
            &vec![&keypair],
            recent_blockhash
        );
        
        let signature = rpc_client.send_and_confirm_transaction(&txn)
        .expect("Failed to send transaction");
    
    println!(
        "Success! Check out your TX here: https://explorer.solana.com/tx/{}/?cluster=devnet",
        signature
    );
}
}

#[test]
fn enroll() {
    use solana_client::rpc_client::RpcClient;
    use solana_sdk::{
        instruction::{AccountMeta, Instruction},
        pubkey::Pubkey,
        signature::{read_keypair_file, Keypair, Signer},
        transaction::Transaction,
        system_program,
    };
    use std::str::FromStr;

    const RPC_URL: &str = "https://api.devnet.solana.com";
    let rpc_client = RpcClient::new(RPC_URL);

    let signer = read_keypair_file("turbin3-wallet.json").expect("Failed to load wallet");
    let mint = Keypair::new();

    let turbin3_prereq_program = Pubkey::from_str("TRBZyQHB3m68FGeVsqTK39Wm4xejadjVhP5MAZaKWDM").unwrap();
    let collection = Pubkey::from_str("5ebsp5RChCGK7ssRZMVMufgVZhd2kFbNaotcZ5UvytN2").unwrap();
    let mpl_core_program = Pubkey::from_str("CoREENxT6tW1HoK8ypY1SxRMZTcVPm7R94rH4PZNhX7d").unwrap();
    let system_program = system_program::id();

    let signer_pubkey = signer.pubkey();

    // get the pda
    let seeds = &[
        b"prereqs",
        signer_pubkey.as_ref(),
    ];

    let (pda, bump) = Pubkey::find_program_address(seeds, &turbin3_prereq_program);

    println!("PDA: {}", pda);
    println!("Bump: {}", bump);

    let data = vec![77, 124, 82, 163, 21, 133, 181, 206];

    let authority_seeds = &[b"collection", collection.as_ref()];
    let (authority_pda, _bump) = Pubkey::find_program_address(authority_seeds, &turbin3_prereq_program);


    let accounts = vec![
        AccountMeta::new(signer.pubkey(), true),
        AccountMeta::new(pda, false), // pda cant sign-> false
        AccountMeta::new(mint.pubkey(), true),
        AccountMeta::new(collection, false),
        AccountMeta::new_readonly(authority_pda, false),
        AccountMeta::new_readonly(mpl_core_program, false),
        AccountMeta::new_readonly(system_program, false),
    ];

    let recent_blockhash = rpc_client.get_latest_blockhash()
        .expect("Failed to get recent blockhash");

    let instruction = Instruction {
        program_id: turbin3_prereq_program,
        accounts,
        data,
    };

    let txn = Transaction::new_signed_with_payer(
        &[instruction],
        Some(&signer.pubkey()),
        &[&signer, &mint],
        recent_blockhash
    );

    let signature = rpc_client.send_and_confirm_transaction(&txn)
        .expect("Failed to send transaction");

    println!(
        "Success! Check out your TX here:\nhttps://explorer.solana.com/tx/{}/?cluster=devnet",
        signature
    );
}