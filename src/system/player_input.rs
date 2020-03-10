use crate::data::game_state::GameState;
use crate::data::channel::ChannelPayload;
use crate::data::constants::Direction;
use std::sync::mpsc::TryRecvError;

pub fn player_input_system(gs: &mut GameState, player_input: Result<ChannelPayload, TryRecvError>) {
    match player_input {
        Err(_e) => {} // ignore empty player_input buffer and disconnected channels
        Ok(payload) => {
            match payload {
                // single word commands without a target/payload
                ChannelPayload::Cmd((id, cmd)) => {
                    let eid = gs.get_player_eid(id);
                    match cmd.as_str() {
                        "n" | "s" | "e" | "w" => {
                            gs.add_mover(eid, Direction::from(cmd));
                        },
                        "l" | "look" => {
                            gs.print_location(id);
                        },
                        "help" => {
                            gs.msg_player(id, "\
                            n s e w: Travel in the specified direction\n\
                            l or look: View your current location\n\
                            s or say: Speak to other players in your current location\n\
                            help: View the list of commands\n\
                            quit or exit: Leave the land of Lera\n\
                            \n");
                        },
                        "quit" | "exit" => {
                            println!("Player with id {} left the game", id);
                            gs.remove_player(id);
                        },
                        _ => { gs.msg_player(id, format!("Unkown command: {}\n", cmd).as_str()); }
                    }
                },

                // commands with targets/payloads
                ChannelPayload::Target((id, cmd, target)) => {
                    let eid = gs.get_player_eid(id);
                    match cmd.as_str() {
                        "s" | "say" => {
                            gs.add_speaker(eid, target.as_str());
                        },
                        _ => { gs.msg_player(id, format!("Unkown command: {}\n", cmd).as_str()); }
                    }
                },

                // command sent by server to join a player to the game
                ChannelPayload::Join((id, name, socket)) => {
                    println!("Player {} with id {} joined game", name, id);
                    let existing = gs.players.get(&id);
                    match existing {
                        Some(_p) => { gs.msg_player(id, "You are already joined to the game.\n"); },
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
