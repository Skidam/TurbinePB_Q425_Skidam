import * as anchor from "@coral-xyz/anchor";
import { Program } from "@coral-xyz/anchor";
import { SolanaCounterProgram } from "../target/types/solana_counter_program";
import { assert } from 'chai'; 
import { EpochSchedule, Keypair } from '@solana/web3.js'; 
import { publicKey } from "@coral-xyz/anchor/dist/cjs/utils";

describe("solana-counter-program", () => {
  // Configure the client to use the local cluster.
  const provider = anchor.AnchorProvider.env(); 

  anchor.setProvider(provider);
  const payer = provider.wallet as anchor.Wallet;




 const program = anchor.workspace.solanaCounterProgram as Program<SolanaCounterProgram>;

 const counterKeypair = new Keypair(); 

 it ('Inintialize Counter', async () => {
  await program.methods
  .initializeCounter()
  .accounts({
    counter: counterKeypair.publicKey,
    payer: payer.publicKey,
  })
  .signers([counterKeypair])
  .rpc();


  const currentCount = await program.account.counter.fetch(counterKeypair.publicKey); 

  assert(currentCount.count.toNumber() === 0, 'Expeceted initialized count to be 0');

 });

 it ('Increament Counter', async () => {
  await program.methods.increment().accounts({ counter: counterKeypair.publicKey}).rpc();

  const currentCount = await program.account.counter.fetch(counterKeypair.publicKey); 

  assert(currentCount.count.toNumber() === 1, 'Expected count to be 1'); 
 });

  it('Increment Counter Again', async () => {
    await program.methods.increment().accounts({ counter: counterKeypair.publicKey }).rpc();

    const currentCount = await program.account.counter.fetch(counterKeypair.publicKey);

    assert(currentCount.count.toNumber() === 2, 'Expected  count to be 2');
  }); 



  // it("Is initialized!", async () => {
  //   // Add your test here.
  //   const tx = await program.methods.initialize().rpc();
  //   console.log("Your transaction signature", tx);
  // });
});
