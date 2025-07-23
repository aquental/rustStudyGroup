import * as anchor from "@project-serum/anchor";
import { Program } from "@project-serum/anchor";
import { Counter, IDL } from "../target/types/counter";
import { Keypair, SystemProgram, PublicKey } from "@solana/web3.js";
import { assert } from "chai";

describe("Counter Program Tests", () => {
  // Configure the client to use the local cluster.
  const provider = anchor.AnchorProvider.env();
  anchor.setProvider(provider);

  // Load the program manually with the deployed program ID
  const programId = new PublicKey("nLjxYLhw8eSjrG4ui8rZvCZR44tmpb9PRBqtQ36MmhR");
  const program = new Program(IDL, programId, provider);
  
  let counterAccount: Keypair;

  beforeEach(() => {
    // Generate a new keypair for each test
    counterAccount = Keypair.generate();
  });

  it("Initializes a counter", async () => {
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
      
    console.log("Initialize transaction signature:", tx);
    
    // Fetch the counter account
    const account = await program.account.counterAccount.fetch(counterAccount.publicKey);
    console.log("Counter initialized with count:", account.count.toString());
    
    // Verify the counter starts at 0
    assert.equal(account.count, 0, "Counter should start at 0");
  });
  
  it("Increments the counter", async () => {
    // First initialize
    await program.methods
      .initialize()
      .accounts({
        counter: counterAccount.publicKey,
        user: provider.wallet.publicKey,
        systemProgram: SystemProgram.programId,
      })
      .signers([counterAccount])
      .rpc();

    // Then increment
    const tx = await program.methods
      .increment()
      .accounts({
        counter: counterAccount.publicKey,
      })
      .rpc();
      
    console.log("Increment transaction signature:", tx);
    
    // Fetch the counter account
    const account = await program.account.counterAccount.fetch(counterAccount.publicKey);
    console.log("Counter incremented to:", account.count.toString());
    
    // Verify the counter is now 1
    assert.equal(account.count, 1, "Counter should be 1 after increment");
  });

  it("Increments the counter multiple times", async () => {
    // Initialize
    await program.methods
      .initialize()
      .accounts({
        counter: counterAccount.publicKey,
        user: provider.wallet.publicKey,
        systemProgram: SystemProgram.programId,
      })
      .signers([counterAccount])
      .rpc();

    // Increment 5 times
    for (let i = 0; i < 5; i++) {
      await program.methods
        .increment()
        .accounts({
          counter: counterAccount.publicKey,
        })
        .rpc();
    }
    
    // Fetch the counter account
    const account = await program.account.counterAccount.fetch(counterAccount.publicKey);
    console.log("Counter after 5 increments:", account.count.toString());
    
    // Verify the counter is 5
    assert.equal(account.count, 5, "Counter should be 5 after 5 increments");
  });

  it("Can create multiple independent counters", async () => {
    const counter1 = Keypair.generate();
    const counter2 = Keypair.generate();
    
    // Initialize both counters
    await program.methods
      .initialize()
      .accounts({
        counter: counter1.publicKey,
        user: provider.wallet.publicKey,
        systemProgram: SystemProgram.programId,
      })
      .signers([counter1])
      .rpc();
      
    await program.methods
      .initialize()
      .accounts({
        counter: counter2.publicKey,
        user: provider.wallet.publicKey,
        systemProgram: SystemProgram.programId,
      })
      .signers([counter2])
      .rpc();
    
    // Increment counter1 twice, counter2 three times
    for (let i = 0; i < 2; i++) {
      await program.methods
        .increment()
        .accounts({ counter: counter1.publicKey })
        .rpc();
    }
    
    for (let i = 0; i < 3; i++) {
      await program.methods
        .increment()
        .accounts({ counter: counter2.publicKey })
        .rpc();
    }
    
    // Verify both counters have independent values
    const account1 = await program.account.counterAccount.fetch(counter1.publicKey);
    const account2 = await program.account.counterAccount.fetch(counter2.publicKey);
    
    console.log(`Counter 1: ${account1.count}, Counter 2: ${account2.count}`);
    
    assert.equal(account1.count, 2, "Counter 1 should be 2");
    assert.equal(account2.count, 3, "Counter 2 should be 3");
  });
});
