mod constants;
mod error;
mod pb;
mod primitives;
mod sandwiches;
mod system_transfers;
mod transaction_details;

use crate::constants::JITO_TIPS;
use crate::pb::sf::solana::dex::sandwiches::v1::{NormalizedSwap, SandwichOutput, SwapsOutput};
use crate::pb::sf::solana::dex::trades::v1::Output;
use crate::pb::sf::solana::transaction::details::v1::{
    TransactionDetailsOutput, TransactionDetailsStore,
};
use crate::pb::sf::solana::transfer::v1::TransferOutput;
use crate::sandwiches::map_sandwiches;
use crate::system_transfers::map_transfers;
use crate::transaction_details::get_transaction_details;
use substreams_solana::pb::sf::solana::r#type::v1::Block;

#[substreams::handlers::map]
fn map_system_transfers(block: Block) -> Result<TransferOutput, substreams::errors::Error> {
    //filter and unpack solana-program instructions
    if let Some(transfers) = map_transfers(block) {
        return Ok(TransferOutput {
            transfers: transfers.into_iter().collect::<Vec<_>>(),
        });
    }
    Ok(TransferOutput {
        transfers: Vec::new(),
    })
}

#[substreams::handlers::map]
fn map_transaction_details(
    block: Block,
) -> Result<TransactionDetailsOutput, substreams::errors::Error> {
    //create a map of tx_id to details
    Ok(TransactionDetailsOutput {
        data: get_transaction_details(block)?,
    })
}

#[substreams::handlers::map]
fn map_transaction_details_store(
    block: TransactionDetailsOutput,
) -> Result<TransactionDetailsStore, substreams::errors::Error> {
    //create a map of tx_id to details
    let map = block
        .data
        .into_iter()
        .map(|d| (d.tx_id.clone(), d))
        .collect();
    Ok(TransactionDetailsStore { data: map })
}

#[substreams::handlers::map]
fn map_tips(out: TransferOutput) -> Result<TransferOutput, substreams::errors::Error> {
    let transfers = out
        .transfers
        .into_iter()
        .filter(|t| JITO_TIPS.contains(&&*t.to))
        .collect::<Vec<_>>();
    Ok(TransferOutput { transfers })
}

#[substreams::handlers::map]
fn map_trades(
    dex_trades: Output,
    transaction_info: TransactionDetailsStore,
) -> Result<SwapsOutput, substreams::errors::Error> {
    let swaps = dex_trades
        .data
        .into_iter()
        .map(|item| {
            let priority_fee = transaction_info
                .data
                .get(&item.tx_id)
                .map(|d| d.priority_fee)
                .unwrap_or(0);
            let transaction_index = transaction_info
                .data
                .get(&item.tx_id)
                .map(|d| d.transaction_index)
                .unwrap_or(0);
            NormalizedSwap::from_trade(item, priority_fee, transaction_index)
        })
        .collect::<Vec<_>>();
    Ok(SwapsOutput { data: swaps })
}

#[substreams::handlers::map]
fn map_to_sandwiches(
    swaps_output: SwapsOutput,
) -> Result<SandwichOutput, substreams::errors::Error> {
    let sandwiches = map_sandwiches(swaps_output.data);
    Ok(SandwichOutput { data: sandwiches })
}
