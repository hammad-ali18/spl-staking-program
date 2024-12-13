const anchor = require('@coral-xyz/anchor');

module.exports = async function (provider) {
  // Configure client to use the provider.
  anchor.setProvider(provider);


  const poolAccount = new anchor.PublicKey('');
  // Deploy the program (if it's a new deployment)
  const program = anchor.workspace.SimpleStake;
  const tx = await program.rpc.initializeSpl({
accounts:{
poolState: poolAccount,
}
  });

  console.log("Program deployed with tx:", tx);
};