mod constants;
mod error;
mod pb;
mod primitives;
mod sandwiches;
mod system_transfers;
mod transaction_info;

use crate::constants::JITO_TIPS;
use crate::pb::sf::solana::dex::sandwiches::v1::SandwichOutput;
use crate::pb::sf::solana::dex::trades::v1::Output;
use crate::pb::sf::solana::transaction::info::v1::TransactionInfoStore;
use crate::pb::sf::solana::transfer::v1::TransferOutput;
use crate::sandwiches::map_sandwiches;
use crate::system_transfers::map_transfers;
use crate::transaction_info::get_transaction_info;
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
fn map_transaction_info(block: Block) -> Result<TransactionInfoStore, substreams::errors::Error> {
    //create a map of tx_id to details
    let info = get_transaction_info(block)?;
    Ok(TransactionInfoStore { store: info })
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
    transaction_info: TransactionInfoStore,
) -> Result<SandwichOutput, substreams::errors::Error> {
    let sandwiches = map_sandwiches(dex_trades.data, transaction_info);
    Ok(SandwichOutput { data: sandwiches })
}
