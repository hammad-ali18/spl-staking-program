import * as anchor from "@coral-xyz/anchor";
import { Program } from "@coral-xyz/anchor";
import { SimpleStake } from "../target/types/simple_stake";
import { readFileSync } from "fs";
import * as spl from "@solana/spl-token";
import { PublicKey } from "@solana/web3.js";
import { createATA4User, createUserAndReqLamports } from "./helper";

let initializer,initializerATA,fundInitializer;
describe("simple_stake", async () => {
  const provider = anchor.AnchorProvider.env();
  anchor.setProvider(provider);
  const idl = JSON.parse(readFileSync('target/idl/simple_stake.json', 'utf-8'));
  const programId = new PublicKey("4EWZ9ERDuq1tQ41XoghKgTuwvotL5p3VrzUaXgrsGfvs");
  const program = new Program<SimpleStake>(idl, programId, anchor.getProvider())
  console.log("wallet", provider.wallet.publicKey);

  const poolOwner = anchor.web3.Keypair.generate();
  let [pool_state,] = anchor.web3.PublicKey.findProgramAddressSync(
    [Buffer.from("init_spl_pool")],
    programId
  )
  let [stake_vault,] = anchor.web3.PublicKey.findProgramAddressSync(
    [Buffer.from("stake_spl_vault"), pool_state.toBuffer()],
    programId
  )
  let [reward_vault,] = anchor.web3.PublicKey.findProgramAddressSync(
    [Buffer.from("reward_spl_vault"), pool_state.toBuffer()],
    programId
  )
  const mint = new PublicKey("8kiKp72ypk12c9vQmerQBzUUs8PFgmQj6QtkwXHTLY1c")
  before(async () => {
    [initializer] = await createUserAndReqLamports(provider);
    [initializerATA] = await createATA4User(provider,mint,initializer,9);
  })

  it("Initializing staking program!", async () => {
    // Add your test here.
    const rewardRate = new anchor.BN(12);
    const stakeDuration = new anchor.BN(78600);
    const rewardInterval = new anchor.BN(3600);
    const tx = await program.methods.initializeSplPool(poolOwner.publicKey, rewardRate, stakeDuration, rewardInterval)
      .accounts({
        poolState: pool_state,
        stakeVault: stake_vault,
        rewardVault: reward_vault,
        initializer: initializer.publicKey,
        mint: mint,
        tokenProgram: spl.TOKEN_PROGRAM_ID,
        systemProgram: anchor.web3.SystemProgram.programId,
      }).signers([initializer])
      .rpc();

    // // Assert the state has been updated correctly
    // const poolAccount = await program.account.poolState.fetch(poolState.publicKey);
    // assert.equal(poolAccount.rewardRate.toString(), rewardRate.toString());
    // assert.equal(poolAccount.stakeDuration.toString(), stakeDuration.toString());
    // assert.equal(poolAccount.rewardInterval.toString(), rewardInterval.toString());
    console.log("Your transaction signature", tx);
  });
});

it("Simple Stake",async()=>{
  
})

