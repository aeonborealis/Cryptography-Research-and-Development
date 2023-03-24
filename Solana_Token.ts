import { Account, PublicKey, Transaction, TransactionInstruction } from "@solana/web3.js";
import { TOKEN_PROGRAM_ID, Token, ASSOCIATED_TOKEN_PROGRAM_ID, TOKEN_ACCOUNT_LAYOUT } from "@solana/spl-token";

// Set the network and token account addresses
const network = "https://api.mainnet-beta.solana.com";
const tokenAccountAddress = new PublicKey("<TOKEN_ACCOUNT_ADDRESS>");

// Set the payer account and its associated token account
const payerAccount = new Account("<PAYER_PRIVATE_KEY>");
const associatedTokenAddress = await Token.getAssociatedTokenAddress(
  ASSOCIATED_TOKEN_PROGRAM_ID,
  TOKEN_PROGRAM_ID,
  new PublicKey("<TOKEN_MINT_ADDRESS>"),
  payerAccount.publicKey,
);

// Create the token object
const token = new Token(
  provider.connection,
  new PublicKey("<TOKEN_MINT_ADDRESS>"),
  TOKEN_ACCOUNT_LAYOUT,
  payerAccount,
);

// Create the transaction instructions
const instructions: TransactionInstruction[] = [];

// Create the token account if it doesn't exist
const tokenAccountExists = await provider.connection.getAccountInfo(tokenAccountAddress);
if (!tokenAccountExists) {
  const createAccountInstruction = SystemProgram.createAccount({
    fromPubkey: payerAccount.publicKey,
    newAccountPubkey: tokenAccountAddress,
    lamports: await provider.connection.getMinimumBalanceForRentExemption(TOKEN_ACCOUNT_LAYOUT.span),
    space: TOKEN_ACCOUNT_LAYOUT.span,
    programId: TOKEN_PROGRAM_ID,
  });
  instructions.push(createAccountInstruction);
}

// Create the associated token account if it doesn't exist
const associatedTokenAccountExists = await provider.connection.getAccountInfo(associatedTokenAddress);
if (!associatedTokenAccountExists) {
  const createAssociatedTokenAccountInstruction = await Token.createAssociatedTokenAccountInstruction(
    ASSOCIATED_TOKEN_PROGRAM_ID,
    TOKEN_PROGRAM_ID,
    new PublicKey("<TOKEN_MINT_ADDRESS>"),
    associatedTokenAddress,
    payerAccount.publicKey,
    payerAccount.publicKey,
  );
  instructions.push(createAssociatedTokenAccountInstruction);
}

// Mint some tokens to the payer account
const mintAmount = 1000000; // 1 million tokens
const mintToInstruction = await token.createMintToInstruction(
  new PublicKey("<TOKEN_MINT_ADDRESS>"),
  tokenAccountAddress,
  payerAccount.publicKey,
  [],
  mintAmount,
);

// Add the mint to instruction to the transaction instructions
instructions.push(mintToInstruction);

// Create the transaction
const transaction = new Transaction({ feePayer: payerAccount.publicKey, instructions });

// Sign and send the transaction
const signedTransaction = await provider.wallet.signTransaction(transaction);
const transactionSignature = await provider.connection.sendRawTransaction(signedTransaction.serialize());
await provider.connection.confirmTransaction(transactionSignature);
