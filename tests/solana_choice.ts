import * as anchor from "@coral-xyz/anchor";
import { Program } from "@coral-xyz/anchor";
import { SolanaChoice } from "../target/types/solana_choice";
import { assert } from "chai";

describe("CHECK TEST", () => {
  const web3 = anchor.web3;
  const program = anchor.workspace.SolanaChoice as Program<SolanaChoice>;
  const signer = web3.Keypair.generate();

  const POLL_ID = 1;
  const DESCRIPTION = "test description";
  const START_DATE = 1738395586;
  const END_DATE = 1769931586;
  const CHOICE_NAME = "Choice 1";

  let pollAccountPDA = null;

  beforeEach(async () => {
    // funding the wallet
    const tx = await program.provider.connection.requestAirdrop(
      signer.publicKey,
      0.05 * web3.LAMPORTS_PER_SOL
    );

    const { blockhash, lastValidBlockHeight } =
      await program.provider.connection.getLatestBlockhash();
    await program.provider.connection.confirmTransaction({
      blockhash,
      lastValidBlockHeight,
      signature: tx,
    });
  });

  before(async () => {
    const [pollPDA] = web3.PublicKey.findProgramAddressSync(
      [
        Buffer.from("poll"),
        new anchor.BN(POLL_ID).toArrayLike(Buffer, "le", 8),
        signer.publicKey.toBuffer(),
      ],
      program.programId
    );

    pollAccountPDA = pollPDA;
  });

  it("initializes the poll account", async () => {
    await program.methods
      .initializePoll(
        new anchor.BN(POLL_ID),
        DESCRIPTION,
        new anchor.BN(START_DATE),
        new anchor.BN(END_DATE)
      )
      .accounts({ signer: signer.publicKey })
      .signers([signer])
      .rpc();

    const fetchPoll = await program.account.pollAccount.fetch(pollAccountPDA);

    console.dir({ fetchPoll }, { depth: Infinity });

    assert.equal(fetchPoll.pollId.toNumber(), POLL_ID);
    assert.equal(fetchPoll.description, DESCRIPTION);
    assert.equal(fetchPoll.startDate.toNumber(), START_DATE);
    assert.equal(fetchPoll.endDate.toNumber(), END_DATE);
  });

  it("initializes the choice accounts", async () => {
    const tx = await program.methods
      .initializeChoice(new anchor.BN(POLL_ID), CHOICE_NAME)
      .accounts({ signer: signer.publicKey })
      .signers([signer])
      .rpc();

    console.log({ initChoiceTx: tx });

    const [choiceAccountPDA] = web3.PublicKey.findProgramAddressSync(
      [
        Buffer.from("choice"),
        new anchor.BN(POLL_ID).toArrayLike(Buffer, "le", 8),
        Buffer.from(CHOICE_NAME),
        signer.publicKey.toBuffer(),
      ],
      program.programId
    );

    const fetchChoice = await program.account.choiceAccount.fetch(
      choiceAccountPDA
    );

    assert.equal(fetchChoice.choiceName, CHOICE_NAME);
    assert.equal(fetchChoice.choiceVotes.toNumber(), 0);
  });

  it("votes for a specific choice account", async () => {
    const tx = await program.methods
      .vote(new anchor.BN(POLL_ID), CHOICE_NAME)
      .accounts({ signer: signer.publicKey })
      .signers([signer])
      .rpc();

    console.dir({ tx }, { depth: Infinity });

    const [choiceAccountPDA] = web3.PublicKey.findProgramAddressSync(
      [
        Buffer.from("choice"),
        new anchor.BN(POLL_ID).toArrayLike(Buffer, "le", 8),
        Buffer.from(CHOICE_NAME),
        signer.publicKey.toBuffer(),
      ],
      program.programId
    );

    const fetchChoice = await program.account.choiceAccount.fetch(
      choiceAccountPDA
    );
    assert.equal(fetchChoice.choiceVotes.toNumber(), 1);
    assert.equal(fetchChoice.choiceName, CHOICE_NAME);
  });
});
