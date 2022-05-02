import * as anchor from "@project-serum/anchor";
import { Program } from "@project-serum/anchor";
import { EpicProject } from "../target/types/epic_project";

describe("epic_project", () => {
  // Configure the client to use the local cluster.
  anchor.setProvider(anchor.AnchorProvider.env());

  const program = anchor.workspace.EpicProject as Program<EpicProject>;

  it("Is initialized!", async () => {
    // Add your test here.
    // const tx = await program.methods.initialize().rpc();
    // console.log("Your transaction signature", tx);

    console.log("🚀 Starting test...");

    // We call our method and wait for our local
    // validator to "mine" the instruction.
    const tx = await program.methods.startStuffOff().rpc();

    console.log("📝 Your transaction signature", tx);
  });
});
