import * as anchor from "@project-serum/anchor";
import { Program } from "@project-serum/anchor";
import { EpicProject } from "../target/types/epic_project";

const { SystemProgram } = anchor.web3;

describe("epic_project", () => {
  // Configure the client to use the local cluster.
  /* We set it before but we needed to update it, so 
     that it can communicate with our frontend! */
  const provider = anchor.AnchorProvider.env();
  anchor.setProvider(provider);

  const program = anchor.workspace.EpicProject as Program<EpicProject>;

  it("Is initialized!", async () => {
    console.log("🚀 Starting test...");

    // Create an account keypair for our program to use.
    const baseAccount = anchor.web3.Keypair.generate();

    // Call start_stuff_off, pass it the params it needs!
    let tx = await program.methods
      .startStuffOff()
      .accounts({
        baseAccount: baseAccount.publicKey,
        user: provider.wallet.publicKey,
        systemProgram: SystemProgram.programId,
      })
      .signers([baseAccount])
      .rpc();

    console.log("📝 Your transaction signature", tx);

    // Fetch data from the account.
    let account = await program.account.baseAccount.fetch(
      baseAccount.publicKey
    );

    console.log("👀 GIF Count", account.totalGifs.toString());
    // console.log("Wallet Address: ", provider.wallet.publicKey.toString());

    // Call add_gif
    // rpc -> Remote Procedure Call
    await program.methods
      .addGif("https://media0.giphy.com/media/dUf2a9CIVydkb5oHvA/giphy.webp")
      .accounts({
        baseAccount: baseAccount.publicKey,
        user: provider.wallet.publicKey,
      })
      .rpc();

    account = await program.account.baseAccount.fetch(baseAccount.publicKey);
    console.log("👀 GIF Count", account.totalGifs.toString());

    // Access gif_list on the account!
    console.log("👀 GIF List", account.gifList);

    // We call our method and wait for our local
    // validator to "mine" the instruction.
    // const tx = await program.methods.startStuffOff().rpc();
  });
});
