use crate::pb::sf::solana::dex::sandwiches::v1::SwapDto;
use crate::pb::sf::solana::dex::trades::v1::TradeData;

#[derive(Clone)]
pub struct TransferInfo {
    pub from: String,
    pub to: String,
    pub lamports: u64,
}

#[derive(Clone, Debug, PartialEq)]
pub struct NormalizedSwap {
    pub(crate) id: String,
    pub(crate) data: TradeData,
}

impl From<TradeData> for SwapDto {
    fn from(value: TradeData) -> Self {
        //if amount is -ve then it is token_in
        let (token_in, token_out, amount_in, amount_out) = if value.base_amount.is_sign_negative() {
            (
                value.base_mint,
                value.quote_mint,
                value.base_amount.abs(),
                value.quote_amount.abs(),
            )
        } else {
            (
                value.quote_mint,
                value.base_mint,
                value.quote_amount.abs(),
                value.base_amount.abs(),
            )
        };
        SwapDto {
            multilocation: format!(
                "{}/{}/{}",
                value.tx_id, value.instruction_index, value.inner_instruxtion_index
            ),
            block_slot: value.block_slot,
            tx_id: value.tx_id,
            signer: value.signer,
            pool_address: value.pool_address,
            token_in,
            token_out,
            amount_in,
            amount_out,
            txn_fee: value.txn_fee,
        }
    }
}
impl From<TradeData> for NormalizedSwap {
    fn from(value: TradeData) -> Self {
        let id = format!(
            "{}/{}/{}",
            value.tx_id, value.instruction_index, value.inner_instruxtion_index
        );
        NormalizedSwap { id, data: value }
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
