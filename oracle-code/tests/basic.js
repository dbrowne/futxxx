const anchor = require("@project-serum/anchor");

describe("oracle", () => {
  const provider = anchor.AnchorProvider.env();
  anchor.setProvider(provider);

  it("Is initialized!", async () => {
    console.log("Test runs");
  });
});

