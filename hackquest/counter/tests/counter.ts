import * as anchor from "@project-serum/anchor";
import { Program } from "@project-serum/anchor";
import { Counter } from "../target/types/counter";
import { Keypair, SystemProgram } from "@solana/web3.js";
import { assert } from "chai";

describe("counter", () => {
  // Configure the client to use the local cluster.
  anchor.setProvider(anchor.AnchorProvider.env());

  const program = anchor.workspace.Counter as Program<Counter>;
  const provider = anchor.AnchorProvider.env();
  
  // Generate a new keypair for the counter account
  const counterAccount = Keypair.generate();

  it("Is initialized!", async () => {
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
  });
  
  it("Increments the counter once", async () => {
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

  it("Increments the counter multiple times", async () => {
    // Increment 5 more times to test multiple increments
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
    console.log("Counter after multiple increments:", account.count.toString());
    
    // Should be 6 (1 from previous test + 5 from this test)
    assert.equal(account.count, 6, "Counter should be 6 after multiple increments");
  });

  it("Fails to initialize an already initialized account", async () => {
    try {
      // Try to initialize the same account again
      await program.methods
        .initialize()
        .accounts({
          counter: counterAccount.publicKey,
          user: provider.wallet.publicKey,
          systemProgram: SystemProgram.programId,
        })
        .signers([counterAccount])
        .rpc();
      
      // If we get here, the test should fail
      assert.fail("Expected initialization to fail for existing account");
    } catch (error) {
      console.log("Expected error caught:", error.message);
      // This should fail because account already exists
      assert.include(error.message.toLowerCase(), "already in use", "Should fail with 'already in use' error");
    }
  });

  it("Fails to increment non-existent counter", async () => {
    const nonExistentCounter = Keypair.generate();
    
    try {
      await program.methods
        .increment()
        .accounts({
          counter: nonExistentCounter.publicKey,
        })
        .rpc();
      
      assert.fail("Expected increment to fail for non-existent account");
    } catch (error) {
      console.log("Expected error caught:", error.message);
      // Should fail because account doesn't exist
      assert.include(error.message.toLowerCase(), "account", "Should fail with account-related error");
    }
  });

  it("Multiple counters can be created and work independently", async () => {
    // Create two separate counter accounts
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

  it("Counter handles large numbers correctly", async () => {
    const largeCounter = Keypair.generate();
    
    // Initialize counter
    await program.methods
      .initialize()
      .accounts({
        counter: largeCounter.publicKey,
        user: provider.wallet.publicKey,
        systemProgram: SystemProgram.programId,
      })
      .signers([largeCounter])
      .rpc();
    
    // Increment many times (but not too many to avoid timeout)
    const incrementCount = 100;
    for (let i = 0; i < incrementCount; i++) {
      await program.methods
        .increment()
        .accounts({ counter: largeCounter.publicKey })
        .rpc();
    }
    
    const account = await program.account.counterAccount.fetch(largeCounter.publicKey);
    console.log(`Counter after ${incrementCount} increments:`, account.count.toString());
    
    assert.equal(account.count, incrementCount, `Counter should be ${incrementCount}`);
  });
});
