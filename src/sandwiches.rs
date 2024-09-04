use crate::pb::sf::solana::dex::sandwiches::v1::{NormalizedSwap, Sandwich};
use crate::primitives::PossibleSandwich;
use itertools::Itertools;
use std::collections::hash_map::Entry;
use std::collections::HashMap;

pub type SwapInfoStore = HashMap<String, NormalizedSwap>;
pub fn map_sandwiches(swaps: Vec<NormalizedSwap>) -> Vec<Sandwich> {
    let swap_info = swaps
        .clone()
        .into_iter()
        .map(|swap| (swap.tx_id.clone(), swap))
        .collect::<SwapInfoStore>();

    //get possible sandwiches
    let sandwiches = get_possible_sandwiches(swaps);
    //substreams::log::println(format!("Possible sandwiches: {:?}", sandwiches));
    //map possible sandwiches to sandwich data
    sandwiches
        .into_iter()
        .filter_map(|ps| calculate_sandwich(ps, &swap_info, 0))
        .flatten()
        .collect()
}

fn calculate_sandwich(
    ps: PossibleSandwich,
    swap_info: &SwapInfoStore,
    _recursive: u8,
) -> Option<Vec<Sandwich>> {
    let PossibleSandwich {
        possible_frontruns,
        possible_backrun,
        victims,
        ..
    } = ps;
    if victims.iter().flatten().count() == 0 {
        return None;
    }

    if !has_pool_overlap(&possible_frontruns, &possible_backrun, &victims, swap_info) {
        return None;
    }
    let frontrun = swap_info.get(&possible_frontruns[0]).cloned()?;
    let backrun = swap_info.get(&possible_backrun).cloned()?;
    let victim_swaps = victims[0]
        .iter()
        .filter_map(|v| swap_info.get(v).cloned())
        .collect();

    //check if profitable
    if backrun.amount_out > frontrun.amount_in {
        return None;
    }

    //this possible sandwich is a sandwich
    Some(vec![Sandwich {
        frontrun: vec![frontrun.clone()],
        backrun: vec![backrun.clone()],
        victim_swaps,
    }])
}

//TODO: Naive implementation must check for multi bun scenarios
// eg: [front_run_token1] [front_run_token2] | [token1_swap] [token2_swap] | [backrun_token1][backrun_token2]
fn has_pool_overlap(
    frontrun_txs: &[String],
    backrun_tx: &String,
    victims_txs: &[Vec<String>],
    swap_info: &SwapInfoStore,
) -> bool {
    let frontrun_swaps = frontrun_txs
        .iter()
        .filter_map(|fr| swap_info.get(fr))
        .collect::<Vec<_>>();

    if frontrun_swaps.is_empty() {
        return false;
    }
    if let Some(backrun_swap) = swap_info.get(backrun_tx) {
        let frontrun_swap = frontrun_swaps[0];
        if !has_reverse_swap_direction(frontrun_swap, backrun_swap) {
            return false;
        }
        let victim_group = victims_txs[0]
            .iter()
            .filter_map(|v| swap_info.get(v).cloned())
            .collect::<Vec<_>>();
        if !verify_sandwich_victims(&victim_group, frontrun_swap, backrun_swap) {
            return false;
        }
        return true;
    }
    false
}
fn has_reverse_swap_direction(swap1: &NormalizedSwap, swap2: &NormalizedSwap) -> bool {
    let same_pool = swap1.pool_address.eq(&swap2.pool_address);
    let is_reverse_direction = swap1.token_in == swap2.token_out;
    if same_pool && is_reverse_direction {
        return true;
    }
    false
}
fn has_same_swap_direction(swap1: &NormalizedSwap, swap2: &NormalizedSwap) -> bool {
    let same_pool = swap1.pool_address.eq(&swap2.pool_address);
    let same_direction = swap1.token_in == swap2.token_in;
    if same_pool && same_direction {
        return true;
    }
    false
}
fn verify_sandwich_victims(
    victims: &[NormalizedSwap],
    frontrun: &NormalizedSwap,
    _backrun: &NormalizedSwap,
) -> bool {
    let total = victims.len();
    let confirmed_victims = victims
        .iter()
        .filter(|v| has_same_swap_direction(frontrun, v))
        .count();
    //check if more than 20 percent are victims
    if (confirmed_victims as f64) / (total as f64) > 0.2 {
        return true;
    }
    false
}
pub fn get_possible_sandwiches(trades: Vec<NormalizedSwap>) -> Vec<PossibleSandwich> {
    let duplicated = get_possible_sandwich_duplicate_senders(trades.clone());
    Itertools::unique(duplicated.into_iter())
        .flat_map(partition_into_gaps)
        .collect::<Vec<_>>()
}

fn partition_into_gaps(ps: PossibleSandwich) -> Vec<PossibleSandwich> {
    let PossibleSandwich {
        eoa,
        possible_frontruns,
        possible_backrun,
        victims,
    } = ps;
    let mut results = vec![];
    let mut last_break = 0;
    let mut victim_sets = vec![];

    victims.into_iter().enumerate().for_each(|(i, group)| {
        // partition here
        if group.is_empty() {
            results.push(PossibleSandwich {
                eoa: eoa.clone(),
                possible_frontruns: possible_frontruns[last_break..i].to_vec(),
                possible_backrun: possible_frontruns[i].clone(),
                victims: std::mem::take(&mut victim_sets),
            });
            last_break = i
        } else {
            victim_sets.push(group)
        }
    });

    if results.is_empty() {
        results.push(PossibleSandwich {
            eoa,
            victims: victim_sets,
            possible_frontruns,
            possible_backrun,
        });
    } else if !victim_sets.is_empty() {
        // add remainder
        results.push(PossibleSandwich {
            eoa,
            victims: victim_sets,
            possible_frontruns: possible_frontruns[last_break..].to_vec(),
            possible_backrun,
        });
    }
    results
}
fn get_possible_sandwich_duplicate_senders(trades: Vec<NormalizedSwap>) -> Vec<PossibleSandwich> {
    // map of address -> transaction_id
    let mut duplicate_senders: HashMap<String, String> = HashMap::default();
    // map of tx_id -> vec<transaction_id>
    let mut possible_victims: HashMap<String, Vec<String>> = HashMap::default();
    // map of address -> sandwiches
    let mut possible_sandwiches: HashMap<String, PossibleSandwich> = HashMap::default();

    for trade in trades {
        let curr_tx = trade.tx_id.clone();
        match duplicate_senders.entry(trade.signer.clone()) {
            Entry::Vacant(e) => {
                e.insert(trade.tx_id.clone());
                // add as first possible frontrun, no victims for this
                possible_victims.insert(curr_tx.clone(), vec![]);
            }
            Entry::Occupied(mut e) => {
                //duplicated entry,
                let prev_tx_hash = e.insert(trade.tx_id.clone());
                // get possible victims of prev transctions (all txs than occur between current and prev_tx_hash)
                if let Some(front_run_victims) = possible_victims.remove(&prev_tx_hash) {
                    match possible_sandwiches.entry(prev_tx_hash.clone()) {
                        Entry::Vacant(s) => {
                            //first time we are facing a duplicate, create brand new sandwich
                            s.insert(PossibleSandwich {
                                eoa: trade.signer,
                                possible_frontruns: vec![prev_tx_hash],
                                possible_backrun: trade.tx_id.clone(),
                                victims: vec![front_run_victims],
                            });
                        }
                        Entry::Occupied(mut s) => {
                            let sandwich = s.get_mut();
                            sandwich.possible_frontruns.push(prev_tx_hash);
                            sandwich.possible_backrun = trade.tx_id.clone();
                            sandwich.victims.push(front_run_victims);
                        }
                    }
                }
                // Add current transaction hash to the list of transactions for this sender
                e.insert(trade.tx_id.clone());
                possible_victims.insert(trade.tx_id.clone(), vec![]);
            }
        }

        //assume this current transaction a victim of all prev transactions
        for (k, v) in possible_victims.iter_mut() {
            if k != &curr_tx {
                v.push(trade.tx_id.clone());
            }
        }
    }

    possible_sandwiches.into_values().collect()
}
