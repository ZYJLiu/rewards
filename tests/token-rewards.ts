import * as anchor from "@project-serum/anchor"
import { Program, ProgramErrorStack } from "@project-serum/anchor"
import { TokenRewards } from "../target/types/token_rewards"
import {
  PublicKey,
  Keypair,
  SystemProgram,
  SYSVAR_RENT_PUBKEY,
  LAMPORTS_PER_SOL,
} from "@solana/web3.js"
import {
  TOKEN_PROGRAM_ID,
  getOrCreateAssociatedTokenAccount,
  getAccount,
} from "@solana/spl-token"
import { findMetadataPda } from "@metaplex-foundation/js"
import { assert, expect } from "chai"
import { token } from "@project-serum/anchor/dist/cjs/utils"

describe("token-rewards", () => {
  // Configure the client to use the local cluster.
  anchor.setProvider(anchor.AnchorProvider.env())

  const program = anchor.workspace.TokenRewards as Program<TokenRewards>
  const connection = anchor.getProvider().connection
  const userWallet = anchor.workspace.TokenRewards.provider.wallet

  const TOKEN_METADATA_PROGRAM_ID = new anchor.web3.PublicKey(
    "metaqbxxUerdq28cj1RbAWkYQm3ybzjb6a8bt518x1s"
  )
  const usdcMint = new PublicKey("Gh9ZwEmdLJ8DscKNTkTqPbNwLNNBjuSzaG9Vp2KGtKJr")

  var merchantPDA: PublicKey
  var rewardMintPDA: PublicKey

  before(async () => {
    ;[merchantPDA] = await PublicKey.findProgramAddress(
      [Buffer.from("MERCHANT"), userWallet.publicKey.toBuffer()],
      program.programId
    )
    ;[rewardMintPDA] = await PublicKey.findProgramAddress(
      [Buffer.from("REWARD")],
      program.programId
    )
  })

  it("Initialize Merchant Account", async () => {
    let name = "name"
    let image =
      "https://arweave.net/8APPIUAigWaUpEt5jC10M1-dgNGaN9kbr1yNI1qv20U?ext=png"

    const tx = await program.methods
      .initMerchant({
        name: name,
        image: image,
      })
      .accounts({
        merchant: merchantPDA,
        user: userWallet.publicKey,
        systemProgram: SystemProgram.programId,
        rent: SYSVAR_RENT_PUBKEY,
      })
      .rpc()
    // console.log("Your transaction signature", tx)

    const merchantAccount = await program.account.merchantState.fetch(
      merchantPDA
    )
    expect(merchantAccount.name).to.equal(name)
    expect(merchantAccount.image).to.equal(image)
  })

  // it("Create Promo", async () => {
  //   let count = await (
  //     await program.account.merchantState.fetch(merchantPDA)
  //   ).promoCount
  //   console.log(count)

  //   const [promoDataPda, promoDataBump] = await PublicKey.findProgramAddress(
  //     [merchantPDA.toBuffer(), count.toBuffer("le", 8)],
  //     program.programId
  //   )

  //   const [promoMintPda, promoMintBump] = await PublicKey.findProgramAddress(
  //     [Buffer.from("MINT"), promoDataPda.toBuffer()],
  //     program.programId
  //   )

  //   const metadataPDA = await findMetadataPda(promoMintPda)

  //   const tx = await program.methods
  //     .createPromo({
  //       name: "NAME",
  //       symbol: "SYMBOL",
  //       uri: "https://arweave.net/OwXDf7SM6nCVY2cvQ4svNjtV7WBTz3plbI4obN9JNkk",
  //     })
  //     .accounts({
  //       merchant: merchantPDA,
  //       promo: promoDataPda,
  //       promoMint: promoMintPda,
  //       user: userWallet.publicKey,
  //       systemProgram: SystemProgram.programId,
  //       rent: SYSVAR_RENT_PUBKEY,
  //       tokenProgram: TOKEN_PROGRAM_ID,
  //       metadata: metadataPDA,
  //       tokenMetadataProgram: TOKEN_METADATA_PROGRAM_ID,
  //     })
  //     .rpc()
  //   // console.log("Your transaction signature", tx)

  //   const promoAccount = await program.account.promoState.fetch(promoDataPda)
  //   console.log(promoAccount)
  //   expect(promoAccount.mint.toString()).to.equal(promoMintPda.toString())
  // })

  // it("Create and Mint Promo", async () => {
  //   let count = await (
  //     await program.account.merchantState.fetch(merchantPDA)
  //   ).promoCount
  //   console.log(count)

  //   const [promoDataPda, promoDataBump] = await PublicKey.findProgramAddress(
  //     [merchantPDA.toBuffer(), count.toBuffer("be", 8)],
  //     program.programId
  //   )

  //   const [promoMintPda, promoMintBump] = await PublicKey.findProgramAddress(
  //     [Buffer.from("MINT"), promoDataPda.toBuffer()],
  //     program.programId
  //   )

  //   const metadataPDA = await findMetadataPda(promoMintPda)

  //   const tx = await program.methods
  //     .createPromo({
  //       name: "NAME",
  //       symbol: "SYMBOL",
  //       uri: "https://arweave.net/OwXDf7SM6nCVY2cvQ4svNjtV7WBTz3plbI4obN9JNkk",
  //     })
  //     .accounts({
  //       merchant: merchantPDA,
  //       promo: promoDataPda,
  //       promoMint: promoMintPda,
  //       user: userWallet.publicKey,
  //       systemProgram: SystemProgram.programId,
  //       rent: SYSVAR_RENT_PUBKEY,
  //       tokenProgram: TOKEN_PROGRAM_ID,
  //       metadata: metadataPDA,
  //       tokenMetadataProgram: TOKEN_METADATA_PROGRAM_ID,
  //     })
  //     .rpc()

  //   const tokenAccount = await getOrCreateAssociatedTokenAccount(
  //     connection,
  //     userWallet.payer,
  //     promoMintPda,
  //     userWallet.publicKey
  //   )

  //   console.log(tokenAccount)

  //   const tx2 = await program.methods
  //     .mintPromo()
  //     .accounts({
  //       promo: promoDataPda,
  //       promoMint: promoMintPda,
  //       tokenProgram: TOKEN_PROGRAM_ID,
  //       tokenAccount: tokenAccount.address,
  //       user: userWallet.publicKey,
  //     })
  //     .rpc()
  //   console.log("Your transaction signature", tx2)

  //   const account = await getAccount(connection, tokenAccount.address)
  //   console.log(account)
  //   expect(Number(account.amount)).to.equal(1)
  // })

  // it("Create Reward Mint", async () => {
  //   const metadataPDA = await findMetadataPda(rewardMintPDA)

  //   const tx = await program.methods
  //     .createReward({
  //       name: "NAME",
  //       symbol: "SYMBOL",
  //       uri: "https://arweave.net/OwXDf7SM6nCVY2cvQ4svNjtV7WBTz3plbI4obN9JNkk",
  //     })
  //     .accounts({
  //       merchant: merchantPDA,
  //       rewardMint: rewardMintPDA,
  //       user: userWallet.publicKey,
  //       systemProgram: SystemProgram.programId,
  //       rent: SYSVAR_RENT_PUBKEY,
  //       tokenProgram: TOKEN_PROGRAM_ID,
  //       metadata: metadataPDA,
  //       tokenMetadataProgram: TOKEN_METADATA_PROGRAM_ID,
  //     })
  //     .rpc()

  //   const merchantAccount = await program.account.merchantState.fetch(
  //     merchantPDA
  //   )
  //   expect(merchantAccount.mint.toString()).to.equal(rewardMintPDA.toString())
  //   // expect(merchantAccount.mintBump).to.equal(rewardMintBump)
  // })

  // it("Gift Tokens", async () => {
  //   const rewardTokenAccount = await getOrCreateAssociatedTokenAccount(
  //     connection,
  //     userWallet.payer,
  //     rewardMintPDA,
  //     userWallet.publicKey
  //   )

  //   const usdcTokenAccount = await getOrCreateAssociatedTokenAccount(
  //     connection,
  //     userWallet.payer,
  //     usdcMint,
  //     userWallet.publicKey
  //   )

  //   const tx = await program.methods
  //     .mintReward({
  //       amount: new anchor.BN(1000000),
  //     })
  //     .accounts({
  //       merchant: merchantPDA,
  //       rewardMint: rewardMintPDA,
  //       usdcMint: usdcMint,
  //       customerRewardToken: rewardTokenAccount.address,
  //       customerUsdcToken: usdcTokenAccount.address,
  //       userUsdcToken: usdcTokenAccount.address,
  //       user: userWallet.publicKey,
  //       customer: userWallet.publicKey,
  //       tokenProgram: TOKEN_PROGRAM_ID,
  //     })
  //     .rpc()

  //   console.log(`https://explorer.solana.com/tx/${tx}?cluster=devnet`)
  // })

  // it("Burn Tokens", async () => {
  //   const rewardTokenAccount = await getOrCreateAssociatedTokenAccount(
  //     connection,
  //     userWallet.payer,
  //     rewardMintPDA,
  //     userWallet.publicKey
  //   )

  //   // const usdcTokenAccount = await getOrCreateAssociatedTokenAccount(
  //   //   connection,
  //   //   userWallet.payer,
  //   //   usdcMint,
  //   //   userWallet.publicKey
  //   // )

  //   const tx = await program.methods
  //     .burnReward({
  //       amount: new anchor.BN(1000000),
  //     })
  //     .accounts({
  //       merchant: merchantPDA,
  //       rewardMint: rewardMintPDA,
  //       // usdcMint: usdcMint,
  //       customerRewardToken: rewardTokenAccount.address,
  //       // customerUsdcToken: usdcTokenAccount.address,
  //       // userUsdcToken: usdcTokenAccount.address,
  //       user: userWallet.publicKey,
  //       customer: userWallet.publicKey,
  //       tokenProgram: TOKEN_PROGRAM_ID,
  //     })
  //     .rpc()

  //   console.log(`https://explorer.solana.com/tx/${tx}?cluster=devnet`)
  // })
})
