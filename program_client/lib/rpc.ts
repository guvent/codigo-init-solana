// This file is auto-generated from the CIDL source.
// Editing this file directly is not recommended as it may be overwritten.

import * as pda from "./pda";
import * as T from "./types";
import {
    Commitment,
    Connection,
    GetAccountInfoConfig,
    Keypair,
    PublicKey,
    sendAndConfirmTransaction,
    SystemProgram,
    Transaction,
    TransactionInstruction,
    TransactionSignature,
} from "@solana/web3.js";
import {deserialize, serialize} from "borsh";


let _programId: PublicKey;
let _connection: Connection;

export const initializeClient = (
    programId: PublicKey,
    connection: Connection
) => {
    _programId = programId;
    _connection = connection;
};

export enum CounterInstruction {
/**
 * Accounts:
 * 0. `[writable, signer]` fee_payer: {@link PublicKey} Auto-generated, default fee payer
 * 1. `[]` greeting_account: {@link GreetingAccount} 
 */
    Increment = 0,

/**
 * Accounts:
 * 0. `[writable, signer]` fee_payer: {@link PublicKey} Auto-generated, default fee payer
 * 1. `[]` greeting_account: {@link GreetingAccount} 
 */
    Decrement = 1,
}

export type IncrementArgs = {
    feePayer: PublicKey;
    greetingAccount: PublicKey;
};


/**
 * ### Returns a {@link TransactionInstruction}
 * Accounts:
 * 0. `[writable, signer]` fee_payer: {@link PublicKey} Auto-generated, default fee payer
 * 1. `[]` greeting_account: {@link GreetingAccount} 
 */
export const increment = (args: IncrementArgs): TransactionInstruction => {
    const data = serialize(
        {
            struct: {
                id: "u8",
            },
        },
        {
            id: CounterInstruction.Increment,
        }
    );


    return new TransactionInstruction({
        data: Buffer.from(data),
        keys: [
            {pubkey: args.feePayer, isSigner: true, isWritable: true},
            {pubkey: args.greetingAccount, isSigner: false, isWritable: false},
        ],
        programId: _programId,
    });
};

/**
 * ### Returns a {@link TransactionSignature}
 * Accounts:
 * 0. `[writable, signer]` fee_payer: {@link PublicKey} Auto-generated, default fee payer
 * 1. `[]` greeting_account: {@link GreetingAccount} 
 */
export const incrementSendAndConfirm = async (
    args: Omit<IncrementArgs, "feePayer"> & { 
        signers: { feePayer: Keypair, }
 }
): Promise<TransactionSignature> => {
    const trx = new Transaction();


    trx.add(increment({
        ...args,
        feePayer: args.signers.feePayer.publicKey,
    }));

    return await sendAndConfirmTransaction(
        _connection,
        trx,
        [args.signers.feePayer, ]
    );
};

export type DecrementArgs = {
    feePayer: PublicKey;
    greetingAccount: PublicKey;
};


/**
 * ### Returns a {@link TransactionInstruction}
 * Accounts:
 * 0. `[writable, signer]` fee_payer: {@link PublicKey} Auto-generated, default fee payer
 * 1. `[]` greeting_account: {@link GreetingAccount} 
 */
export const decrement = (args: DecrementArgs): TransactionInstruction => {
    const data = serialize(
        {
            struct: {
                id: "u8",
            },
        },
        {
            id: CounterInstruction.Decrement,
        }
    );


    return new TransactionInstruction({
        data: Buffer.from(data),
        keys: [
            {pubkey: args.feePayer, isSigner: true, isWritable: true},
            {pubkey: args.greetingAccount, isSigner: false, isWritable: false},
        ],
        programId: _programId,
    });
};

/**
 * ### Returns a {@link TransactionSignature}
 * Accounts:
 * 0. `[writable, signer]` fee_payer: {@link PublicKey} Auto-generated, default fee payer
 * 1. `[]` greeting_account: {@link GreetingAccount} 
 */
export const decrementSendAndConfirm = async (
    args: Omit<DecrementArgs, "feePayer"> & { 
        signers: { feePayer: Keypair, }
 }
): Promise<TransactionSignature> => {
    const trx = new Transaction();


    trx.add(decrement({
        ...args,
        feePayer: args.signers.feePayer.publicKey,
    }));

    return await sendAndConfirmTransaction(
        _connection,
        trx,
        [args.signers.feePayer, ]
    );
};

// Getters

export const getGreetingAccount = async (
    publicKey: PublicKey,
    commitmentOrConfig: Commitment | GetAccountInfoConfig | undefined = "processed"
): Promise<T.GreetingAccount | undefined> => {
    const buffer = await _connection.getAccountInfo(publicKey, commitmentOrConfig);

    if (!buffer) {
        return undefined
    }

    if (buffer.data.length <= 0) {
        return undefined
    }

    return T.decodeGreetingAccount(deserialize(T.GreetingAccountSchema, buffer.data) as Record<string, unknown>);
}


// Websocket Events

