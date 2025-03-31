import * as anchor from "@coral-xyz/anchor";
import * as assert from 'assert';

describe("oracle", () => {
  const provider = anchor.AnchorProvider.env(); // Use environment provider (localnet/devnet)
  anchor.setProvider(provider);

  it("Is initialized!", async () => {
    console.log("Test runs");

    // Accessing the Oracle program via the workspace
    const program = anchor.workspace.Oracle;

    // Here you can fetch accounts or check the program state
    try {
      const state = await program.account.oracle.fetch("someOraclePublicKey");

      // Add assertions or checks here to confirm the state
      assert.isDefined(state, "Oracle account is not initialized correctly");
    } catch (error) {
      console.error("Error fetching the Oracle account:", error);
      assert.fail("Failed to fetch Oracle account");
    }
  });
});
