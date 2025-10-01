import * as anchor from "@coral-xyz/anchor";
import { Program } from "@coral-xyz/anchor";
import { PublicKey, SystemProgram } from "@solana/web3.js";
import { expect } from "chai";

describe("snowbank", () => {
  anchor.setProvider(anchor.AnchorProvider.env());
  const program = anchor.workspace.Snowbank as Program;

  it("builds", async () => {
    expect(program.programId).to.be.ok;
  });
});
