import { AnchorProvider, utils } from '@project-serum/anchor'
import { ConfirmOptions, Connection, Keypair, PublicKey, sendAndConfirmRawTransaction, Transaction } from "@solana/web3.js"

export const EVENT_BUFFER_SEED = 'EVENT_BUFFER'
export const EVENT_AUTHORITY_SEED = 'EVENTSNAP'
export const MOCKED_EVENT_SEED = 'MOCKED_EVENT'

export const getEventBufferAddress = async (programId: PublicKey) => {
    const [address, bump] = await PublicKey.findProgramAddress(
        [Buffer.from(utils.bytes.utf8.encode(EVENT_BUFFER_SEED))],
        programId
    )

    return {
        eventBuffer: address,
        bump
    }
}

export const getEventAuthorityAddress = async (programId: PublicKey) => {
    const [address, bump] = await PublicKey.findProgramAddress(
        [Buffer.from(utils.bytes.utf8.encode(EVENT_AUTHORITY_SEED))],
        programId
    )

    return {
        eventAuthority: address,
        bump
    }
}

export const getEventAddress = async (programId: PublicKey) => {
    const [address, bump] = await PublicKey.findProgramAddress(
        [Buffer.from(utils.bytes.utf8.encode(MOCKED_EVENT_SEED))],
        programId
    )
    return {
        eventAddress: address,
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

export async function assertThrowsAsync(fn: Promise<any>, word?: string) {
    try {
        await fn
    } catch (e: any) {
        let err
        if (e.code) {
            err = '0x' + e.code.toString(16)
        } else {
            err = e.toString()
        }
        if (word) {
            const regex = new RegExp(`${word}$`)
            if (!regex.test(err)) {
                console.log(err)
                throw new Error('Invalid Error message')
            }
        }
        return
    }
    throw new Error('Function did not throw error')
}

export const sleep = async (ms: number) => {
    return await new Promise(resolve => setTimeout(resolve, ms))
}