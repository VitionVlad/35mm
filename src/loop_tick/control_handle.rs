use std::f32::consts::PI;

use crate::{app_state::{AppState, SPEED}, engine::engine::Engine};

pub fn control_handle(eng: &mut Engine, state: &mut AppState) {
    if !state.intram {
        if eng.control.get_key_state(40) {
            state.scn.objects[state.pu].physic_object.acceleration.x += -SPEED * eng.times_to_calculate_physics as f32;
            state.pivotr = PI / 2.0;
            state.sfx[0].play = true;
        } else if eng.control.get_key_state(44) {
            state.scn.objects[state.pu].physic_object.acceleration.x += SPEED * eng.times_to_calculate_physics as f32;
            state.pivotr = (PI / 2.0) * 3.0;
            state.sfx[0].play = true;
        } else if eng.control.get_key_state(25) {
            state.scn.objects[state.pu].physic_object.acceleration.z += SPEED * eng.times_to_calculate_physics as f32;
            state.pivotr = PI;
            state.sfx[0].play = true;
        } else if eng.control.get_key_state(22) {
            state.scn.objects[state.pu].physic_object.acceleration.z += -SPEED * eng.times_to_calculate_physics as f32;
            state.pivotr = 0.0;
            state.sfx[0].play = true;
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
