use std::f32::consts::PI;

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

    if !state.intram{
        if eng.control.get_key_state(40) {
            state.scn.objects[state.pu].physic_object.acceleration.x +=
                -SPEED * eng.times_to_calculate_physics as f32;
            state.pivotr = PI / 2.0;
        } else if eng.control.get_key_state(44) {
            state.scn.objects[state.pu].physic_object.acceleration.x +=
                SPEED * eng.times_to_calculate_physics as f32;
            state.pivotr = (PI / 2.0) * 3.0;
        } else if eng.control.get_key_state(25) {
            state.scn.objects[state.pu].physic_object.acceleration.z +=
                SPEED * eng.times_to_calculate_physics as f32;
            state.pivotr = PI;
        } else if eng.control.get_key_state(22) {
            state.scn.objects[state.pu].physic_object.acceleration.z +=
                -SPEED * eng.times_to_calculate_physics as f32;
            state.pivotr = 0.0;
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

    match state.selp {
        0 => {
            eng.used_light_count = state.locls;
            eng.lights[0].color = Vec3 {
                x: 0.8,
                y: 0.9,
                z: 1.0,
            };

            state.bwbtn.object.draw = false;
            state.bwbtn.exec(eng);

            state.colbtn.object.draw = false;
            state.colbtn.exec(eng);

            if distance(
                state.scn.objects[state.pu].physic_object.pos,
                state.scn.objects[state.tramin].physic_object.pos,
            ) < 7.5{
                let tx = &format!("E");
                state.phcnt.draw = true;
                state.phcnt.size.x = 15_f32;
                state.phcnt.size.y = 30_f32;
                state.phcnt.pos.z = 0.1;
                state.phcnt.pos.x =
                    eng.render.resolution_x as f32 / 2.0 - ((tx.len() as f32 * state.phcnt.size.x) / 2.0);
                state.phcnt.pos.y = state.bwbtn.object.physic_object.pos.y - state.phcnt.size.y * 2.0;
                state.phcnt.exec(eng, tx);

                state.bluepan.object.draw = true;
                state.bluepan.object.physic_object.scale.y = state.phcnt.size.y;
                state.bluepan.object.physic_object.scale.x = state.bwbtn.object.physic_object.scale.x * 2.0;
                state.bluepan.object.physic_object.pos.x =
                    eng.render.resolution_x as f32 / 2.0 - state.bluepan.object.physic_object.scale.x / 2.0;
                state.bluepan.object.physic_object.pos.y = state.phcnt.pos.y;
                state.bluepan.object.mesh.ubo[50] = 0.0;
                state.bluepan.exec(eng);

                if eng.control.get_key_state(26) && state.tm <= 0 && !state.intram{
                    state.lsp.1 = false;
                    state.tm = 50;
                    state.cstop += 1;
                    state.intram = true;
                }
            } else {
                state.phcnt.draw = false;
                state.phcnt.exec(eng, " ");
                state.bluepan.object.draw = false;
                state.bluepan.exec(eng);
            }

            if state.cme && !state.intram{
                state.cambtn.object.physic_object.scale.x = 80.0;
                state.cambtn.object.physic_object.scale.y = 80.0;
                state.cambtn.object.physic_object.pos.x =
                    eng.render.resolution_x as f32 / 2.0 - state.cambtn.object.physic_object.scale.x;
                state.cambtn.object.physic_object.pos.y =
                    eng.render.resolution_y as f32 - state.cambtn.object.physic_object.scale.y;
                state.cambtn.object.draw = true;
                if state.cambtn.exec(eng) && state.tm <= 0 && eng.control.mousebtn[2] {
                    state.selp = 1;
                    state.tm = 50;
                }
            } else {
                state.cambtn.object.draw = false;
                state.cambtn.exec(eng);
            }
        }
        1 => {
            eng.used_light_count = state.locls + 1;
            eng.lights[0].color = Vec3 {
                x: 0.08,
                y: 0.09,
                z: 0.1,
            };

            eng.lights[1].rot.y = -state.scn.objects[state.pu].physic_object.rot.y;
            eng.lights[1].pos.x =
                state.scn.objects[state.pu].physic_object.pos.x - state.scn.objects[state.pu].physic_object.rot.y.sin() * 0.3;
            eng.lights[1].pos.y = state.scn.objects[state.pu].physic_object.pos.y;
            eng.lights[1].pos.z =
                state.scn.objects[state.pu].physic_object.pos.z - state.scn.objects[state.pu].physic_object.rot.y.cos() * 0.3;
            eng.lights[1].light_type = LightType::Spot;
            eng.lights[1].color = Vec3 {
                x: 5.0,
                y: 5.0,
                z: 5.0,
            };

            for i in 0..state.aproxpoint.len() {
                state.aproxpoint[i].x =
                    state.scn.objects[state.pu].physic_object.pos.x
                        - state.scn.objects[state.pu].physic_object.rot.y.sin() * (i + 1) as f32;
                state.aproxpoint[i].y =
                    state.scn.objects[state.pu].physic_object.pos.z
                        - state.scn.objects[state.pu].physic_object.rot.y.cos() * (i + 1) as f32;
            }

            if eng.control.get_key_state(48) && state.tm <= 0 && state.bwfilm > 0 {
                for i in 0..state.destructables.len() {
                    for j in 0..state.aproxpoint.len() {
                        if distance(
                            Vec3 {
                                x: state.aproxpoint[j].x,
                                y: state.scn.objects[state.destructables[i]].physic_object.pos.y,
                                z: state.aproxpoint[j].y,
                            },
                            state.scn.objects[state.destructables[i]].physic_object.pos,
                        ) <= (2.0 + j as f32)
                        {
                            state.scn.objects[state.destructables[i]].physic_object.pos.y = -1000.0;
                            state.scn.objects[state.destructables[i]].draw = false;
                            break;
                        }
                    }
                }
                state.pkbf = 2.0;
                state.bwfilm -= 1;
                state.tm = 50;
            }

            let tx = &format!("{}", state.bwfilm);
            state.phcnt.draw = true;
            state.phcnt.size.x = 15_f32;
            state.phcnt.size.y = 30_f32;
            state.phcnt.pos.z = 0.1;
            state.phcnt.pos.x =
                eng.render.resolution_x as f32 / 2.0 - ((tx.len() as f32 * state.phcnt.size.x) / 2.0);
            state.phcnt.pos.y = state.bwbtn.object.physic_object.pos.y - state.phcnt.size.y;
            state.phcnt.exec(eng, tx);

            state.bluepan.object.draw = true;
            state.bluepan.object.physic_object.scale.y = state.phcnt.size.y;
            state.bluepan.object.physic_object.scale.x = state.bwbtn.object.physic_object.scale.x * 2.0;
            state.bluepan.object.physic_object.pos.x =
                eng.render.resolution_x as f32 / 2.0 - state.bluepan.object.physic_object.scale.x / 2.0;
            state.bluepan.object.physic_object.pos.y = state.phcnt.pos.y;
            state.bluepan.object.mesh.ubo[50] = 0.0;
            state.bluepan.exec(eng);

            state.bwbtn.object.physic_object.scale.x = 80.0;
            state.bwbtn.object.physic_object.scale.y = 80.0;
            state.bwbtn.object.physic_object.pos.x =
            eng.render.resolution_x as f32 / 2.0 - state.bwbtn.object.physic_object.scale.x;
            state.bwbtn.object.physic_object.pos.y =
            eng.render.resolution_y as f32 - state.bwbtn.object.physic_object.scale.y;
            state.bwbtn.object.draw = true;
            if state.bwbtn.exec(eng) && state.tm <= 0 && eng.control.mousebtn[2] {
                state.selp = 2;
                state.tm = 50;
            }

            state.colbtn.object.draw = false;
            state.colbtn.exec(eng);

            state.cambtn.object.draw = false;
            state.cambtn.exec(eng);
        }
        2 => {
            eng.used_light_count = state.locls;
            eng.lights[0].color = Vec3 {
                x: 1.0,
                y: 0.9,
                z: 1.8,
            };
            let tx = &format!("{}", state.clfilm);
            state.phcnt.draw = true;
            state.phcnt.size.x = 15_f32;
            state.phcnt.size.y = 30_f32;
            state.phcnt.pos.z = 0.1;
            state.phcnt.pos.x =
                eng.render.resolution_x as f32 / 2.0 - ((tx.len() as f32 * state.phcnt.size.x) / 2.0);
            state.phcnt.pos.y = state.colbtn.object.physic_object.pos.y - state.phcnt.size.y;
            state.phcnt.exec(eng, tx);

            if eng.control.get_key_state(48) && state.tm <= 0 && state.bwfilm > 0 {
                state.lsp.0.x = state.scn.objects[state.pu].physic_object.pos.x;
                state.lsp.0.y = state.scn.objects[state.pu].physic_object.pos.z;
                state.lsp.1 = true;
                state.pkbf = 2.0;
                state.clfilm -= 1;
                state.tm = 50;
            } else if eng.control.get_key_state(26) && state.tm <= 0 {
                state.scn.objects[state.pu].physic_object.pos.x = state.lsp.0.x;
                state.scn.objects[state.pu].physic_object.pos.z = state.lsp.0.y;
                state.pkbf = 2.0;
                state.tm = 50;
            }

            state.bluepan.object.draw = true;
            state.bluepan.object.physic_object.scale.y = state.phcnt.size.y;
            state.bluepan.object.physic_object.scale.x = state.colbtn.object.physic_object.scale.x * 2.0;
            state.bluepan.object.physic_object.pos.x =
                eng.render.resolution_x as f32 / 2.0 - state.bluepan.object.physic_object.scale.x / 2.0;
            state.bluepan.object.physic_object.pos.y = state.phcnt.pos.y;
            state.bluepan.object.mesh.ubo[50] = 0.0;
            state.bluepan.exec(eng);

            state.bwbtn.object.draw = false;
            state.bwbtn.exec(eng);

            state.colbtn.object.physic_object.scale.x = 80.0;
            state.colbtn.object.physic_object.scale.y = 80.0;
            state.colbtn.object.physic_object.pos.x =
                eng.render.resolution_x as f32 / 2.0 - state.colbtn.object.physic_object.scale.x;
            state.colbtn.object.physic_object.pos.y =
                eng.render.resolution_y as f32 - state.colbtn.object.physic_object.scale.y;
            state.colbtn.object.draw = true;
            if state.colbtn.exec(eng) && state.tm <= 0 && eng.control.mousebtn[2] {
                state.selp = 0;
                state.tm = 50;
            }

            state.cambtn.object.draw = false;
            state.cambtn.exec(eng);
        }
        _ => {}
    }

    if state.intram{
        state.scn.objects[state.pu].physic_object.solid = false;
        state.scn.objects[state.pu].physic_object.pos = state.scn.objects[state.tramin].physic_object.pos;
        state.scn.objects[state.tramin].physic_object.acceleration.x += SPEED*5.0*eng.times_to_calculate_physics as f32;
        if state.scn.objects[state.tramin].physic_object.pos.x >= state.scn.objects[state.stops[(state.cstop-1) as usize]].physic_object.pos.x{
            state.intram = false;
            state.scn.objects[state.pu].physic_object.pos.x = state.scn.objects[state.stops[(state.cstop-1) as usize]].physic_object.pos.x;
            state.scn.objects[state.pu].physic_object.pos.z = state.scn.objects[state.stops[(state.cstop-1) as usize]].physic_object.pos.z + 5.0;
        }
    }else{
        state.scn.objects[state.pu].physic_object.solid = true;
    }

    state.psbtn.object.physic_object.scale.x = 80.0;
    state.psbtn.object.physic_object.scale.y = 80.0;
    state.psbtn.object.physic_object.pos.x = eng.render.resolution_x as f32 / 2.0;
    state.psbtn.object.physic_object.pos.y = eng.render.resolution_y as f32 - state.psbtn.object.physic_object.scale.y;
    if !state.cme || state.intram{
        state.psbtn.object.physic_object.pos.x =
            eng.render.resolution_x as f32 / 2.0 - state.psbtn.object.physic_object.scale.x / 2.0;
    }
    state.psbtn.exec(eng);
}
