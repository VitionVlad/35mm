use std::f32::consts::PI;

use crate::{
    app_state::*,
    engine::engine::Engine,
};

pub fn handle_scene(eng: &mut Engine, state: &mut AppState) {
    if state.selp == 0 {
        let mut button_status: Vec<(u32, u32, bool)> = state.btns.iter().map(|b| (b.in_scene_index, b.scene_index, b.pressed)).collect();
        for i in 0..state.btns.len() {
            let button = &mut state.btns[i];
            let player_pos = state.scn.objects[state.pu].physic_object.pos;
            let button_pos = state.scn.objects[button.index].physic_object.pos;
            let dist = distance(player_pos, button_pos);
            if button.axis < 4 {
                if dist <= 1.0 {
                    // show blue panel with "E"
                    let tx = "E";
                    state.phcnt.draw = true;
                    state.phcnt.size.x = 15.0;
                    state.phcnt.size.y = 30.0;
                    state.phcnt.pos.z = 0.1;
                    state.phcnt.pos.x = eng.render.resolution_x as f32 / 2.0 - (tx.len() as f32 * state.phcnt.size.x) / 2.0;
                    state.phcnt.pos.y = state.bwbtn.object.physic_object.pos.y - state.phcnt.size.y * 2.0;
                    state.phcnt.exec(eng, tx);
                    state.bluepan.object.draw = true;
                    state.bluepan.object.physic_object.scale.y = state.phcnt.size.y;
                    state.bluepan.object.physic_object.scale.x = state.bwbtn.object.physic_object.scale.x * 2.0;
                    state.bluepan.object.physic_object.pos.x = eng.render.resolution_x as f32 / 2.0 - state.bluepan.object.physic_object.scale.x / 2.0;
                    state.bluepan.object.physic_object.pos.y = state.phcnt.pos.y;
                    state.bluepan.object.mesh.ubo[50] = 0.0;
                    state.bluepan.exec(eng);
                    if eng.control.get_key_state(26) && state.tm <= 0 {
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
                            3 =>{},
                            _ => (),
                        }
                        state.tm = 50;
                    }
                } else {
                    state.phcnt.draw = false;
                    state.bluepan.object.draw = false;
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
    match state.cstop {
        1 => {
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
            //if state.switched_1_4{
            //    match state.doors[0].axis {
            //        0 => state.scn.objects[state.doors[0].index].physic_object.pos.x = state.doors[0].initial_pos.x - state.doors[0].movement,
            //        1 => state.scn.objects[state.doors[0].index].physic_object.pos.y = state.doors[0].initial_pos.y - state.doors[0].movement,
            //        2 => state.scn.objects[state.doors[0].index].physic_object.pos.z = state.doors[0].initial_pos.z - state.doors[0].movement,
            //        _ => {}
            //    }
            //}else{
            //    match state.doors[0].axis {
            //        0 => state.scn.objects[state.doors[0].index].physic_object.pos.x = state.doors[0].initial_pos.x,
            //        1 => state.scn.objects[state.doors[0].index].physic_object.pos.y = state.doors[0].initial_pos.y,
            //        2 => state.scn.objects[state.doors[0].index].physic_object.pos.z = state.doors[0].initial_pos.z,
            //        _ => {}
            //    }
            //}
        }
        _ => {
        }
    }
}