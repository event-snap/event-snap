import { PublicKey } from "@solana/web3.js"

export const SEED = 'EVENTSNAP'

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