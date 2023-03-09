import * as anchor from "@project-serum/anchor";
import { Program } from "@project-serum/anchor";
import { EventSpan } from "../target/types/event_span";
import { Keypair, SystemProgram, SYSVAR_RENT_PUBKEY, Transaction } from '@solana/web3.js'
import { getEventBufferAddress, getProgramAuthority, getStateAddress, signAndSend, sleep } from "./utiles";

describe("event-span", () => {
  // Configure the client to use the local cluster.
  const provider = anchor.AnchorProvider.env()
  const connection = provider.connection
  const program = anchor.workspace.EventSpan as Program<EventSpan>;
  const admin = Keypair.generate()
  anchor.setProvider(provider);

  before(async () => {
    await connection.requestAirdrop(admin.publicKey, 1e9)
    await sleep(500)
  })

  it("Is initialized!", async () => {
    const { programAuthority, nonce } = await getProgramAuthority(program.programId)
    const { state } = await getStateAddress(program.programId)
    const { eventBuffer } = await getEventBufferAddress(program.programId)

    const ix = program.instruction.initialize(nonce, {
      accounts: {
        state,
        eventBuffer,
        admin: admin.publicKey,
        programAuthority,
        rent: SYSVAR_RENT_PUBKEY,
        systemProgram: SystemProgram.programId
      }
    })
    const tx = new Transaction().add(ix)
    await signAndSend(tx, [admin], connection)
  });
});
