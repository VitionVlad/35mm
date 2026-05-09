use crate::{
    app_state::{distance, AppState, SPEED},
    engine::{engine::Engine, light::LightType, math::vec3::Vec3},
};

pub fn tick(eng: &mut Engine, state: &mut AppState) {
    if state.tm > 0 {
        state.tm -= eng.times_to_calculate_physics as i32;
    }

    state.viewport.ubo_index = 51;
    state.viewport.object.mesh.ubo[49] = state.scn.objects[state.pu].physic_object.pos.x;
    state.viewport.object.mesh.ubo[50] = state.scn.objects[state.pu].physic_object.pos.z;
    state.viewport.object.mesh.ubo[51] = state.pkbf;

    if state.pkbf < 1_f32 {
        state.pkbf += SPEED * 5.0 * eng.times_to_calculate_physics as f32;
    }
    if state.pkbf > 1_f32 {
        state.pkbf -= SPEED * 5.0 * eng.times_to_calculate_physics as f32;
    }
    if (1.0 - SPEED * 5.0 * eng.times_to_calculate_physics as f32) < state.pkbf
        && (1.0 + SPEED * 5.0 * eng.times_to_calculate_physics as f32) > state.pkbf
    {
        state.pkbf = 1.0;
    }

    eng.cameras[0].physic_object.pos = Vec3 {
        x: state.scn.objects[state.pu].physic_object.pos.x - 7.5_f32,
        y: 10_f32,
        z: state.scn.objects[state.pu].physic_object.pos.z - 7.5_f32,
    };
    eng.cameras[0].fov = 37.5_f32;
    eng.cameras[0].physic_object.rot.x = 0.7_f32;
    eng.cameras[0].physic_object.rot.y = 2.355_f32;

    eng.lights[0].camera.physic_object.pos = Vec3 {
        x: state.scn.objects[state.pu].physic_object.pos.x - 47.5_f32,
        y: 55_f32,
        z: state.scn.objects[state.pu].physic_object.pos.z - 47.5_f32,
    };
    eng.lights[0].light_type = LightType::Directional;
    eng.lights[0].direction = Vec3 {
        x: 1.0_f32,
        y: -1.0_f32,
        z: 1.0_f32,
    };
    eng.lights[0].pos = eng.lights[0].camera.physic_object.pos;
    eng.lights[0].rot.x = 0.7_f32;
    eng.lights[0].rot.y = 2.355_f32;
    eng.lights[0].camera.fov = 20_f32;

    for i in 0..state.cvec.len() {
        if !state.cvec[i].consumed {
            let p1 = state.scn.objects[state.pu].physic_object.pos;
            let p2 = state.scn.objects[state.cvec[i].index].physic_object.pos;
            let d = distance(p1, p2);
            if d <= 5.0 && d > 0.5 {
                if p2.x > p1.x {
                    state.scn.objects[state.cvec[i].index].physic_object.acceleration.x -=
                        2.0 * SPEED * eng.times_to_calculate_physics as f32;
                } else {
                    state.scn.objects[state.cvec[i].index].physic_object.acceleration.x +=
                        2.0 * SPEED * eng.times_to_calculate_physics as f32;
                }
                if p2.z > p1.z {
                    state.scn.objects[state.cvec[i].index].physic_object.acceleration.z -=
                        2.0 * SPEED * eng.times_to_calculate_physics as f32;
                } else {
                    state.scn.objects[state.cvec[i].index].physic_object.acceleration.z +=
                        2.0 * SPEED * eng.times_to_calculate_physics as f32;
                }
            } else if d <= 0.5 {
                state.cvec[i].consumed = true;
                state.scn.objects[state.cvec[i].index].draw = false;
                state.scn.objects[state.cvec[i].index].draw_shadow = false;
                state.pkbf = 0.0;
                state.sfx[6].move_sound_cursor(0.0);
                state.sfx[6].play = true;
                match state.cvec[i].ctype {
                    0 => state.cme = true,
                    1 => state.bwfilm += 8,
                    2 => state.clfilm += 8,
                    _ => {}
                }
            }
        }
    }

    state.scn.exec(eng);

    state.viewport.object.physic_object.scale.x = eng.render.resolution_x as f32;
    state.viewport.object.physic_object.scale.y = eng.render.resolution_y as f32;
    state.viewport.exec(eng);

    state.fpscnt.pos.x = 0.0;
    state.fpscnt.pos.y = 0.0;
    state.fpscnt.size.x = 15_f32;
    state.fpscnt.size.y = 30_f32;
    let fps = eng.fps;
    state.fpscnt.exec(eng, &format!("fps:{}", fps));

    state.psbtn.object.physic_object.scale.x = 80.0;
    state.psbtn.object.physic_object.scale.y = 80.0;
    state.psbtn.object.physic_object.pos.x = eng.render.resolution_x as f32 / 2.0;
    state.psbtn.object.physic_object.pos.y = eng.render.resolution_y as f32 - state.psbtn.object.physic_object.scale.y;
    if !state.cme || state.intram {
        state.psbtn.object.physic_object.pos.x =
            eng.render.resolution_x as f32 / 2.0 - state.psbtn.object.physic_object.scale.x / 2.0;
    }
    state.psbtn.exec(eng);
}
