import * as spl from "@solana/spl-token";
import * as anchor from "@coral-xyz/anchor";

export const createUserAndReqLamports = async (provider: anchor.AnchorProvider): Promise<[anchor.web3.Keypair]> => {
    const user = anchor.web3.Keypair.generate();
    let token_airdrop = await provider.connection.requestAirdrop(user.publicKey,
        10 * anchor.web3.LAMPORTS_PER_SOL);

    const latestBlockHash = await provider.connection.getLatestBlockhash();

    await provider.connection.confirmTransaction({
        blockhash: latestBlockHash.blockhash,
        lastValidBlockHeight: latestBlockHash.lastValidBlockHeight,
        signature: token_airdrop,
    });

    return [user];

}
export const createATA4User = async (provider: anchor.AnchorProvider, mint: anchor.web3.PublicKey, user: anchor.web3.Keypair, decimals: number): Promise<[anchor.web3.PublicKey]> => {
    // Create TX to mint tokens to the User
    const txFundATA = new anchor.web3.Transaction();
    let userATA = await spl.getAssociatedTokenAddress(
        mint,
        user.publicKey,
        false,
        spl.TOKEN_PROGRAM_ID,
        spl.ASSOCIATED_TOKEN_PROGRAM_ID
    )
    txFundATA.add(
        spl.createAssociatedTokenAccountInstruction(
            user.publicKey,
            userATA,
            user.publicKey,
            mint,
            spl.TOKEN_PROGRAM_ID,
            spl.ASSOCIATED_TOKEN_PROGRAM_ID
        )
    );

    txFundATA.add(
        spl.createMintToInstruction(
            mint,
            userATA,
            provider.wallet.publicKey,
            2 * 10 ** decimals,
            // 2000000000,
            [],
            spl.TOKEN_PROGRAM_ID
        )
    );

    const txFundToken = await provider.sendAndConfirm(txFundATA, [user]);
    return [userATA];
}