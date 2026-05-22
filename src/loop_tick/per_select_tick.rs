use crate::{
    app_state::{distance, AppState, SPEED},
    engine::{engine::Engine, math::vec3::Vec3},
};

fn apply_tram_acceleration(ph: &mut crate::engine::physics::PhysicsObject, target_x: f32, distance_scale: f32, min_acc: f32, max_acc: f32) {
    let distance = target_x - ph.pos.x;
    if distance.abs() < f32::EPSILON {
        return;
    }

    let direction = distance.signum();
    let distance_factor = (distance.abs() / distance_scale).clamp(0.0, 1.0);
    let accel = min_acc + (max_acc - min_acc) * distance_factor;

    ph.acceleration.x += direction * accel;
}

pub fn per_select_tick(eng: &mut Engine, state: &mut AppState) {
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

            state.shbtn.object.draw = false;
            state.reccbtn.object.draw = false;

            if state.cme && !state.intram {
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

            if distance(
                state.scn.objects[state.pu].physic_object.pos,
                state.scn.objects[state.tramin].physic_object.pos,
            ) < 7.5 && !state.intram {
                if state.cstop < state.stops.len() as u32 && ((state.cstop == 1 && state.switched_1_4 && state.switched_5_6) || state.cstop != 1){
                    state.phcnt.draw = false;
                    state.bluepan.object.draw = false;

                    state.trambtn.object.physic_object.scale.x = 80.0;
                    state.trambtn.object.physic_object.scale.y = 80.0;
                    state.trambtn.object.physic_object.pos.x =
                        eng.render.resolution_x as f32 / 2.0 - state.trambtn.object.physic_object.scale.x / 2.0;
                    state.trambtn.object.physic_object.pos.y =
                        eng.render.resolution_y as f32 - state.trambtn.object.physic_object.scale.y * 2.0 - 20.0;
                    state.trambtn.object.draw = true;
                    let tram_pressed = state.trambtn.exec(eng) && eng.control.mousebtn[2];

                    let abtn_pressed = eng.control.gamepad_button_count > 0 && eng.control.get_gamepad_button_state(state.gamepad_buttons[0]);

                    if (eng.control.get_key_state(state.keycodes[0]) || tram_pressed || abtn_pressed) && state.tm <= 0 {
                        state.lsp.1 = false;
                        state.tm = 50;
                        state.ttm = 250;
                        state.intram = true;
                        state.sfx[7].move_sound_cursor(0.0);
                        state.sfx[7].play = true;
                        if state.dbg {
                            println!("tram will start in {} ticks, current cstop: {}, next stop: {}", state.ttm, state.cstop, state.cstop + 1);
                        }
                        state.cstop += 1;
                    }
                } else if state.cstop == 1 && !(state.switched_1_4 && state.switched_5_6){
                    state.nebtn.object.physic_object.scale.x = 80.0;
                    state.nebtn.object.physic_object.scale.y = 80.0;
                    state.nebtn.object.physic_object.pos.x =
                        eng.render.resolution_x as f32 / 2.0 - state.nebtn.object.physic_object.scale.x / 2.0;
                    state.nebtn.object.physic_object.pos.y =
                        eng.render.resolution_y as f32 - state.nebtn.object.physic_object.scale.y * 2.0 - 20.0;
                    state.nebtn.object.draw = true;
                    state.nebtn.exec(eng);
                }else {
                    state.nebtn.object.draw = false;
                    state.nebtn.exec(eng);
                    state.trambtn.object.draw = false;
                    state.trambtn.exec(eng);
                }
            }else{
                state.nebtn.object.draw = false;
                state.nebtn.exec(eng);
                state.trambtn.object.draw = false;
                state.trambtn.exec(eng);
            }
            state.bluepan.object.draw = false;
            state.bluepan.exec(eng);
            state.phcnt.draw = false;
            state.phcnt.exec(eng, " ");
            state.shbtn.object.draw = false;
            state.reccbtn.object.draw = false;
            state.shbtn.exec(eng);
            state.reccbtn.exec(eng);
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
            eng.lights[1].light_type = crate::engine::light::LightType::Spot;
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

            let mut touch_shutter = false;
            state.shbtn.object.draw = false;
            state.reccbtn.object.draw = false;
            if state.controlt == 1 {
                let button_size = 80.0;
                let mut sign = 1.0;
                let button_x = if state.left_hand {
                    sign = -1.0;
                    eng.render.resolution_x as f32 / 4.0 - button_size / 2.0
                } else {
                    eng.render.resolution_x as f32 * 3.0 / 4.0 - button_size / 2.0
                };
                let center_y = eng.render.resolution_y as f32 / 2.0;

                state.shbtn.object.physic_object.scale.x = button_size;
                state.shbtn.object.physic_object.scale.y = button_size;
                state.shbtn.object.physic_object.pos.x = button_x + sign*button_size;
                state.shbtn.object.physic_object.pos.y = center_y + button_size * 1.5;
                state.shbtn.object.draw = true;
                touch_shutter = state.shbtn.exec(eng) && eng.control.mousebtn[2];
            }

            let trigg = if eng.control.gamepad_axis_count > 0 {
                eng.control.get_gamepad_axis_state(state.gamepad_axes[2]) > 0.0
            } else {
                false
            };

            if (eng.control.get_key_state(state.keycodes[1]) || touch_shutter || trigg) && state.tm <= 0 && state.bwfilm > 0 {
                for i in 0..state.destructables.len() {
                    for j in 0..state.aproxpoint.len() {
                        if distance(
                            Vec3 {
                                x: state.aproxpoint[j].x,
                                y: state.scn.objects[state.destructables[i].index].physic_object.pos.y,
                                z: state.aproxpoint[j].y,
                            },
                            state.scn.objects[state.destructables[i].index].physic_object.pos,
                        ) <= (2.0 + j as f32)
                        {
                            if state.destructables[i].index == state.ekey {
                                state.ekey = usize::MAX;
                                if state.dbg {
                                    println!("ekey collected");
                                }
                            } else if state.destructables[i].index == state.gkey {
                                state.gkey = usize::MAX;
                                if state.dbg {
                                    println!("gkey collected");
                                }
                            }
                            state.scn.objects[state.destructables[i].index].physic_object.pos.y = -1000.0;
                            state.scn.objects[state.destructables[i].index].draw = false;
                            break;
                        }
                    }
                }
                state.pkbf = 2.0;
                state.bwfilm -= 1;
                state.tm = 50;
                state.sfx[1].move_sound_cursor(0.0);
                state.sfx[1].play = true;
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

            state.trambtn.object.draw = false;
            state.trambtn.exec(eng);
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

            let trigg = if eng.control.gamepad_axis_count > 0 {
                eng.control.get_gamepad_axis_state(state.gamepad_axes[2]) > 0.0
            } else {
                false
            };

            let ybtn_pressed = eng.control.gamepad_button_count > 0 && eng.control.get_gamepad_button_state(state.gamepad_buttons[3]);

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

            let mut touch_shutter = false;
            let mut touch_recc = false;
            state.shbtn.object.draw = false;
            state.reccbtn.object.draw = false;
            if state.controlt == 1 {
                let button_size = 80.0;
                let mut sign = 1.0;
                let button_x = if state.left_hand {
                    sign = -1.0;
                    eng.render.resolution_x as f32 / 4.0 - button_size / 2.0
                } else {
                    eng.render.resolution_x as f32 * 3.0 / 4.0 - button_size / 2.0
                };
                let center_y = eng.render.resolution_y as f32 / 2.0;

                state.shbtn.object.physic_object.scale.x = button_size;
                state.shbtn.object.physic_object.scale.y = button_size;
                state.reccbtn.object.physic_object.scale.x = button_size;
                state.reccbtn.object.physic_object.scale.y = button_size;

                state.shbtn.object.physic_object.pos.x = button_x + sign * button_size;
                state.shbtn.object.physic_object.pos.y = center_y + button_size * 2.0;
                state.shbtn.object.draw = true;
                touch_shutter = state.shbtn.exec(eng) && eng.control.mousebtn[2];

                state.reccbtn.object.physic_object.pos.x = button_x + sign * button_size;
                state.reccbtn.object.physic_object.pos.y = center_y;
                state.reccbtn.object.draw = true;
                touch_recc = state.reccbtn.exec(eng) && eng.control.mousebtn[2];
            }

            if (eng.control.get_key_state(state.keycodes[1]) || touch_shutter || trigg) && state.tm <= 0 && state.clfilm > 0 {
                state.lsp.0.x = state.scn.objects[state.pu].physic_object.pos.x;
                state.lsp.0.y = state.scn.objects[state.pu].physic_object.pos.z;
                state.lsp.1 = true;
                state.pkbf = 2.0;
                state.clfilm -= 1;
                state.tm = 50;
                state.sfx[1].move_sound_cursor(0.0);
                state.sfx[1].play = true;
            } else if (eng.control.get_key_state(state.keycodes[0]) || touch_recc || ybtn_pressed) && state.tm <= 0 && state.lsp.1 {
                state.scn.objects[state.pu].physic_object.pos.x = state.lsp.0.x;
                state.scn.objects[state.pu].physic_object.pos.z = state.lsp.0.y;
                state.pkbf = 2.0;
                state.tm = 50;
                state.sfx[1].move_sound_cursor(0.0);
                state.sfx[1].play = true;
            }

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

            state.trambtn.object.draw = false;
            state.trambtn.exec(eng);
        }
        _ => {}
    }

    if state.intram {
        let max_tram_acc = SPEED * 5.0 * eng.times_to_calculate_physics as f32;
        state.scn.objects[state.pu].physic_object.solid = false;
        state.scn.objects[state.pu].physic_object.pos = state.scn.objects[state.tramin].physic_object.pos;
        if state.ttm <= 0 {
            let target_x = state.scn.objects[state.stops[(state.cstop - 1) as usize]].physic_object.pos.x;
            let distance = target_x - state.scn.objects[state.tramin].physic_object.pos.x;
            let approaching_stop = distance.abs() < 30.0;

            apply_tram_acceleration(
                &mut state.scn.objects[state.tramin].physic_object,
                target_x,
                30.0,
                SPEED * 0.1 * eng.times_to_calculate_physics as f32,
                max_tram_acc,
            );
            if state.scn.objects[state.tramin].physic_object.pos.x >= target_x {
                state.intram = false;
                state.scn.objects[state.pu].physic_object.pos.x = state.scn.objects[state.stops[(state.cstop - 1) as usize]].physic_object.pos.x;
                state.scn.objects[state.pu].physic_object.pos.z = state.scn.objects[state.stops[(state.cstop - 1) as usize]].physic_object.pos.z + 2.5;
            }

            let tram_acc = state.scn.objects[state.tramin].physic_object.acceleration.x.abs();
            state.sfx[5].volume = (tram_acc / max_tram_acc).clamp(0.0, 1.0);
            if state.dbg {
                println!("tram_acc: {}, max_tram_acc: {}, volume: {}", tram_acc, max_tram_acc, state.sfx[5].volume);
            }

            let stop_sound_vol = if approaching_stop {
                state.sfx[5].volume*2.0
            } else {
                0.0
            };
            state.sfx[8].volume = stop_sound_vol;
            state.sfx[8].play = stop_sound_vol > 0.01;
        } else {
            state.sfx[8].play = false;
            state.sfx[8].volume = 0.0;
        }

        state.sfx[5].play = true;
    } else {
        state.sfx[5].play = false;
        state.sfx[5].volume = 0.0;
        state.sfx[8].play = false;
        state.sfx[8].volume = 0.0;
        state.scn.objects[state.pu].physic_object.solid = true;
    }
}
