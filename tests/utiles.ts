import { AnchorProvider, utils } from '@project-serum/anchor'
import { ConfirmOptions, Connection, Keypair, PublicKey, sendAndConfirmRawTransaction, Transaction } from "@solana/web3.js"

export const SEED = 'EVENTSNAP'
export const STATE_SEED = 'STATE'

export const getProgramAuthority = async (programId: PublicKey) => {
    const [programAuthority, nonce] = await PublicKey.findProgramAddress(
        [Buffer.from(SEED)],
        programId
    )

    return {
        programAuthority,
        nonce
    }
}

export const getStateAddress = async (programId: PublicKey) => {
    const [address, bump] = await PublicKey.findProgramAddress(
        [Buffer.from(utils.bytes.utf8.encode(STATE_SEED))],
        programId
    )

    return {
        state: address,
        bump
    }
}

export const signAndSend = async (
    tx: Transaction,
    signers: Keypair[],
    connection: Connection,
    opts?: ConfirmOptions
) => {
    tx.setSigners(...signers.map(s => s.publicKey))
    const blockhash = await connection.getRecentBlockhash(
        opts?.commitment ?? AnchorProvider.defaultOptions().commitment
    )
    tx.recentBlockhash = blockhash.blockhash
    tx.partialSign(...signers)
    const rawTx = tx.serialize()
    return await sendAndConfirmRawTransaction(
        connection,
        rawTx,
        opts ?? AnchorProvider.defaultOptions()
    )
}

export const sleep = async (ms: number) => {
    return await new Promise(resolve => setTimeout(resolve, ms))
}