import * as anchor from "@project-serum/anchor";
import { Program } from "@project-serum/anchor";
import { Counter, IDL } from "../target/types/counter";
import { Keypair, SystemProgram, PublicKey } from "@solana/web3.js";
import { assert } from "chai";

describe("Counter Program - Final Test", () => {
  // Configure the client to use the local cluster.
  const provider = anchor.AnchorProvider.env();
  anchor.setProvider(provider);

  // Load the program manually with the deployed program ID
  const programId = new PublicKey("nLjxYLhw8eSjrG4ui8rZvCZR44tmpb9PRBqtQ36MmhR");
  const program = new Program(IDL, programId, provider);
  
  it("Program loads correctly", () => {
    console.log("✅ Program ID:", program.programId.toBase58());
    console.log("✅ Available methods:", Object.keys(program.methods));
    assert.isTrue(program.programId.equals(programId), "Program ID should match");
    assert.isNotNull(program.methods.initialize, "Initialize method should exist");
    assert.isNotNull(program.methods.increment, "Increment method should exist");
  });

  it("Initialize and increment counter", async () => {
    const counterAccount = Keypair.generate();
    
    try {
      console.log("🔄 Initializing counter...");
      
      // Initialize the counter with skip preflight checks
      const initTx = await program.methods
        .initialize()
        .accounts({
          counter: counterAccount.publicKey,
          user: provider.wallet.publicKey,
          systemProgram: SystemProgram.programId,
        })
        .signers([counterAccount])
        .rpc({ skipPreflight: true });
        
      console.log("✅ Initialize transaction:", initTx);
      
      // Small delay to ensure transaction is processed
      await new Promise(resolve => setTimeout(resolve, 1000));
      
      // Fetch the counter account
      const account1 = await program.account.counterAccount.fetch(counterAccount.publicKey);
      console.log("✅ Counter initialized with count:", account1.count.toString());
      assert.equal(account1.count, 0, "Counter should start at 0");
      
      console.log("🔄 Incrementing counter...");
      
      // Increment the counter
      const incTx = await program.methods
        .increment()
        .accounts({
          counter: counterAccount.publicKey,
        })
        .rpc({ skipPreflight: true });
        
      console.log("✅ Increment transaction:", incTx);
      
      // Small delay to ensure transaction is processed
      await new Promise(resolve => setTimeout(resolve, 1000));
      
      // Fetch the counter account again
      const account2 = await program.account.counterAccount.fetch(counterAccount.publicKey);
      console.log("✅ Counter incremented to:", account2.count.toString());
      assert.equal(account2.count, 1, "Counter should be 1 after increment");
      
      console.log("🎉 All tests passed!");
      
    } catch (error) {
      console.error("❌ Test failed with error:", error);
      if (error.logs) {
        console.error("Transaction logs:", error.logs);
      }
      throw error;
    }
  });
});
