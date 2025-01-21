import * as anchor from "@coral-xyz/anchor";
import { Program } from "@coral-xyz/anchor";
import { SolanaProgram } from "../target/types/solana_program";
import { PublicKey, SystemProgram } from "@solana/web3.js";
import * as assert from "assert";

describe("solana-program", () => {
  // Configure the client to use the local cluster.
  const provider = anchor.AnchorProvider.env();
  anchor.setProvider(provider);

  const program = anchor.workspace.SolanaProgram as Program<SolanaProgram>;

  it("Creates a proposal", async () => {
    // Generate a new keypair for the creator
    const creator = anchor.web3.Keypair.generate();

    // Airdrop SOL to the creator
    const airdropSignature = await provider.connection.requestAirdrop(
      creator.publicKey,
      anchor.web3.LAMPORTS_PER_SOL
    );
    await provider.connection.confirmTransaction(airdropSignature);

    // Define the proposal details
    const title = "Test Proposal";
    const description = "This is a test proposal.";
    const startDate = new anchor.BN(Math.floor(new Date().getTime() / 1000)); // Current time in seconds
    const endDate = startDate.add(new anchor.BN(86400)); // 24 hours later

    // Find the PDA for the proposal counter
    const [proposalCounterPda, proposalCounterBump] = await PublicKey.findProgramAddressSync(
      [Buffer.from("proposal_counter"), creator.publicKey.toBuffer()],
      program.programId
    );

    // Fetch the proposal counter to get the current count
    let proposalCounter;
    try {
      proposalCounter = await program.account.proposalCounter.fetch(proposalCounterPda);
    } catch (e) {
      // If the proposal counter doesn't exist, initialize it with count 0
      proposalCounter = { count: 0 };
    }

    // Find the PDA for the proposal using the proposal counter's count
    const [proposalPda, proposalBump] = await PublicKey.findProgramAddressSync(
      [Buffer.from("proposal"), creator.publicKey.toBuffer(), Buffer.from(proposalCounter.count.toString())],
      program.programId
    );

    // Create the proposal
    await program.methods
      .createProposal(title, description, startDate, endDate)
      .accounts({
        proposalCounter: proposalCounterPda,
        proposal: proposalPda,
        creator: creator.publicKey,
        systemProgram: SystemProgram.programId,
      })
      .signers([creator])
      .rpc();

    // Fetch the created proposal
    const proposal = await program.account.proposal.fetch(proposalPda);

    // Validate the proposal details
    console.log("Proposal ID:", proposal.id.toString());
    console.log("Proposal Creator:", proposal.creator.toString());
    console.log("Proposal Title:", proposal.title);
    console.log("Proposal Description:", proposal.description);
    console.log("Proposal Start Date:", proposal.startDate.toString());
    console.log("Proposal End Date:", proposal.endDate.toString());
    console.log("Proposal For Votes:", proposal.forVotes.toString());
    console.log("Proposal Against Votes:", proposal.againstVotes.toString());
    console.log("Proposal Abstain Votes:", proposal.abstainVotes.toString());

    // Assertions
    assert.strictEqual(proposal.title, title);
    assert.strictEqual(proposal.description, description);
    assert.strictEqual(proposal.startDate.toString(), startDate.toString());
    assert.strictEqual(proposal.endDate.toString(), endDate.toString());
    assert.strictEqual(proposal.forVotes.toString(), "0");
    assert.strictEqual(proposal.againstVotes.toString(), "0");
    assert.strictEqual(proposal.abstainVotes.toString(), "0");
  });
});