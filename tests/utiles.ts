import { AnchorProvider, utils } from '@project-serum/anchor'
import { ConfirmOptions, Connection, Keypair, PublicKey, sendAndConfirmRawTransaction, Transaction } from "@solana/web3.js"

export const SEED = 'EVENTSNAP'
export const STATE_SEED = 'STATE'
export const EVNET_BUFFER = 'EVENT_BUFFER'

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

export const getEventBufferAddress = async (programId: PublicKey) => {
    const [address, bump] = await PublicKey.findProgramAddress(
        [Buffer.from(utils.bytes.utf8.encode(EVNET_BUFFER))],
        programId
    )

    return {
        eventBuffer: address,
        bump
    }
}

export const getEventAddress = async (programId: PublicKey) => {
    const [address, bump] = await PublicKey.findProgramAddress(
        [Buffer.from(utils.bytes.utf8.encode(EVNET_BUFFER))],
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