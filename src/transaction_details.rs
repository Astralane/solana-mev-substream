use crate::constants::{
    COMPUTE_BUDGET_PROGRAM_ADDRESS, DEFAULT_INSTRUCTION_COMPUTE_UNIT_LIMIT,
    MICRO_LAMPORTS_PER_LAMPORT,
};
use crate::error::MevSubstreamError;
use crate::pb::sf::solana::transaction::details::v1::TransactionDetails;
use crate::primitives::ComputeBudgetInstruction;
use borsh::BorshDeserialize;
use substreams_solana::pb::sf::solana::r#type::v1::Block;

pub fn get_transaction_details(block: Block) -> Result<Vec<TransactionDetails>, MevSubstreamError> {
    //create a map of tx_id to details
    let mut tx_details = Vec::new();
    for (idx, confirmed_transaction) in block.transactions.into_iter().enumerate() {
        let accounts = confirmed_transaction.resolved_accounts_as_strings();

        let Some(meta) = confirmed_transaction.meta.clone() else {
            continue;
        };
        let Some(transaction) = confirmed_transaction.transaction.clone() else {
            continue;
        };
        let tx_id = bs58::encode(&transaction.signatures[0]).into_string();

        let mut compute_unit_price: Option<u64> = None;
        let mut compute_units: Option<u32> = None;
        let mut non_compute_instructions: u32 = 0;

        //check for instruction to compute program
        let Some(message) = transaction.message else {
            continue;
        };
        for (ix_index, ix) in message.instructions.iter().enumerate() {
            if accounts[ix.program_id_index as usize] == COMPUTE_BUDGET_PROGRAM_ADDRESS {
                //discriminator type u8
                let (discriminator, rest) = ix.data.split_at(1);
                match discriminator[0] {
                    2 => {
                        //set limit decode u32
                        let value:[u8; 4] = rest[..4].try_into().expect("value is not 4 bytes");
                        let limit = u32::from_le_bytes(value);
                        compute_units = Some(limit);
                    }
                    3 => {
                        //set price decode u64
                        let value:[u8; 8] = rest[..8].try_into().expect("value is not 8 bytes");
                        let price = u64::from_le_bytes(value);
                        compute_unit_price = Some(price);
                    }
                    _ => { /*do nothing*/ }
                }
            } else {
                non_compute_instructions += 1;
            }
        }

        tx_details.push(TransactionDetails {
            slot: block.slot,
            tx_id,
            transaction_index: idx as u32,
            signer: accounts[0].clone(),
            tx_fee: meta.fee,
            priority_fee: compute_priority_fee(
                compute_unit_price.unwrap_or(0),
                compute_units.unwrap_or(
                    non_compute_instructions
                        .saturating_mul(DEFAULT_INSTRUCTION_COMPUTE_UNIT_LIMIT.into()),
                ),
            ),
        });
    }
    Ok(tx_details)
}
pub fn compute_priority_fee(compute_unit_price: u64, compute_unit_limit: u32) -> u64 {
    let micro_lamport_fee = (compute_unit_price as u128).saturating_mul(compute_unit_limit as u128);
    micro_lamport_fee
        .saturating_add(MICRO_LAMPORTS_PER_LAMPORT.saturating_sub(1) as u128)
        .checked_div(MICRO_LAMPORTS_PER_LAMPORT as u128)
        .and_then(|fee| u64::try_from(fee).ok())
        .unwrap_or(u64::MAX)
}

#[cfg(test)]
mod test {
    use borsh::BorshDeserialize;
    use crate::primitives::ComputeBudgetInstruction;

    #[test]
    fn test_compute_budget_decoder() {
        let ix = "6LRurBWDYzjLoUbGDs1HPLVCuuLVSe8xqYo";
        let vec_u8 = bs58::decode(ix).into_vec().unwrap();
        let decoded = ComputeBudgetInstruction::try_from_slice(&vec_u8);
        assert!(decoded.is_ok());
    }
}