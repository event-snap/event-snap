import { utils } from '@project-serum/anchor'
import { PublicKey } from "@solana/web3.js"

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
        address,
        bump
    }
}