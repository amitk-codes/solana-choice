import * as anchor from "@coral-xyz/anchor";
import { Program } from "@coral-xyz/anchor";
import { SolanaChoice } from "../target/types/solana_choice";
import { assert } from "chai";

describe("solana_choice", () => {
  // Configure the client to use the local cluster.
  anchor.setProvider(anchor.AnchorProvider.env());

  const program = anchor.workspace.SolanaChoice as Program<SolanaChoice>;
  const web3 = anchor.web3;
  const connection = program.provider.connection;

  const fundWallet = async (walletAddress: anchor.web3.PublicKey) => {
    const airdropSignature = await connection.requestAirdrop(walletAddress, 2 * web3.LAMPORTS_PER_SOL);
    const { blockhash, lastValidBlockHeight } = await connection.getLatestBlockhash();

    await connection.confirmTransaction({
      lastValidBlockHeight,
      blockhash,
      signature: airdropSignature
    }, "confirmed")
  }

  it("Initializes the Poll Account", async () => {
    const demoKeypair = web3.Keypair.generate();
    await fundWallet(demoKeypair.publicKey)

    const [pollAccountDump] = web3.PublicKey.findProgramAddressSync(
      [Buffer.from("poll"), new anchor.BN(1).toArrayLike(Buffer, "le", 8), demoKeypair.publicKey.toBuffer()],
      program.programId
    )

    const demoPollId = 1;
    const demoDescription = "test-description";
    const demoStartDate = 1727760548;
    const demoEndDate = 1733030948;

    const accounts = {
      signer: demoKeypair.publicKey,
      poll_account: pollAccountDump,
      system_program: web3.SystemProgram.programId
    }

    await program.methods
      .initializePoll(
        new anchor.BN(demoPollId),
        demoDescription,
        new anchor.BN(demoStartDate),
        new anchor.BN(demoEndDate)
      )
      .accounts(accounts)
      .signers([demoKeypair])
      .rpc();

    const fetchPollAccount = await program.account.pollAccount.fetch(pollAccountDump);
    assert.equal(fetchPollAccount.description, demoDescription);
    assert.equal(fetchPollAccount.pollId.toNumber(), demoPollId)
    assert.equal(fetchPollAccount.startDate.toNumber(), demoStartDate)
    assert.equal(fetchPollAccount.endDate.toNumber(), demoEndDate)
  });

  it("Initializes the Choice Account", async () => {

    // First initializing the poll account
    const demoKeypair = web3.Keypair.generate();
    await fundWallet(demoKeypair.publicKey)

    const [pollAccountBump] = web3.PublicKey.findProgramAddressSync(
      [Buffer.from("poll"), new anchor.BN(1).toArrayLike(Buffer, "le", 8), demoKeypair.publicKey.toBuffer()],
      program.programId
    )

    const demoPollId = 1;
    const demoDescription = "test-description";
    const demoStartDate = 1727760548;
    const demoEndDate = 1733030948;

    const pollAccounts = {
      signer: demoKeypair.publicKey,
      poll_account: pollAccountBump,
      system_program: web3.SystemProgram.programId
    }

    await program.methods
      .initializePoll(
        new anchor.BN(demoPollId),
        demoDescription,
        new anchor.BN(demoStartDate),
        new anchor.BN(demoEndDate)
      )
      .accounts(pollAccounts)
      .signers([demoKeypair])
      .rpc();


    // Now running actual test by using above poll account

    const demoChoice = "test-choice"

    const [choiceAccountBump] = await web3.PublicKey.findProgramAddressSync(
      [Buffer.from("choice"), new anchor.BN(1).toArrayLike(Buffer, "le", 8), Buffer.from(demoChoice), demoKeypair.publicKey.toBuffer()],
      program.programId
    )

    const accounts = {
      signer: demoKeypair.publicKey,
      choice_account: choiceAccountBump,
      system_program: web3.SystemProgram.programId
    }

    await program.methods
      .initializeChoice(
        new anchor.BN(1),
        demoChoice
      )
      .accounts(accounts)
      .signers([demoKeypair])
      .rpc()

    const fetchChoiceAccount = await program.account.choiceAccount.fetch(choiceAccountBump);
    assert.equal(fetchChoiceAccount.choiceName, demoChoice);
    assert.equal(fetchChoiceAccount.choiceVotes.toNumber(), 0)
  })
});
