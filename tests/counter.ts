import * as anchor from "@coral-xyz/anchor";
import { Program } from "@coral-xyz/anchor";

import { Counter } from "../target/types/counter";
import { BN } from "bn.js";
import { assert } from "chai";

describe("counter", () => {
  // Configure the client to use the local cluster.
  const provider = anchor.AnchorProvider.env();
  anchor.setProvider(provider);
  const counterKp = new anchor.web3.Keypair();
  const program = anchor.workspace.Counter as Program<Counter>;

  it("CreateCounter method works correctly", async () => {
    // Add your test here.
    const count = new BN(23);
    const tx = await program.methods
      .createCounter(count)
      .accounts({
        counter: counterKp.publicKey,
        authority: provider.wallet.publicKey,
        systemProgram: anchor.web3.SystemProgram.programId,
      })
      .signers([counterKp])
      .rpc(); // Execute instructions
    console.log("Your transaction signature", tx);

    const counter = await program.account.counter.fetch(counterKp.publicKey);
    console.log("Counter count is: ", counter.number.toNumber());
    assert(count.eq(counter.number));
  });

  it("UpdateCounter method works correctly", async () => {
    const number = new BN(23);
    const tx = await program.methods
      .updateCounter(number)
      .accounts({
        counter: counterKp.publicKey,
        authority: provider.wallet.publicKey,
      })
      .rpc();

    console.log("Your transaction signature", tx);
    const counter = await program.account.counter.fetch(counterKp.publicKey);
    console.log("Counter count is: ", counter.number.toNumber());
    assert(number.eq(counter.number));
  });

  it("IncrementCounter method works correctly", async () => {
    let counter = await program.account.counter.fetch(counterKp.publicKey);
    const oldValue = counter.number;
    const tx = await program.methods
      .incrementCounter()
      .accounts({
        counter: counterKp.publicKey,
        authority: provider.wallet.publicKey,
      })
      .rpc();

    console.log("Your transaction signature", tx);
    counter = await program.account.counter.fetch(counterKp.publicKey);
    console.log("Counter count is: ", counter.number.toNumber());
    assert(oldValue.lt(counter.number));
  });

  it("DecrementCounter method works correctly", async () => {
    let counter = await program.account.counter.fetch(counterKp.publicKey);
    const oldValue = counter.number;
    const tx = await program.methods
      .decrementCounter()
      .accounts({
        counter: counterKp.publicKey,
        authority: provider.wallet.publicKey,
      })
      .rpc();

    console.log("Your transaction signature", tx);
    counter = await program.account.counter.fetch(counterKp.publicKey);
    console.log("Counter count is: ", counter.number.toNumber());
    assert(oldValue.gt(counter.number));
  });

  it("DeleteCounter method works correctly", async () => {
    const tx = await program.methods
      .deleteCounter()
      .accounts({
        counter: counterKp.publicKey,
        authority: provider.wallet.publicKey,
      })
      .rpc();

    console.log("Your transaction signature", tx);
    const counter = await program.account.counter.fetchNullable(
      counterKp.publicKey
    );
    assert.equal(counter, null);
  });
});
