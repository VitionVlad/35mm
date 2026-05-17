use std::f32::consts::PI;

use crate::{
    app_state::*,
    engine::engine::Engine,
};

fn reset_final_door_game(state: &mut AppState) {
    state.cstop = 0;
    state.tm = 0;
    state.intram = false;
    state.sc3state = 0;
    state.switched_1_4 = false;
    state.switched_5_6 = false;
    state.switch_states = [false; 6];
    state.selp = 0;
    state.cme = false;
    state.bwfilm = 0;
    state.clfilm = 0;
    state.locls = 1;

    for button in state.btns.iter_mut() {
        button.pressed = false;
    }

    let player_pos = &mut state.scn.objects[state.pu].physic_object.pos;
    player_pos.x = state.initial_pivot_pos.x;
    player_pos.y = state.initial_pivot_pos.y;
    player_pos.z = state.initial_pivot_pos.z;

    if state.dbg {
        println!("final door activated: resetting game state and player position");
    }
}

fn handle_final_door_interaction(eng: &mut Engine, state: &mut AppState) {
    let player_pos = state.scn.objects[state.pu].physic_object.pos;
    let door_pos = state.scn.objects[state.finaldooridx].physic_object.pos;
    let dist = distance(player_pos, door_pos);

    if dist <= 1.0 && state.selp == 0 {
        let can_open = state.gkey == usize::MAX && state.sc3state == 2;
        let icon = if can_open { 
            &mut state.drbtn 
        } else if state.gkey == usize::MAX && state.sc3state != 2 { 
            &mut state.nebtn 
        } else { 
            &mut state.nkbtn 
        };
        icon.object.physic_object.scale.x = 80.0;
        icon.object.physic_object.scale.y = 80.0;
        icon.object.physic_object.pos.x = eng.render.resolution_x as f32 / 2.0 - icon.object.physic_object.scale.x / 2.0;
        icon.object.physic_object.pos.y = eng.render.resolution_y as f32 - icon.object.physic_object.scale.y * 2.0 - 20.0;
        icon.object.draw = true;
        let icon_pressed = icon.exec(eng) && can_open && eng.control.mousebtn[2];

        if can_open && ((eng.control.get_key_state(26) || icon_pressed) && state.tm <= 0) {
            state.sfx[3].move_sound_cursor(0.0);
            state.sfx[3].play = true;
            reset_final_door_game(state);
            state.drbtn.object.draw = false;
            state.drbtn.exec(eng);
        }
    } else {
        state.drbtn.object.draw = false;
        state.nkbtn.object.draw = false;
        state.nebtn.object.draw = false;
        state.nkbtn.exec(eng);
        state.drbtn.exec(eng);
        state.nebtn.exec(eng);
    }
}

fn process_button_interactions(eng: &mut Engine, state: &mut AppState) {
    let mut button_status: Vec<(u32, u32, bool)> = state.btns.iter().map(|b| (b.in_scene_index, b.scene_index, b.pressed)).collect();
    for i in 0..state.btns.len() {
        let button = &mut state.btns[i];
        let player_pos = state.scn.objects[state.pu].physic_object.pos;
        let button_pos = state.scn.objects[button.index].physic_object.pos;
        let dist = distance(player_pos, button_pos);
        if button.axis < 4 {
            let can_use = dist <= 1.0 && (button.scene_index != 3 || state.ekey == usize::MAX);
            let show_nk = dist <= 1.0 && button.scene_index == 3 && state.ekey != usize::MAX;
            if (can_use || show_nk) && state.selp == 0 {
                let icon = if show_nk { &mut state.nkbtn } else { &mut state.btnbtn };
                icon.object.physic_object.scale.x = 80.0;
                icon.object.physic_object.scale.y = 80.0;
                icon.object.physic_object.pos.x = eng.render.resolution_x as f32 / 2.0 - icon.object.physic_object.scale.x / 2.0;
                icon.object.physic_object.pos.y = eng.render.resolution_y as f32 - icon.object.physic_object.scale.y * 2.0 - 20.0;
                icon.object.draw = true;
                let icon_pressed = icon.exec(eng) && !show_nk && eng.control.mousebtn[2];
                if can_use && ((eng.control.get_key_state(26) || icon_pressed) && state.tm <= 0) {
                        let rot_axis = button.axis;
                        match rot_axis {
                            0 => state.scn.objects[button.index].physic_object.rot.x += PI,
                            1 => state.scn.objects[button.index].physic_object.rot.y += PI,
                            2 => state.scn.objects[button.index].physic_object.rot.z += PI,
                            _ => {}
                        }
                        state.sfx[3].move_sound_cursor(0.0);
                        state.sfx[3].play = true;
                        
                        match button.scene_index {
                            2 => {
                                let play_powered = match button.in_scene_index {
                                    5 | 4 | 3 => true,
                                    1 => button_status.iter().any(|&(idx, scene_idx, pressed)| idx == 3 && scene_idx == 2 && pressed),
                                    2 => button_status.iter().any(|&(idx, scene_idx, pressed)| idx == 4 && scene_idx == 2 && pressed),
                                    6 => button_status.iter().any(|&(idx, scene_idx, pressed)| idx == 5 && scene_idx == 2 && pressed),
                                    _ => false,
                                };
                                if play_powered && !button.pressed{
                                    state.sfx[4].move_sound_cursor(0.0);
                                    state.sfx[4].play = true;
                                }
                                button.pressed = !button.pressed;
                                button_status[i] = (button.in_scene_index, button.scene_index, button.pressed);
                            
                                if button_status.iter().any(|&(idx, scene_idx, pressed)| idx == 1 && scene_idx == 2 && pressed) &&
                                button_status.iter().any(|&(idx, scene_idx, pressed)| idx == 2 && scene_idx == 2 && pressed) &&
                                button_status.iter().any(|&(idx, scene_idx, pressed)| idx == 3 && scene_idx == 2 && pressed)&& 
                                button_status.iter().any(|&(idx, scene_idx, pressed)| idx == 4 && scene_idx == 2 && pressed){
                                    state.switched_1_4 = true;
                                    if state.dbg {
                                        println!("switched 1, 2, 3, 4");
                                    }
                                }else {
                                    state.switched_1_4 = false;
                                }
                                if button_status.iter().any(|&(idx, scene_idx, pressed)| idx == 5 && scene_idx == 2 && pressed) &&
                                button_status.iter().any(|&(idx, scene_idx, pressed)| idx == 6 && scene_idx == 2 && pressed){
                                    state.switched_5_6 = true;
                                    if state.dbg {
                                        println!("switched 5 and 6");
                                    }
                                }else{
                                    state.switched_5_6 = false;
                                }
                            },
                            3 =>{
                                button.pressed = !button.pressed;
                                button_status[i] = (button.in_scene_index, button.scene_index, button.pressed);

                                if state.dbg {
                                    println!("scene 3 button, idx = {}, in_scene_index = {}, pressed = {}", button.index, button.in_scene_index, button.pressed);
                                }
                                if button_status.iter().any(|&(idx, scene_idx, pressed)| idx == 4 && scene_idx == 3 && pressed) &&
                                button_status.iter().any(|&(idx, scene_idx, pressed)| idx == 2 && scene_idx == 3 && pressed) &&
                                button_status.iter().any(|&(idx, scene_idx, pressed)| idx == 3 && scene_idx == 3 && pressed){
                                    state.sc3state = 1;
                                    if button_status.iter().any(|&(idx, scene_idx, pressed)| idx == 1 && scene_idx == 3 && pressed){
                                        state.sc3state = 2;
                                    }
                                    if state.dbg {
                                        println!("scene 3 state = {}", state.sc3state);
                                    }
                                }else {
                                    state.sc3state = 0;
                                }
                            },
                            _ => (),
                        }
                        state.tm = 50;
                    }
                break;
            } else {
                state.btnbtn.object.draw = false;
                state.nkbtn.object.draw = false;
                state.btnbtn.exec(eng);
                state.nkbtn.exec(eng);
            }
        } else {
            let same_index_pressed = button_status.iter().any(|&(idx, scene_idx, pressed)| idx == button.in_scene_index && scene_idx == button.scene_index && pressed);
            let rot_axis = if button.axis >= 4 { button.axis - 3 } else { button.axis };
            if same_index_pressed {
                match rot_axis {
                    0 => state.scn.objects[button.index].physic_object.rot.x = PI,
                    1 => state.scn.objects[button.index].physic_object.rot.y = PI,
                    2 => state.scn.objects[button.index].physic_object.rot.z = PI,
                    _ => {}
                }
            } else {
                match rot_axis {
                    0 => state.scn.objects[button.index].physic_object.rot.x = 0.0,
                    1 => state.scn.objects[button.index].physic_object.rot.y = 0.0,
                    2 => state.scn.objects[button.index].physic_object.rot.z = 0.0,
                    _ => {}
                }
            }
        }
    }
}

pub fn handle_scene(eng: &mut Engine, state: &mut AppState) {
    //state.btnbtn.object.draw = false;
    //state.nkbtn.object.draw = false;

    match state.cstop {
        1 => {
            if state.skp2 {
                state.switched_5_6 = true;
                state.switched_1_4 = true; 
            }
            if state.switched_1_4{
                match state.doors[0].axis {
                    0 => state.scn.objects[state.doors[0].index].physic_object.pos.x = state.doors[0].initial_pos.x - state.doors[0].movement,
                    1 => state.scn.objects[state.doors[0].index].physic_object.pos.y = state.doors[0].initial_pos.y - state.doors[0].movement,
                    2 => state.scn.objects[state.doors[0].index].physic_object.pos.z = state.doors[0].initial_pos.z - state.doors[0].movement,
                    _ => {}
                }
            }else{
                match state.doors[0].axis {
                    0 => state.scn.objects[state.doors[0].index].physic_object.pos.x = state.doors[0].initial_pos.x,
                    1 => state.scn.objects[state.doors[0].index].physic_object.pos.y = state.doors[0].initial_pos.y,
                    2 => state.scn.objects[state.doors[0].index].physic_object.pos.z = state.doors[0].initial_pos.z,
                    _ => {}
                }
            }
        }
        2 => {
            if state.dbg {
                state.ekey = usize::MAX;
                state.gkey = usize::MAX;
            }
            if state.sc3state != 0{
                match state.doors[1].axis {
                    0 => state.scn.objects[state.doors[1].index].physic_object.pos.x = state.doors[1].initial_pos.x - state.doors[1].movement,
                    1 => state.scn.objects[state.doors[1].index].physic_object.pos.y = state.doors[1].initial_pos.y - state.doors[1].movement,
                    2 => state.scn.objects[state.doors[1].index].physic_object.pos.z = state.doors[1].initial_pos.z - state.doors[1].movement,
                    _ => {}
                }
            }else{
                match state.doors[1].axis {
                    0 => state.scn.objects[state.doors[1].index].physic_object.pos.x = state.doors[1].initial_pos.x,
                    1 => state.scn.objects[state.doors[1].index].physic_object.pos.y = state.doors[1].initial_pos.y,
                    2 => state.scn.objects[state.doors[1].index].physic_object.pos.z = state.doors[1].initial_pos.z,
                    _ => {}
                }
            }

            handle_final_door_interaction(eng, state);
        }
        _ => {
        }
    }

    process_button_interactions(eng, state);
}