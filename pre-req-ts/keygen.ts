import { Keypair } from "@solana/web3.js";
import bs58 from "bs58";
import promptSync from "prompt-sync";

const prompt = promptSync();

// functions
function generateWallet() {
  const keypair = Keypair.generate();
  console.log(`You have generated a new Solana wallet: ${keypair.publicKey.toBase58()}`)
  console.log(`[${keypair.secretKey}]`)
}

function base58ToWallet() {
  const base58 = prompt("Enter your base58 private key: ");
  try {
    const secretKey = bs58.decode(base58);
    console.log("Decoded Secret key:", `[${Array.from(secretKey)}]`);
  } catch (err) {
    console.error("Invalid base58 key.");
  }
}

function walletToBase58() {
  const raw = prompt("enter your raw secret key array: ");
  try {
    const bytes = raw.split(",").map((x: string) => parseInt(x.trim()));
    const secretKey = Uint8Array.from(bytes);
    const base58 = bs58.encode(secretKey);
    console.log("Base58 Private key:", base58);
  } catch (err) {
    console.error("Invalid input.");
  }
}

// cli
console.log("PKs- Solana Wallet CLI");
console.log("1. Generate new wallet");
console.log("2. Convert base58 -to- raw secret key");
console.log("3. Convert raw secret key -to- base58");

const choice = prompt("Select an option (1-3): ");

switch (choice) {
  case "1":
    generateWallet();
    break;
  case "2":
    base58ToWallet();
    break;
  case "3":
    walletToBase58();
    break;
  default:
    console.log("Invalid choice. Exiting.");
}
