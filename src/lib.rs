mod pb;
mod primitives;
mod sandwiches;
mod system_transfers;

use crate::pb::sf::solana::dex::sandwiches::v1::SandwichOutput;
use crate::pb::sf::solana::dex::trades::v1::Output;
use crate::pb::sf::solana::transfer::v1::TransferOutput;
use crate::sandwiches::map_sandwiches;
use crate::system_transfers::parse_transactions;
use substreams_solana::pb::sf::solana::r#type::v1::Block;

#[substreams::handlers::map]
fn map_system_transfers(block: Block) -> Result<TransferOutput, substreams::errors::Error> {
    let mut block_height: Option<u64> = None;
    if let Some(v) = block.block_height.as_ref() {
        block_height = Some(v.block_height)
    }

    //filter and unpack solana-program instructions
    let transfers = block
        .transactions
        .into_iter()
        .filter_map(parse_transactions)
        .filter(|trs| !trs.is_empty())
        .flatten()
        .collect::<Vec<_>>();
    Ok(TransferOutput { transfers })
}

#[substreams::handlers::map]
fn map_trades(dex_trades: Output) -> Result<SandwichOutput, substreams::errors::Error> {
    let sandwiches = map_sandwiches(dex_trades.data);
    Ok(SandwichOutput { data: sandwiches })
}
