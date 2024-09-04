use crate::constants::SYSTEM_PROGRAM_ADDRESS;
use crate::pb::sf::solana::transfer::v1::SystemTransfer;
use borsh::BorshDeserialize;
use substreams_solana::pb::sf::solana::r#type::v1::Block;

#[derive(Debug, BorshDeserialize)]
pub struct TransferLayout {
    pub lamports: u64,
}

#[derive(Clone)]
pub struct TransferInfo {
    pub from: String,
    pub to: String,
    pub lamports: u64,
}

pub fn map_transfers(block: Block) -> Option<Vec<SystemTransfer>> {
    let transactions = block.transactions;
    let mut transfers = vec![];

    for (tx_idx, transaction) in transactions.into_iter().enumerate() {
        let Some(meta) = transaction.meta.clone() else {
            continue;
        };
        let Some(tx) = transaction.transaction.clone() else {
            continue;
        };
        let Some(msg) = tx.clone().message else {
            continue;
        };
        let ixs = msg.instructions;
        let accounts = transaction.resolved_accounts_as_strings();

        //add data addresses store
        for (idx, ix) in ixs.into_iter().enumerate() {
            let program = &accounts[ix.program_id_index as usize];
            if program != SYSTEM_PROGRAM_ADDRESS {
                continue;
            }
            let maybe_transfer = parse_system_instruction(&ix.data, &ix.accounts, &accounts);
            if let Some(info) = maybe_transfer {
                let st = SystemTransfer {
                    slot: block.slot,
                    tx_id: bs58::encode(&tx.signatures[0]).into_string(),
                    instruction_index: idx as u32,
                    inner_instruction_index: 0,
                    is_inner_instruction: false,
                    from: info.from,
                    to: info.to,
                    lamports: info.lamports,
                    transaction_index: tx_idx as u32,
                    tx_fee: meta.fee,
                };
                transfers.push(st)
            }
            // add inner instruction transfers
            meta.inner_instructions
                .iter()
                .filter(|i| i.index as usize == idx)
                .for_each(|inner_ixs| {
                    inner_ixs.instructions.iter().enumerate().for_each(
                        |(inner_ix_idx, inner_ix)| {
                            let program = &accounts[inner_ix.program_id_index as usize];
                            if program != SYSTEM_PROGRAM_ADDRESS {
                                return;
                            }
                            let maybe_transfer = parse_system_instruction(
                                &inner_ix.data,
                                &inner_ix.accounts,
                                &accounts,
                            );
                            if let Some(info) = maybe_transfer {
                                let st = SystemTransfer {
                                    slot: block.slot,
                                    tx_id: bs58::encode(&tx.signatures[0]).into_string(),
                                    instruction_index: idx as u32,
                                    inner_instruction_index: inner_ix_idx as u32,
                                    is_inner_instruction: true,
                                    from: info.from,
                                    to: info.to,
                                    lamports: info.lamports,
                                    transaction_index: tx_idx as u32,
                                    tx_fee: meta.fee,
                                };
                                transfers.push(st)
                            }
                        },
                    )
                });
        }
    }

    Some(transfers)
}

pub fn parse_system_instruction(
    instruction_data: &[u8],
    account_indices: &[u8],
    accounts: &[String],
) -> Option<TransferInfo> {
    //discriminator type = u32
    let (disc_bytes, rest) = instruction_data.split_at(4);
    //ref: https://docs.rs/solana-program/latest/solana_program/system_instruction/enum.SystemInstruction.html
    match disc_bytes[0] {
        2 => {
            let TransferLayout { lamports, .. } = TransferLayout::try_from_slice(rest).ok()?;
            let from_idx = account_indices[0];
            let to_idx = account_indices[1];
            Some(TransferInfo {
                from: accounts[from_idx as usize].clone(),
                to: accounts[to_idx as usize].clone(),
                lamports,
            })
        }
        _ => None,
    }
}
