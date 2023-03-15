import * as anchor from "@project-serum/anchor";
import { BN, Program } from "@project-serum/anchor";
import { EventSpan } from "../target/types/event_span";
import { Keypair, SystemProgram, SYSVAR_RENT_PUBKEY, Transaction } from '@solana/web3.js'
import { assertThrowsAsync, getEventAddress, getEventBufferAddress, getProgramAuthority, getStateAddress, signAndSend, sleep } from "./utiles";

describe("event-span", () => {
  // Configure the client to use the local cluster.
  const provider = anchor.AnchorProvider.env()
  const connection = provider.connection
  const program = anchor.workspace.EventSpan as Program<EventSpan>;
  const admin = Keypair.generate()
  const noAdmin = Keypair.generate()
  const solAmount = 1e9
  anchor.setProvider(provider);

  before(async () => {
    await connection.requestAirdrop(admin.publicKey, solAmount)
    await connection.requestAirdrop(noAdmin.publicKey, solAmount)
    await sleep(500)
  })

  it("Is initialized!", async () => {
    const { programAuthority, nonce } = await getProgramAuthority(program.programId)
    const { state } = await getStateAddress(program.programId)
    const { eventBuffer } = await getEventBufferAddress(program.programId)

    const initIx = program.instruction.initialize(nonce, {
      accounts: {
        state,
        eventBuffer,
        admin: admin.publicKey,
        programAuthority,
        rent: SYSVAR_RENT_PUBKEY,
        systemProgram: SystemProgram.programId
      }
    })
    await signAndSend(new Transaction().add(initIx), [admin], connection)

    {
      const balanceAmount = await connection.getBalance(eventBuffer)
      const adminAmount = await connection.getBalance(admin.publicKey)
      console.log(`balance amount = ${balanceAmount}`)
      console.log(`admin amount = ${adminAmount}`)
    }

    const amountToDeposit = new BN(300000000)
    const amountToWithdraw = new BN(300000000).divn(10)
    const depositIx = program.instruction.depositEventBuffer(amountToDeposit, {
      accounts: {
        state,
        depositor: admin.publicKey,
        eventBuffer,
        rent: SYSVAR_RENT_PUBKEY,
        systemProgram: SystemProgram.programId
      }
    })
    await signAndSend(new Transaction().add(depositIx), [admin], connection)

    {
      const balanceAmount = await connection.getBalance(eventBuffer)
      const adminAmount = await connection.getBalance(admin.publicKey)
      console.log(`balance amount = ${balanceAmount}`)
      console.log(`admin amount = ${adminAmount}`)
    }

    const withdrawByNoAdminIx = program.instruction.withdrawEventBuffer(amountToWithdraw, {
      accounts: {
        state,
        eventBuffer,
        admin: noAdmin.publicKey,
        rent: SYSVAR_RENT_PUBKEY,
        programAuthority,
        systemProgram: SystemProgram.programId
      }
    })
    await assertThrowsAsync(signAndSend(new Transaction().add(withdrawByNoAdminIx), [noAdmin], connection))

    const withdrawByAdminIx = program.instruction.withdrawEventBuffer(amountToWithdraw, {
      accounts: {
        state,
        eventBuffer,
        admin: admin.publicKey,
        rent: SYSVAR_RENT_PUBKEY,
        programAuthority,
        systemProgram: SystemProgram.programId
      }
    })
    await signAndSend(new Transaction().add(withdrawByAdminIx), [admin], connection)

    {
      const balanceAmount = await connection.getBalance(eventBuffer)
      const adminAmount = await connection.getBalance(admin.publicKey)
      console.log(`balance amount = ${balanceAmount}`)
      console.log(`admin amount = ${adminAmount}`)
    }

    const { eventAddress, bump } = await getEventAddress(program.programId)
    console.log(eventAddress.toString())
    console.log(bump)

    const trigger2Ix = program.instruction.triggerEventsCreationTwo({
      accounts: {
        state,
        eventBuffer,
        eventAddress: eventAddress,
        signer: noAdmin.publicKey,
        rent: SYSVAR_RENT_PUBKEY,
        systemProgram: SystemProgram.programId
      },
    })
    await signAndSend(new Transaction().add(trigger2Ix), [noAdmin], connection)

    await sleep(500)

    const event = await program.account.eventStruct.fetch(eventAddress)
    console.log(event)
  });
});
