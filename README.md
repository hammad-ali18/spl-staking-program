# Simple Stake Program

A staking program built on Solana using the Anchor framework. This program allows users to stake SPL tokens, claim rewards, and unstake their tokens after a defined duration. It provides the following features:

- Initialize staking pools with customizable parameters.
- Stake SPL tokens and earn rewards based on a reward rate.
- Claim accumulated rewards at regular intervals.
- Unstake tokens after the staking duration ends.

---

## Running Test cases over solana localnet
## Table of Contents
- [Setup Local Validator](#setup-local-validator)
- [Deploy SPL Token on Localnet](#deploy-spl-token-on-localnet)
- [Run Tests](#run-tests)
- [Test Case Details](#test-case-details)
  - [Stake Tokens](#stake-tokens)
  - [Claim Rewards](#claim-rewards)
  - [Unstake Tokens](#unstake-tokens)

---

## Setup Local Validator

To set up and run a local Solana validator:

1. Ensuring Solana CLI has been installed.

2. Set the local validator as your cluster:
   ```bash
   solana config set --url localhost
   ```
3. Start the local validator on a separate terminal:
   ```bash
   solana-test-validator
   ```
---

## Deploy SPL Token on Localnet

1. Create a new SPL token using the Solana CLI:
   ```bash
   spl-token create-token
   ```

2. Create a token account for the mint:
   ```bash
   spl-token create-account <TOKEN_MINT_ADDRESS>
   ```

3. Mint tokens to the token account:
   ```bash
   spl-token mint <TOKEN_MINT_ADDRESS> <AMOUNT> <RECIPIENT_ACCOUNT_ADDRESS>
   ```

4. Replace mint in tests/simple_stake.ts
```javascript
  const mint = new PublicKey("replace_with_newly_deployed_mint_address"); 
  ```
---


## Run Tests

1. Install dependencies:
   ```bash
   npm install
   ```

2. Build the program:
   ```bash
   anchor build
   ```

3. Deploy the program to the local validator:
   ```bash
   anchor deploy
   ```

4. Generata a new programID and synced it across.
   ```bash
    solana-keygen new --outfile target/deploy/simple_stake-keypair.json
   ```
   ```bash
   anchor keys sync
   ```
   Also make sure you manually replace in this too.
   ```javascript
     const programId = new PublicKey("replace_by_newly_generated");
    ```
5. Run the test cases ( make sure local validaor running on a   separate terminal ):
   ```bash
   anchor test --skip-local-validator --skip-deploy
   ```

---

## Test Case Details

### Stake Tokens

Stakes a specified amount of tokens in the staking pool.

```javascript
it("Simple Stake", async () => {
  let amountToStake = new anchor.BN(2000000000);
  let tx = await program.methods.stakeSpl(amountToStake)
    .accounts({
        ...
    })

});
```

### Claim Rewards

Allows a staker to claim rewards from the staking pool.

```javascript
it("Simple Claim", async () => {
  let tx = await program.methods.claimSpl()
    .accounts({
    ...
    })
    .signers([staker1]).rpc();
  console.log("Claim tx", tx);
});
```

### Unstake Tokens

Unstakes a specified amount of tokens from the staking pool.

```javascript
it("Simple UnStake", async () => {
  let amountToUnStake = new anchor.BN(200000000);
  let tx = await program.methods.unstakeSpl(amountToUnStake)
    .accounts({
    ...
    })
    .signers([staker1]).rpc();
  console.log("Unstake tx", tx);
});
```

---

## Additional Resources
- [Solana Documentation](https://docs.solana.com/)
- [Anchor Framework](https://project-serum.github.io/anchor/)
- [SPL Token Library](https://spl.solana.com/token)

---

Feel free to contribute or raise issues for improvements!
