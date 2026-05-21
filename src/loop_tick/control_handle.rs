use std::f32::consts::PI;

use crate::{app_state::{AppState, SPEED}, engine::engine::Engine};

pub fn control_handle(eng: &mut Engine, state: &mut AppState) {
    if !state.intram {
        if eng.control.get_key_state(40) {
            state.scn.objects[state.pu].physic_object.acceleration.x += -SPEED * eng.times_to_calculate_physics as f32;
            state.pivotr = PI / 2.0;
            state.sfx[0].play = true;
            state.controlt = 0;
        } else if eng.control.get_key_state(44) {
            state.scn.objects[state.pu].physic_object.acceleration.x += SPEED * eng.times_to_calculate_physics as f32;
            state.pivotr = (PI / 2.0) * 3.0;
            state.sfx[0].play = true;
            state.controlt = 0;
        } else if eng.control.get_key_state(25) {
            state.scn.objects[state.pu].physic_object.acceleration.z += SPEED * eng.times_to_calculate_physics as f32;
            state.pivotr = PI;
            state.sfx[0].play = true;
            state.controlt = 0;
        } else if eng.control.get_key_state(22) {
            state.scn.objects[state.pu].physic_object.acceleration.z += -SPEED * eng.times_to_calculate_physics as f32;
            state.pivotr = 0.0;
            state.sfx[0].play = true;
            state.controlt = 0;
        }else if eng.control.get_key_state(13) && state.cme && state.tm <= 0 && !state.intram {
            state.selp = 0;
            state.tm = 50;
            state.controlt = 0;
        }else if eng.control.get_key_state(14) && state.cme && state.tm <= 0 && !state.intram {
            state.selp = 1;
            state.tm = 50;
            state.controlt = 0;
        }else if eng.control.get_key_state(15) && state.cme && state.tm <= 0 && !state.intram {
            state.selp = 2;
            state.tm = 50;
            state.controlt = 0;
        }

        // Virtual joystick: left half-screen press engages controlt = 1
        if eng.control.mousebtn[2] {
            let resx_half = (eng.render.resolution_x as f64) / 2.0;
            if eng.control.xpos < resx_half {
                if state.controlt != 1 {
                    state.controlt = 1;
                    state.joy_origin.x = eng.control.xpos as f32;
                    state.joy_origin.y = eng.control.ypos as f32;
                }
                // while held, compute angle from origin to current mouse and set pivot target
                let dx = eng.control.xpos as f32 - state.joy_origin.x;
                let dy = eng.control.ypos as f32 - state.joy_origin.y;
                if dx != 0.0 || dy != 0.0 {
                    let ang = dx.atan2(dy)+PI/4.0;
                    state.pivotr = ang;
                    let a = SPEED * eng.times_to_calculate_physics as f32;
                    state.scn.objects[state.pu].physic_object.acceleration.x += -a * state.pivotr.sin();
                    state.scn.objects[state.pu].physic_object.acceleration.z += -a * state.pivotr.cos();
                    state.sfx[0].play = true;
                }
            }
        } else if state.controlt == 1 && !eng.control.mousebtn[2] {
            // release
            state.controlt = 0;
        }
    }

    let step = SPEED * eng.times_to_calculate_physics as f32 * 20.0;
    let error_margin = SPEED * 5.0;
    let mut delta = (state.pivotr - state.scn.objects[state.pu].physic_object.rot.y + PI) % (2.0 * PI) - PI;
    if delta < -PI {
        delta += 2.0 * PI;
    }
    if delta.abs() <= error_margin {
        state.scn.objects[state.pu].physic_object.rot.y = state.pivotr;
    } else {
        let direction = delta.signum();
        let movement = direction * step;
        if step > delta.abs() {
            state.scn.objects[state.pu].physic_object.rot.y = state.pivotr;
        } else {
            state.scn.objects[state.pu].physic_object.rot.y += movement;
        }
    }
}
