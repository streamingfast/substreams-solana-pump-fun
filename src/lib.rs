mod idl;
mod pb;

use anchor_lang::AnchorDeserialize;
use anchor_lang::Discriminator;
use base64::prelude::*;
use pb::substreams::v1::program::Buy;
use pb::substreams::v1::program::CompleteEvent;
use pb::substreams::v1::program::Create;
use pb::substreams::v1::program::CreateEvent;
use pb::substreams::v1::program::Data;
use pb::substreams::v1::program::Sell;
use pb::substreams::v1::program::SetParams;
use pb::substreams::v1::program::SetParamsEvent;
use pb::substreams::v1::program::TradeEvent;

use sha2::Digest;
use sha2::Sha256;
use sologger_log_context::programs_selector::ProgramsSelector;
use sologger_log_context::sologger_log_context::LogContext;
use substreams::Hex;
use substreams_database_change::pb::database::DatabaseChanges;
use substreams_database_change::tables::Tables as DatabaseChangeTables;
use substreams_solana::pb::sf::solana::r#type::v1::Block;

const PROGRAM_ID: &str = "6EF8rrecthR5Dkzon8Nwu78hRvfCKubJ14M5uBEwF6P";

#[substreams::handlers::map]
fn map_program_data(blk: Block) -> Data {
    let mut create_event_list: Vec<CreateEvent> = Vec::new();
    let mut trade_event_list: Vec<TradeEvent> = Vec::new();
    let mut complete_event_list: Vec<CompleteEvent> = Vec::new();
    let mut set_params_event_list: Vec<SetParamsEvent> = Vec::new();
    let mut set_params_list: Vec<SetParams> = Vec::new();
    let mut create_list: Vec<Create> = Vec::new();
    let mut buy_list: Vec<Buy> = Vec::new();
    let mut sell_list: Vec<Sell> = Vec::new();

    blk.transactions().for_each(|transaction| {
        let meta_wrapped = &transaction.meta;
        let meta = meta_wrapped.as_ref().unwrap();

        // ------------- EVENTS -------------
        let programs_selector: ProgramsSelector = ProgramsSelector::new(&["*".to_string()]);
        let log_contexts = LogContext::parse_logs_basic(&meta.log_messages, &programs_selector);

        log_contexts
            .iter()
            .filter(|context| context.program_id == PROGRAM_ID)
            .for_each(|context| {
                context.data_logs.iter().for_each(|data| {
                    if let Ok(decoded) = BASE64_STANDARD.decode(data) {
                        let slice_u8: &mut &[u8] = &mut &decoded[..];
                        let slice_discriminator: [u8; 8] =
                            slice_u8[0..8].try_into().expect("error");

                        match slice_discriminator {
                            idl::idl::program::events::CreateEvent::DISCRIMINATOR => {
                                if let Ok(event) =
                                    idl::idl::program::events::CreateEvent::deserialize(
                                        &mut &slice_u8[8..],
                                    )
                                {
                                    create_event_list.push(CreateEvent {
                                        name: event.name,
                                        symbol: event.symbol,
                                        uri: event.uri,
                                        mint: event.mint.to_string(),
                                        bonding_curve: event.bonding_curve.to_string(),
                                        user: event.user.to_string(),
                                    });
                                }
                            }
                            idl::idl::program::events::TradeEvent::DISCRIMINATOR => {
                                if let Ok(event) =
                                    idl::idl::program::events::TradeEvent::deserialize(
                                        &mut &slice_u8[8..],
                                    )
                                {
                                    trade_event_list.push(TradeEvent {
                                        mint: event.mint.to_string(),
                                        sol_amount: event.sol_amount,
                                        token_amount: event.token_amount,
                                        is_buy: event.is_buy,
                                        user: event.user.to_string(),
                                        timestamp: event.timestamp,
                                        virtual_sol_reserves: event.virtual_sol_reserves,
                                        virtual_token_reserves: event.virtual_token_reserves,
                                        real_sol_reserves: event.real_sol_reserves,
                                        real_token_reserves: event.real_token_reserves,
                                    });
                                }
                            }
                            idl::idl::program::events::CompleteEvent::DISCRIMINATOR => {
                                if let Ok(event) =
                                    idl::idl::program::events::CompleteEvent::deserialize(
                                        &mut &slice_u8[8..],
                                    )
                                {
                                    complete_event_list.push(CompleteEvent {
                                        user: event.user.to_string(),
                                        mint: event.mint.to_string(),
                                        bonding_curve: event.bonding_curve.to_string(),
                                        timestamp: event.timestamp,
                                    });
                                }
                            }
                            idl::idl::program::events::SetParamsEvent::DISCRIMINATOR => {
                                if let Ok(event) =
                                    idl::idl::program::events::SetParamsEvent::deserialize(
                                        &mut &slice_u8[8..],
                                    )
                                {
                                    set_params_event_list.push(SetParamsEvent {
                                        fee_recipient: event.fee_recipient.to_string(),
                                        initial_virtual_token_reserves: event
                                            .initial_virtual_token_reserves,
                                        initial_virtual_sol_reserves: event
                                            .initial_virtual_sol_reserves,
                                        initial_real_token_reserves: event
                                            .initial_real_token_reserves,
                                        token_total_supply: event.token_total_supply,
                                        fee_basis_points: event.fee_basis_points,
                                    });
                                }
                            }
                            _ => {}
                        }
                    }
                });
            }); // ------------- INSTRUCTIONS -------------
        transaction
            .walk_instructions()
            .into_iter()
            .filter(|inst| inst.program_id().to_string() == PROGRAM_ID)
            .for_each(|inst| {
                let slice_u8: &[u8] = &inst.data()[..];
                if slice_u8[0..8] == idl::idl::program::client::args::SetParams::DISCRIMINATOR {
                    if let Ok(instruction) =
                        idl::idl::program::client::args::SetParams::deserialize(&mut &slice_u8[8..])
                    {
                        set_params_list.push(SetParams {
                            fee_recipient: instruction.fee_recipient.to_string(),
                            initial_virtual_token_reserves: instruction
                                .initial_virtual_token_reserves,
                            initial_virtual_sol_reserves: instruction.initial_virtual_sol_reserves,
                            initial_real_token_reserves: instruction.initial_real_token_reserves,
                            token_total_supply: instruction.token_total_supply,
                            fee_basis_points: instruction.fee_basis_points,
                        });
                    }
                }
                if slice_u8[0..8] == idl::idl::program::client::args::Create::DISCRIMINATOR {
                    if let Ok(instruction) =
                        idl::idl::program::client::args::Create::deserialize(&mut &slice_u8[8..])
                    {
                        create_list.push(Create {
                            name: instruction.name,
                            symbol: instruction.symbol,
                            uri: instruction.uri,
                        });
                    }
                }
                if slice_u8[0..8] == idl::idl::program::client::args::Buy::DISCRIMINATOR {
                    if let Ok(instruction) =
                        idl::idl::program::client::args::Buy::deserialize(&mut &slice_u8[8..])
                    {
                        buy_list.push(Buy {
                            amount: instruction.amount,
                            max_sol_cost: instruction.max_sol_cost,
                        });
                    }
                }
                if slice_u8[0..8] == idl::idl::program::client::args::Sell::DISCRIMINATOR {
                    if let Ok(instruction) =
                        idl::idl::program::client::args::Sell::deserialize(&mut &slice_u8[8..])
                    {
                        sell_list.push(Sell {
                            amount: instruction.amount,
                            min_sol_output: instruction.min_sol_output,
                        });
                    }
                }
            });
    });

    Data {
        create_event_list,
        trade_event_list,
        complete_event_list,
        set_params_event_list,
        set_params_list,
        create_list,
        buy_list,
        sell_list,
    }
}

#[substreams::handlers::map]
fn db_out(data: Data) -> DatabaseChanges {
    let mut tables: DatabaseChangeTables = DatabaseChangeTables::new();

    for event in data.create_event_list {
        let mut hasher = Sha256::new();
        hasher.update(&event.name);
        hasher.update(&event.symbol);
        hasher.update(&event.uri);
        hasher.update(&event.mint);
        hasher.update(&event.bonding_curve);
        hasher.update(&event.user);
        let pk = Hex::encode(hasher.finalize());

        tables
            .create_row("createevent", pk)
            .set("name", event.name)
            .set("symbol", event.symbol)
            .set("uri", event.uri)
            .set("mint", event.mint)
            .set("bonding_curve", event.bonding_curve)
            .set("user", event.user);
    }

    for event in data.trade_event_list {
        let mut hasher = Sha256::new();
        hasher.update(&event.mint);
        hasher.update(event.sol_amount.to_le_bytes());
        hasher.update(event.token_amount.to_le_bytes());
        hasher.update(&[event.is_buy as u8]);
        hasher.update(&event.user);
        hasher.update(event.timestamp.to_le_bytes());
        hasher.update(event.virtual_sol_reserves.to_le_bytes());
        hasher.update(event.virtual_token_reserves.to_le_bytes());
        hasher.update(event.real_sol_reserves.to_le_bytes());
        hasher.update(event.real_token_reserves.to_le_bytes());
        let pk = Hex::encode(hasher.finalize());

        tables
            .create_row("tradeevent", pk)
            .set("mint", event.mint)
            .set("sol_amount", event.sol_amount.to_string())
            .set("token_amount", event.token_amount.to_string())
            .set("is_buy", event.is_buy.to_string())
            .set("user", event.user)
            .set("timestamp", event.timestamp.to_string())
            .set(
                "virtual_sol_reserves",
                event.virtual_sol_reserves.to_string(),
            )
            .set(
                "virtual_token_reserves",
                event.virtual_token_reserves.to_string(),
            )
            .set("real_sol_reserves", event.real_sol_reserves.to_string())
            .set("real_token_reserves", event.real_token_reserves.to_string());
    }

    return tables.to_database_changes();
}
