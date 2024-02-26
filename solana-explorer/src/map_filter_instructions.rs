use std::{
    collections::{HashMap, HashSet},
    iter::{self, once},
};

use crate::pb::sol::transactions::v1::{Instruction, Instructions};
use anyhow::anyhow;
use serde::Deserialize;
use substreams_solana::pb::sf::solana::r#type::v1::{Block, ConfirmedTransaction};

#[derive(Deserialize, Debug)]
struct InstructionFilterParams {
    accounts: Vec<String>,
}

#[substreams::handlers::map]
fn map_filter_instructions(params: String, blk: Block) -> Result<Instructions, substreams::errors::Error> {
    let filters = parse_filters_from_params(params)?;

    let instructions: Vec<Instruction> = blk
        .transactions()
        .flat_map(|tx| {
            let ConfirmedTransaction { transaction, meta } = tx;
            let inner_instructions = &meta.as_ref().unwrap().inner_instructions;
            let msg = transaction.as_ref().unwrap().message.as_ref().unwrap();
            let inner_instructions_by_index = inner_instructions
                .into_iter()
                .map(|i| (i.index, &i.instructions))
                .collect::<HashMap<_, _>>();
            let resolved_accounts = &tx
                .resolved_accounts()
                .into_iter()
                .map(|v| v.clone())
                .collect::<Vec<_>>();

            msg.instructions
                .iter()
                .enumerate()
                .filter(|(_, inst)| {
                    apply_filter(
                        &inst.accounts,
                        inst.program_id_index as u8,
                        &filters,
                        resolved_accounts.as_ref(),
                    )
                })
                .flat_map(|(index, inst)| {
                    once(Instruction {
                        program_id: bs58::encode(resolved_accounts[inst.program_id_index as usize].to_vec())
                            .into_string(),
                        accounts: inst
                            .accounts
                            .iter()
                            .map(|acct| bs58::encode(resolved_accounts[*acct as usize].to_vec()).into_string())
                            .collect(),
                        data: bs58::encode(&inst.data).into_string(),
                    })
                    .chain(
                        inner_instructions_by_index
                            .get(&(index as u32))
                            .unwrap_or(&&Vec::new())
                            .iter()
                            .filter(|ix| {
                                apply_filter(
                                    &ix.accounts,
                                    ix.program_id_index as u8,
                                    &filters,
                                    resolved_accounts.as_ref(),
                                )
                            })
                            .map(|ix| Instruction {
                                program_id: bs58::encode(resolved_accounts[ix.program_id_index as usize].to_vec())
                                    .into_string(),
                                accounts: ix
                                    .accounts
                                    .iter()
                                    .map(|acct| bs58::encode(resolved_accounts[*acct as usize].to_vec()).into_string())
                                    .collect(),
                                data: bs58::encode(&ix.data).into_string(),
                            }),
                    )
                    .collect::<Vec<_>>()
                })
                .collect::<Vec<_>>()
        })
        .collect::<Vec<_>>();

    Ok(Instructions { instructions })
}

fn parse_filters_from_params(params: String) -> Result<InstructionFilterParams, substreams::errors::Error> {
    match serde_qs::from_str(&params) {
        Ok(filters) => Ok(filters),
        Err(e) => Err(anyhow!("Failed to parse filters from params: {}", e)),
    }
}

fn apply_filter(
    accounts: &Vec<u8>,
    program_id_idx: u8,
    filters: &InstructionFilterParams,
    account_keys: &Vec<Vec<u8>>,
) -> bool {
    let acct_keys: HashSet<String> = accounts
        .into_iter()
        .chain(iter::once(&program_id_idx))
        .map(|c| bs58::encode(account_keys[*c as usize].clone()).into_string())
        .collect();
    if filters.accounts.is_empty() {
        return true;
    }

    return filters.accounts.iter().any(|a| acct_keys.contains(a));
}
