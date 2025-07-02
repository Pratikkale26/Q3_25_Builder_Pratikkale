import { Transaction, SystemProgram, Connection, Keypair, LAMPORTS_PER_SOL, sendAndConfirmTransaction, PublicKey } from "@solana/web3.js";
import wallet from "./dev-wallet.json"

const keypair = Keypair.fromSecretKey(new Uint8Array(wallet));
const to = new PublicKey("28FUAMHV8yfJkqVHEsnqRcp8urwGRvSMxBFMGviXa2DX")

const connection = new Connection("https://solana-devnet.g.alchemy.com/v2/02RfDld8FSNkt6LNuvBQWXwejneybGP9");

// (async () => {
//     try {
//         const txn = new Transaction().add(
//             SystemProgram.transfer({
//                 fromPubkey: keypair.publicKey,
//                 toPubkey: to,
//                 lamports: LAMPORTS_PER_SOL/100
//             })
//         )

//         txn.recentBlockhash = (
//             await connection.getLatestBlockhash("confirmed")
//         ).blockhash;

//         txn.feePayer = keypair.publicKey;
//         const signature = await sendAndConfirmTransaction(
//             connection,
//             txn,
//             [keypair]
//         )

//         console.log(`Success! Check out your txn here: https://explorer.solana.com/tx/${signature}?cluster=devnet`);
//     }catch(e){
//         console.error(`something went wrong: ${e}`);
//     }
// })();

(async () => {
    try {
        const balance = await connection.getBalance(keypair.publicKey);
        const txn = new Transaction().add(
            SystemProgram.transfer({
                fromPubkey: keypair.publicKey,
                toPubkey: to,
                lamports: balance
            })
        );

        txn.recentBlockhash = (await
            connection.getLatestBlockhash("confirmed")
        ).blockhash;
        txn.feePayer = keypair.publicKey;

        const fee = (await
            connection.getFeeForMessage(txn.compileMessage(),
        "confirmed")
        ).value || 0;

        txn.instructions.pop();
        txn.add(
            SystemProgram.transfer({
                fromPubkey: keypair.publicKey,
                toPubkey: to,
                lamports: balance - fee
            })
        );

        // sign transaction, broadcast and cofirm
        const signature = await sendAndConfirmTransaction(
            connection,
            txn,
            [keypair]
        );

        console.log(`Success! Check out your txn here: https://explorer.solana.com/tx/${signature}?cluster=devnet`);
    }catch(e){
        console.log(`something went wrong: ${e}`)
    }
})();