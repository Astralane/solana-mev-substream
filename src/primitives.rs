use crate::pb::sf::solana::dex::sandwiches::v1::SwapInfo;
use crate::pb::sf::solana::dex::trades::v1::TradeData;
use crate::pb::sf::solana::transaction::info::v1::TransactionDetails;
use borsh::BorshDeserialize;
use std::collections::HashMap;

#[derive(Clone)]
pub struct TransferInfo {
    pub from: String,
    pub to: String,
    pub lamports: u64,
}

//map of tx_id to NormalizedSwap
pub type SwapInfoStore = HashMap<String, NormalizedSwap>;
#[derive(Clone, Debug, PartialEq)]
pub struct NormalizedSwap {
    pub(crate) multi_location: String,
    pub(crate) tx_index: u32,
    pub(crate) inner: TradeData,
}

impl From<NormalizedSwap> for SwapInfo {
    fn from(value: NormalizedSwap) -> Self {
        //if amount is -ve then it is token_in
        let (token_in, token_out, amount_in, amount_out) =
            if value.inner.base_amount.is_sign_negative() {
                (
                    value.inner.base_mint,
                    value.inner.quote_mint,
                    value.inner.base_amount.abs(),
                    value.inner.quote_amount.abs(),
                )
            } else {
                (
                    value.inner.quote_mint,
                    value.inner.base_mint,
                    value.inner.quote_amount.abs(),
                    value.inner.base_amount.abs(),
                )
            };
        SwapInfo {
            multi_location: format!(
                "{}/{}/{}",
                value.inner.tx_id,
                value.inner.instruction_index,
                value.inner.inner_instruxtion_index
            ),
            instruction_index: value.inner.instruction_index,
            is_inner_instruction: value.inner.is_inner_instruction,
            inner_instruction_index: value.inner.instruction_index,
            transaction_index: value.tx_index,
            block_slot: value.inner.block_slot,
            tx_id: value.inner.tx_id,
            signer: value.inner.signer,
            pool_address: value.inner.pool_address,
            token_in,
            token_out,
            amount_in,
            amount_out,
            tx_fee: value.inner.txn_fee,
            inner_program: value.inner.inner_program,
            outer_program: value.inner.outer_program,
        }
    }
}
impl From<(TransactionDetails, TradeData)> for NormalizedSwap {
    fn from(value: (TransactionDetails, TradeData)) -> Self {
        let multi_location = format!(
            "{}/{}/{}",
            value.1.tx_id, value.1.instruction_index, value.1.inner_instruxtion_index
        );
        NormalizedSwap {
            multi_location,
            tx_index: value.0.transaction_index,
            inner: value.1,
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

#[derive(Clone, Debug, BorshDeserialize)]
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
