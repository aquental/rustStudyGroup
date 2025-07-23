import * as anchor from "@project-serum/anchor";
import { Program } from "@project-serum/anchor";
import { Counter } from "../target/types/counter";

describe("Simple workspace test", () => {
  // Configure the client to use the local cluster.
  anchor.setProvider(anchor.AnchorProvider.env());

  const program = anchor.workspace.Counter as Program<Counter>;
  
  it("Can access the program", async () => {
    console.log("Program ID:", program.programId.toBase58());
    console.log("Program methods available:", Object.keys(program.methods || {}));
    console.log("Program RPC methods available:", Object.keys(program.rpc || {}));
    console.log("Program available:", !!program);
  });
});
