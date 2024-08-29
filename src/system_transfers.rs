use crate::pb::sf::solana::r#type::v1::ConfirmedTransaction;
use crate::pb::sf::solana::transfer::v1::SystemTransfer;
use borsh::BorshDeserialize;

pub const SYSTEM_PROGRAM_ADDRESS: &str = "11111111111111111111111111111111";

#[derive(Debug, BorshDeserialize)]
pub struct TransferLayout {
    pub lamports: u64,
}

pub fn parse_transactions(
    tx: substreams_solana::pb::sf::solana::r#type::v1::ConfirmedTransaction,
) -> Option<Vec<SystemTransfer>> {
    let meta = tx.meta?;
    let msg = tx.transaction?.message?;
    let ixs = msg.instructions;
    let accounts = msg
        .account_keys
        .into_iter()
        .map(|a| bs58::encode(a).into_string())
        .collect::<Vec<_>>();

    let mut transfers = vec![];
    //add data addresses store
    for (idx, ix) in ixs.into_iter().enumerate() {
        let program = &accounts[ix.program_id_index as usize];
        if program != SYSTEM_PROGRAM_ADDRESS {
            continue;
        }
        let maybe_transfer = parse_system_instruction(&ix.data, &ix.accounts, &accounts);
        if let Some(t) = maybe_transfer {
            transfers.push(t)
        }
        // add inner instruction transfers
        meta.inner_instructions
            .iter()
            .filter(|i| i.index as usize == idx)
            .for_each(|inner_ixs| {
                inner_ixs.instructions.iter().for_each(|inner_ix| {
                    let program = &accounts[inner_ix.program_id_index as usize];
                    if program != SYSTEM_PROGRAM_ADDRESS {
                        return;
                    }
                    let maybe_transfer =
                        parse_system_instruction(&inner_ix.data, &inner_ix.accounts, &accounts);
                    if let Some(t) = maybe_transfer {
                        transfers.push(t)
                    }
                })
            });
    }

    Some(transfers)
}

pub fn parse_system_instruction(
    instruction_data: &Vec<u8>,
    account_indices: &Vec<u8>,
    accounts: &Vec<String>,
) -> Option<SystemTransfer> {
    let (disc_bytes, rest) = instruction_data.split_at(4);
    //ref: https://docs.rs/solana-program/latest/solana_program/system_instruction/enum.SystemInstruction.html
    match disc_bytes[0] {
        2 => {
            let TransferLayout { lamports, .. } = TransferLayout::try_from_slice(rest).ok()?;
            let from_idx = account_indices[0];
            let to_idx = account_indices[1];
            substreams::log::println(format!(
                "from {:?} to {:?} accounts {:?}",
                from_idx, to_idx, accounts
            ));
            Some(SystemTransfer {
                from: accounts[from_idx as usize].clone(),
                to: accounts[to_idx as usize].clone(),
                lamports,
            })
        }
        _ => None,
    }
}
