mod constants;
mod pb;
mod primitives;
mod sandwiches;
mod system_transfers;

use crate::constants::JITO_TIPS;
use crate::pb::sf::solana::dex::sandwiches::v1::SandwichOutput;
use crate::pb::sf::solana::dex::trades::v1::Output;
use crate::pb::sf::solana::transfer::v1::TransferOutput;
use crate::sandwiches::map_sandwiches;
use crate::system_transfers::map_transfers;
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
fn map_jito_tips(out: TransferOutput) -> Result<TransferOutput, substreams::errors::Error> {
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
    block: Block,
) -> Result<SandwichOutput, substreams::errors::Error> {
    let sandwiches = map_sandwiches(dex_trades.data, block);
    Ok(SandwichOutput { data: sandwiches })
}
