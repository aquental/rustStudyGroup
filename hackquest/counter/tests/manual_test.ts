import * as anchor from "@project-serum/anchor";
import { Program } from "@project-serum/anchor";
import { Counter, IDL } from "../target/types/counter";
import { Keypair, SystemProgram, PublicKey } from "@solana/web3.js";
import { assert } from "chai";

describe("Manual program loading test", () => {
  // Configure the client to use the local cluster.
  const provider = anchor.AnchorProvider.env();
  anchor.setProvider(provider);

  // Load the program manually with the deployed program ID
  const programId = new PublicKey("nLjxYLhw8eSjrG4ui8rZvCZR44tmpb9PRBqtQ36MmhR");
  const program = new Program(IDL, programId, provider);
  
  // Generate a new keypair for the counter account
  const counterAccount = Keypair.generate();

  it("Can load the program manually", () => {
    console.log("Program ID:", program.programId.toBase58());
    console.log("Program methods available:", Object.keys(program.methods || {}));
    assert.isNotNull(program);
    assert.isNotNull(program.methods);
  });

  it("Is initialized!", async () => {
    try {
      // Initialize the counter
      const tx = await program.methods
        .initialize()
        .accounts({
          counter: counterAccount.publicKey,
          user: provider.wallet.publicKey,
          systemProgram: SystemProgram.programId,
        })
        .signers([counterAccount])
        .rpc();
        
      console.log("Initialize transaction signature", tx);
      
      // Fetch the counter account
      const account = await program.account.counterAccount.fetch(counterAccount.publicKey);
      console.log("Counter initialized with count:", account.count.toString());
      
      // Verify the counter starts at 0
      assert.equal(account.count, 0, "Counter should start at 0");
    } catch (error) {
      console.log("Full error:", error);
      if (error.logs) {
        console.log("Error logs:", error.logs);
      }
      throw error;
    }
  });
  
  it("Increments the counter", async () => {
    // Increment the counter
    const tx = await program.methods
      .increment()
      .accounts({
        counter: counterAccount.publicKey,
      })
      .rpc();
      
    console.log("Increment transaction signature", tx);
    
    // Fetch the counter account
    const account = await program.account.counterAccount.fetch(counterAccount.publicKey);
    console.log("Counter incremented to:", account.count.toString());
    
    // Verify the counter is now 1
    assert.equal(account.count, 1, "Counter should be 1 after first increment");
  });
});
