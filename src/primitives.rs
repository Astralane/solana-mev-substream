use crate::pb::sf::solana::dex::sandwiches::v1::NormalizedSwap;
use crate::pb::sf::solana::dex::trades::v1::TradeData;
use borsh::{BorshDeserialize, BorshSerialize};

impl NormalizedSwap {
    pub fn from_trade(value: TradeData, priority_fee: u64, tx_index: u32) -> Self {
        //if amount is -ve then it is token_in
        let (token_in, token_out, amount_in, amount_out) = if value.base_amount.is_sign_negative() {
            (
                value.quote_mint,
                value.base_mint,
                value.quote_amount.abs(),
                value.base_amount.abs(),
            )
        } else {
            (
                value.base_mint,
                value.quote_mint,
                value.base_amount.abs(),
                value.quote_amount.abs(),
            )
        };
        NormalizedSwap {
            block_date: value.block_date,
            multi_location: format!(
                "{}/{}/{}",
                value.tx_id, value.instruction_index, value.inner_instruxtion_index
            ),
            instruction_index: value.instruction_index,
            is_inner_instruction: value.is_inner_instruction,
            inner_instruction_index: value.instruction_index,
            block_slot: value.block_slot,
            tx_id: value.tx_id,
            signer: value.signer,
            pool_address: value.pool_address,
            token_in,
            token_out,
            amount_in,
            amount_out,
            tx_fee: value.txn_fee,
            inner_program: value.inner_program,
            outer_program: value.outer_program,
            priority_fee,
            transaction_index: tx_index,
        }
    }
}

#[derive(Clone, Debug, PartialEq, Eq, Hash)]
pub struct PossibleSandwich {
    pub eoa: String,
    pub possible_frontruns: Vec<String>,
    pub possible_backrun: String,
    // Mapping of possible frontruns to the set of possible victims.
    // By definition the victims of latter transactions can also be victims of the former
    pub victims: Vec<Vec<String>>,
}

#[derive(Clone, Debug, BorshDeserialize, BorshSerialize)]
pub enum ComputeBudgetInstruction {
    Unused, // deprecated variant, reserved value.
    /// Request a specific transaction-wide program heap region size in bytes.
    /// The value requested must be a multiple of 1024. This new heap region
    /// size applies to each program executed in the transaction, including all
    /// calls to CPIs.
    RequestHeapFrame(u32),
    /// Set a specific compute unit limit that the transaction is allowed to consume.
    SetComputeUnitLimit(u32),
    /// Set a compute unit price in "micro-lamports" to pay a higher transaction
    /// fee for higher transaction prioritization.
    SetComputeUnitPrice(u64),
    /// Set a specific transaction-wide account data size limit, in bytes, is allowed to load.
    SetLoadedAccountsDataSizeLimit(u32),
}
