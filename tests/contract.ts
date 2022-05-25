import * as anchor from "@project-serum/anchor";
import { Program } from "@project-serum/anchor";
import { Contract } from "../target/types/contract";

describe("contract", () => {
  // Configure the client to use the local cluster.
  anchor.setProvider(anchor.AnchorProvider.env());

  const program = anchor.workspace.Contract as Program<Contract>;
  const privateKey = Uint8Array.from([97,189,209,164,72,48,166,55,26,227,84,68,194,26,237,210,129,227,166,186,159,114,219,236,202,253,58,62,57,17,193,144,24,242,239,222,202,185,41,253,195,94,254,152,169,189,248,95,170,176,11,134,250,144,208,224,163,23,134,64,57,115,28,40]);
  const signer = anchor.web3.Keypair.fromSecretKey(privateKey);
  const receiver = anchor.web3.Keypair.generate();

  it("Is reward!", async () => {
    let lamports = new anchor.BN(1_000_000_000);
    const tx = await program.rpc.reward(lamports, {
      accounts: {
        signer: signer.publicKey,
        receiver: receiver.publicKey,
        systemProgram: anchor.web3.SystemProgram.programId
      },
    });

    console.log("Your transaction signature", tx);
  });
});
