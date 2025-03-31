import * as anchor from "@coral-xyz/anchor";
import { assert } from "chai";
import oracleIdl from "../target/idl/oracle.json" assert { type: "json" };

describe("oracle", () => {
    const provider = anchor.AnchorProvider.env();
    anchor.setProvider(provider);

    const programId = new anchor.web3.PublicKey("Bs7GGMzNW9nhrZrhxyaLW1AQiaQX6kk1CTqfvj1RkRvS");
    const program = new anchor.Program(oracleIdl as anchor.Idl, programId, provider);

    it("Creates oracle and commits, reveals, resolves", async () => {
        const oracle = anchor.web3.Keypair.generate();
        const commitment = anchor.web3.Keypair.generate();
        const signer = provider.wallet;

        // Create Oracle
        await program.methods
            .createOracle()
            .accounts({
                oracle: oracle.publicKey,
                signer: signer.publicKey,
                systemProgram: anchor.web3.SystemProgram.programId,
            })
            .signers([oracle])
            .rpc();

        // Commit Bit
        const bit = true;
        const salt = "s3cr3t";
        const preimage = `${bit}:${salt}`;
        const hashBuffer = anchor.utils.sha256.digest(preimage);
        const hash = Buffer.from(hashBuffer);

        await program.methods
            .commit(hash as any)
            .accounts({
                oracle: oracle.publicKey,
                commitment: commitment.publicKey,
                signer: signer.publicKey,
                systemProgram: anchor.web3.SystemProgram.programId,
            })
            .signers([commitment])
            .rpc();

        // Advance to Reveal
        await program.methods
            .advancePhase()
            .accounts({
                oracle: oracle.publicKey,
                signer: signer.publicKey,
            })
            .rpc();

        // Reveal
        await program.methods
            .reveal(bit, salt)
            .accounts({
                oracle: oracle.publicKey,
                commitment: commitment.publicKey,
                signer: signer.publicKey,
            })
            .rpc();

        // Advance to Resolved
        await program.methods
            .advancePhase()
            .accounts({
                oracle: oracle.publicKey,
                signer: signer.publicKey,
            })
            .rpc();

        // Resolve Oracle
        await program.methods
            .resolveOracle()
            .accounts({
                oracle: oracle.publicKey,
                signer: signer.publicKey,
            })
            .remainingAccounts([
                {
                    pubkey: commitment.publicKey,
                    isSigner: false,
                    isWritable: false,
                },
            ])
            .rpc();

        const state = await program.account.oracle.fetch(oracle.publicKey);
        assert.equal(state.isResolved, true);
        assert.equal(state.resolutionBit, true);
    });
});
