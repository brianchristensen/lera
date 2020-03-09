use crate::data::game_state::GameState;
use crate::data::channel::ChannelPayload;
use std::sync::mpsc::TryRecvError;

pub fn player_input_system(gs: &mut GameState, player_input: Result<ChannelPayload, TryRecvError>) {
    match player_input {
        Err(_e) => {}, // ignore disconnected socket or empty message buffer
        Ok(payload) => {
            match payload {
                // single word commands without a target/payload
                ChannelPayload::Cmd((id, cmd)) => {
                    match cmd.as_str() {
                        "n" | "s" | "e" | "w" => {
                            //gs.move_player(id, cmd);
                        },
                        "l" | "look" => {
                            gs.print_location(id);
                        },
                        "help" => {
                            gs.dm_player(id, "\
                            n s e w: Travel in the specified direction\n\
                            l or look: View your current location\n\
                            s or say: Speak to other players in your current location\n\
                            help: View the list of commands\n\n\
                            ");
                        },
                        _ => { gs.dm_player(id, format!("Unkown command: {}\n", cmd).as_str()); }
                    }
                },

                // commands with targets/payloads
                ChannelPayload::Target((id, cmd, target)) => {
                    match cmd.as_str() {
                        "s" | "say" => {
                            gs.dm_location(id, target.as_str());
                        },
                        _ => { gs.dm_player(id, format!("Unkown command: {}\n", cmd).as_str()); }
                    }
                },

                // command sent by server to join a player to the game
                ChannelPayload::Join((id, name, socket)) => {
                    println!("Player {} with id {} joined game", name, id);
                    let existing = gs.players.get(&id);
                    match existing {
                        Some(_p) => { gs.dm_player(id, "You are already joined to the game.\n"); },
                        None => {
                            // join game
                            gs.join_player(id, name, socket);
                            // print starting location
                            gs.print_location(id);
                        }
                    }
                }
            }
        }
    }
}
