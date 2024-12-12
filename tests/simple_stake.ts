import * as anchor from "@coral-xyz/anchor";
import { Program } from "@coral-xyz/anchor";
import { SimpleStake } from "../target/types/simple_stake";
import { readFileSync } from "fs";
import * as spl from "@solana/spl-token";
import { PublicKey } from "@solana/web3.js";
import { createATA4User, createUserAndReqLamports } from "./helper";

describe("simple_stake", () => {
  const provider = anchor.AnchorProvider.env();
  anchor.setProvider(provider);
  const idl = JSON.parse(readFileSync("target/idl/simple_stake.json", "utf-8"));
  const programId = new PublicKey("6E1wYENN1oy4jjv4EgARLCi64AAyF4DUncksCq5j1KkP");
  const program = new Program<SimpleStake>(idl, programId, anchor.getProvider());
  console.log("Wallet:", provider.wallet.publicKey.toString());

  const poolOwner = anchor.web3.Keypair.generate();
  const mint = new PublicKey("8kiKp72ypk12c9vQmerQBzUUs8PFgmQj6QtkwXHTLY1c");

  let pool_state: PublicKey;
  let stake_vault: PublicKey;
  let reward_vault: PublicKey;
  let initializer: anchor.web3.Keypair;
  let initializerATA: PublicKey;
  let staker1: anchor.web3.Keypair;
  let stakerATA1: PublicKey;
  let staker_info: PublicKey;

  before(async () => {
    // Derive PDAs
    [pool_state] = anchor.web3.PublicKey.findProgramAddressSync(
      [Buffer.from("init_spl_pool")],
      programId
    );
    [stake_vault] = anchor.web3.PublicKey.findProgramAddressSync(
      [Buffer.from("stake_spl_vault"), pool_state.toBuffer()],
      programId
    );
    console.log("poolstate, stake_vault",pool_state.toBase58(),stake_vault.toBase58());
    [reward_vault] = anchor.web3.PublicKey.findProgramAddressSync(
      [Buffer.from("reward_spl_vault"), pool_state.toBuffer()],
      programId
    );

    // Initialize accounts and ATAs
    initializer = await createUserAndReqLamports(provider);
    initializerATA = await createATA4User(provider, mint, initializer, 9);

    staker1 = await createUserAndReqLamports(provider);
    stakerATA1 = await createATA4User(provider, mint, staker1, 9);

    // Derive staker info PDA
    [staker_info] = anchor.web3.PublicKey.findProgramAddressSync(
      [Buffer.from("staker_spl_info"), staker1.publicKey.toBuffer()],
      programId
    );

    console.log("Initializer PublicKey:", initializer.publicKey.toString());
    console.log("Staker PublicKey:", staker1.publicKey.toString(), stakerATA1);
  });

  // it("Initializing staking program!", async () => {
  //   console.log("hello");
  //   const rewardRate = new anchor.BN(12);
  //   const stakeDuration = new anchor.BN(78600); // Example stake duration
  //   const rewardInterval = new anchor.BN(3600); // Example reward interval

  //   const tx = await program.methods
  //     .initializeSplPool(poolOwner.publicKey, rewardRate, stakeDuration, rewardInterval)
  //     .accounts({
  //       poolState: pool_state,
  //       stakeVault: stake_vault,
  //       rewardVault: reward_vault,
  //       initializer: initializer.publicKey,
  //       mint: mint,
  //       tokenProgram: spl.TOKEN_PROGRAM_ID,
  //       systemProgram: anchor.web3.SystemProgram.programId,
  //     })
  //     .signers([initializer])
  //     .rpc();

  //   console.log("Transaction Signature:", tx);
  // });

  it("Simple Stake", async () => {
    let amountToStake = new anchor.BN(2000000000);
    let tx = await program.methods.stakeSpl(amountToStake)
      .accounts({
        stakerInfo: staker_info,
        stakerTokenAcc: stakerATA1,
        poolState: pool_state,
        stakeVault: stake_vault,
        staker: staker1.publicKey,
        mint: mint,
        associatedTokenProgram: spl.ASSOCIATED_TOKEN_PROGRAM_ID,
        tokenProgram: spl.TOKEN_PROGRAM_ID,
        systemProgram: anchor.web3.SystemProgram.programId
      })
      .signers([staker1]).rpc();
    console.log("Stake tx", tx);
  });
  it("Simple UnStake", async () => {
    let amountToUnStake = new anchor.BN(200000000);
    let tx = await program.methods.unstakeSpl(amountToUnStake)
      .accounts({
        stakerInfo: staker_info,
        stakerTokenAcc: stakerATA1,
        poolState: pool_state,
        stakeVault: stake_vault,
        staker: staker1.publicKey,
        mint: mint,
        associatedTokenProgram: spl.ASSOCIATED_TOKEN_PROGRAM_ID,
        tokenProgram: spl.TOKEN_PROGRAM_ID,
        systemProgram: anchor.web3.SystemProgram.programId
      })
      .signers([staker1]).rpc();
    console.log("Unstake tx", tx);
  });
});
