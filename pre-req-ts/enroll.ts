import { Connection, Keypair, PublicKey } from "@solana/web3.js"
import { Program, Wallet, AnchorProvider } from "@coral-xyz/anchor"
import { IDL, Turbin3Prereq } from "./programs/Turbin3_prereq"
import wallet from "./turbin3-wallet.json"
import { SYSTEM_PROGRAM_ID } from "@coral-xyz/anchor/dist/cjs/native/system";

const keypair = Keypair.fromSecretKey(new Uint8Array(wallet));
const MPL_CORE_PROGRAM_ID = new PublicKey("CoREENxT6tW1HoK8ypY1SxRMZTcVPm7R94rH4PZNhX7d")
const connection = new Connection("https://solana-devnet.g.alchemy.com/v2/02RfDld8FSNkt6LNuvBQWXwejneybGP9")

// anchor provider
const provider = new AnchorProvider(connection, new Wallet(keypair), {
    commitment: "confirmed"
});

const program : Program<Turbin3Prereq> = new Program(IDL, provider);

const account_seeds = [Buffer.from("prereqs"), keypair.publicKey.toBuffer()];
const [account_key, _acc_bump] = PublicKey.findProgramAddressSync(account_seeds, program.programId);

const mintCollection = new PublicKey("5ebsp5RChCGK7ssRZMVMufgVZhd2kFbNaotcZ5UvytN2")
const mintTs = Keypair.generate();

// execute initial transaction
(async () => {
    try {
        const txnHash = await program.methods
        .initialize("Pratikkale26")
        .accountsPartial({
            user: keypair.publicKey,
            account: account_key,
            system_program: SYSTEM_PROGRAM_ID
        })
        .signers([keypair])
        .rpc();

        console.log(`Success! Check out your TX here:https://explorer.solana.com/tx/${txnHash}?cluster=devnet`);
    }catch(e){
        console.log(`something went wrong: ${e}`)
    }
})();

// execute submitTs transaction
(async () => {
    try {
        const txnHash = await program.methods
        .submitTs()
        .accountsPartial({
            user: keypair.publicKey,
            account: account_key,
            mint: mintTs.publicKey,
            collection: mintCollection,
            mpl_core_program: MPL_CORE_PROGRAM_ID,
            system_program: SYSTEM_PROGRAM_ID,
        })
        .signers([keypair, mintTs])
        .rpc()

        console.log(`Success! Check out your TX here:https://explorer.solana.com/tx/${txnHash}?cluster=devnet`);
    }catch(e){
        console.log(`something went wrong: ${e}`)
    }
})();