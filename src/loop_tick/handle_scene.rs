use std::f32::consts::PI;

use crate::{
    app_state::*,
    engine::engine::Engine,
};

pub fn handle_scene(eng: &mut Engine, state: &mut AppState) {
    let button_status: Vec<(u32, bool)> = state.btns.iter().map(|b| (b.in_scene_index, b.pressed)).collect();
    for button in &mut state.btns {
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
                    button.pressed = true;
                    let rot_axis = button.axis;
                    match rot_axis {
                        0 => state.scn.objects[button.index].physic_object.rot.x += PI,
                        1 => state.scn.objects[button.index].physic_object.rot.y += PI,
                        2 => state.scn.objects[button.index].physic_object.rot.z += PI,
                        _ => {}
                    }
                    state.tm = 50;
                }
            } else {
                // hide if not close
                state.phcnt.draw = false;
                state.bluepan.object.draw = false;
            }
        } else {
            // for axis >=4, rotate indicator if same index pressed
            let same_index_pressed = button_status.iter().any(|&(idx, pressed)| idx == button.in_scene_index && pressed);
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
    
    match state.cstop {
        1 => {
            // currently nothing
        }
        2 => {
            // currently nothing
        }
        _ => {
            // default case, currently nothing
        }
    }
}