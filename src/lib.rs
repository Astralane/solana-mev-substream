mod pb;
mod primitives;
mod utils;

use crate::pb::sf::solana::dex::sandwiches::v1::{Sandwich, SandwichOutput};
use crate::pb::sf::solana::dex::trades::v1::Output;
use crate::pb::sol::block::v1::BlockMeta;
use crate::utils::map_sandwiches;
use substreams_database_change::pb::database::DatabaseChanges;
use substreams_database_change::tables::Tables as DatabaseChangeTables;
use substreams_solana::pb::sf::solana::r#type::v1::Block;

#[substreams::handlers::map]
fn map_block(block: Block) -> Result<BlockMeta, substreams::errors::Error> {
    let mut block_height: Option<u64> = None;
    if let Some(v) = block.block_height.as_ref() {
        block_height = Some(v.block_height)
    }

    Ok(BlockMeta {
        hash: block.blockhash.to_string(),
        parent_hash: block.previous_blockhash.to_string(),
        slot: block.slot,
        parent_slot: block.parent_slot,
        transaction_count: block.transactions.len() as u64,
        block_height,
    })
}

#[substreams::handlers::map]
fn map_trades(dex_trades: Output) -> Result<SandwichOutput, substreams::errors::Error> {
    let sandwiches = map_sandwiches(dex_trades.data);
    Ok(SandwichOutput { data: sandwiches })
}
#[substreams::handlers::map]
fn db_out(bm: BlockMeta) -> Result<DatabaseChanges, substreams::errors::Error> {
    // Initialize changes container
    let mut tables = DatabaseChangeTables::new();

    tables
        .create_row("block_meta", bm.hash)
        .set("id", bm.parent_hash)
        .set("number", bm.slot)
        .set("timestamp", bm.transaction_count.to_string());
    Ok(tables.to_database_changes())
}
