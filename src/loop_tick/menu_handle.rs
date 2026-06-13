use crate::{
    app_state::AppState,
    engine::{engine::Engine, math::{vec2::Vec2, vec3::Vec3}}, loop_tick::save_load::{load_progress, save_progress},
};

use super::handle_scene::reset_final_door_game;

pub fn menu_handle(eng: &mut Engine, state: &mut AppState) {
    if state.pausemn{
        state.selp = 0;
        match state.menusel{
            0 => {
                state.logo.object.physic_object.scale.x = 200f32;
                state.logo.object.physic_object.scale.y = 200f32;
                state.logo.object.physic_object.pos.x = 0.0;
                state.logo.object.physic_object.pos.y = eng.render.resolution_y as f32/2.0 - 241.0;
                state.logo.object.draw = true;
                state.logo.signal = false;
                state.logo.exec(eng);

                let mut lg = state.logo.object.physic_object.scale.x;

                for i in 0..6{
                    state.ruitxt[i].signal = true;
                    state.ruitxt[i].per_symbol = false;
                    state.ruitxt[i].draw = true;
                    state.ruitxt[i].size = Vec2{ x: 20f32, y: 40f32};
                    state.ruitxt[i].pos = Vec3{ x: 0f32, y: state.logo.object.physic_object.pos.y+state.logo.object.physic_object.scale.y+6.0+i as f32*(state.ruitxt[i].size.y+6.0), z: 0.1};
                    let txt = state.jsontext.other_nodes[0].other_nodes[0].other_nodes[3].other_nodes[i+1].strval.clone();
                    if txt.len() as f32 * state.ruitxt[i].size.x > lg{
                        lg = txt.len() as f32 * state.ruitxt[i].size.x;
                    }
                    if state.ruitxt[i].exec(eng, &txt) && eng.control.mousebtn[2] && state.tm <= 0{
                        match i {
                            0 => {
                                state.pausemn = !state.pausemn;
                                state.tm = 50;
                            },
                            1 => {
                                reset_final_door_game(state);
                                state.pkbf = 2.0;
                                state.tm = 50;
                                state.pausemn = false;
                                state.sfx[1].move_sound_cursor(0.0);
                                state.sfx[1].play = true;
                            }
                            2 => {
                                let _ = save_progress("save.json", state);
                                state.pausemn = false;
                                state.tm = 50;
                            }
                            3 => {
                                let _ = load_progress("save.json", state);
                                state.pausemn = false;
                                state.pkbf = 2.0;
                                state.sfx[1].move_sound_cursor(0.0);
                                state.sfx[1].play = true;
                                state.tm = 50;
                            }
                            4 => {
                                state.menusel = 1;
                                state.tm = 50;
                            }
                            5 => {
                                state.close = true;
                            },
                            _ => {}
                        }
                    }
                }

                state.bluepan.object.physic_object.scale.x = lg;
                state.bluepan.object.physic_object.scale.y = eng.render.resolution_y as f32;
                state.bluepan.object.physic_object.pos.x = 0f32;
                state.bluepan.object.physic_object.pos.y = 0f32;
                state.bluepan.object.draw = true;
                state.bluepan.object.mesh.ubo[50] = 0.0;
                state.bluepan.exec(eng);
            },
            1 => {
                state.logo.object.draw = false;
                state.logo.exec(eng);

                state.ruitxt[0].signal = false;
                state.ruitxt[0].per_symbol = false;
                state.ruitxt[0].draw = true;
                state.ruitxt[0].size = Vec2{ x: 30f32, y: 60f32};
                state.ruitxt[0].pos = Vec3{ x: 0f32, y: eng.render.resolution_y as f32/2.0 - 148.0, z: 0.1};
                let txt = state.jsontext.other_nodes[0].other_nodes[0].other_nodes[3].other_nodes[5].strval.clone();
                state.ruitxt[0].exec(eng, &txt);

                let mut lg = state.ruitxt[0].size.x*txt.len() as f32;

                for i in 1..5{
                    let newind = i+6;
                    state.ruitxt[i].signal = true;
                    state.ruitxt[i].per_symbol = false;
                    state.ruitxt[i].draw = true;
                    state.ruitxt[i].size = Vec2{ x: 20f32, y: 40f32};
                    state.ruitxt[i].pos = Vec3{ x: 0f32, y: state.ruitxt[0].pos.y+state.ruitxt[0].size.y+6.0+(i-1) as f32*(state.ruitxt[i].size.y+6.0), z: 0.1};
                    let txt = state.jsontext.other_nodes[0].other_nodes[0].other_nodes[3].other_nodes[newind].strval.clone();
                    if txt.len() as f32 * state.ruitxt[i].size.x > lg{
                        lg = txt.len() as f32 * state.ruitxt[i].size.x;
                    }
                    if state.ruitxt[i].exec(eng, &txt) && eng.control.mousebtn[2] && state.tm <= 0{
                        match i {
                            1 => {
                                state.menusel = 2;
                                state.tm = 50;
                            },
                            2 => {
                                state.menusel = 3;
                                state.tm = 50;
                            },
                            3 => {
                                state.menusel = 4;
                                state.tm = 50;
                            },
                            4 => {
                                state.menusel = 5;
                                state.tm = 50;
                            },
                            _ => {}
                        }
                    }
                }

                state.ruitxt[5].signal = true;
                state.ruitxt[5].per_symbol = false;
                state.ruitxt[5].draw = true;
                state.ruitxt[5].size = Vec2{ x: 20f32, y: 40f32};
                state.ruitxt[5].pos = Vec3{ x: 0f32, y: state.ruitxt[0].pos.y+state.ruitxt[0].size.y+6.0+4.0*(state.ruitxt[5].size.y+6.0), z: 0.1};
                let txt = state.jsontext.other_nodes[0].other_nodes[0].other_nodes[3].other_nodes[0].strval.clone();
                if txt.len() as f32 * state.ruitxt[5].size.x > lg{
                    lg = txt.len() as f32 * state.ruitxt[5].size.x;
                }
                if state.ruitxt[5].exec(eng, &txt) && eng.control.mousebtn[2] && state.tm <= 0{
                    state.menusel = 0;
                    state.tm = 50;
                }

                state.bluepan.object.physic_object.scale.x = lg;
                state.bluepan.object.physic_object.scale.y = eng.render.resolution_y as f32;
                state.bluepan.object.physic_object.pos.x = 0f32;
                state.bluepan.object.physic_object.pos.y = 0f32;
                state.bluepan.object.draw = true;
                state.bluepan.object.mesh.ubo[50] = 0.0;
                state.bluepan.exec(eng);
            }
            2 => {
                state.logo.object.draw = false;
                state.logo.exec(eng);

                state.ruitxt[0].signal = false;
                state.ruitxt[0].per_symbol = false;
                state.ruitxt[0].draw = true;
                state.ruitxt[0].size = Vec2{ x: 30f32, y: 60f32};
                state.ruitxt[0].pos = Vec3{ x: 0f32, y: eng.render.resolution_y as f32/2.0 - 148.0, z: 0.1};
                let txt = state.jsontext.other_nodes[0].other_nodes[0].other_nodes[3].other_nodes[7].strval.clone();
                state.ruitxt[0].exec(eng, &txt);

                let mut lg = state.ruitxt[0].size.x*txt.len() as f32;

                for i in 1..5{
                    let newind = i+10;
                    state.ruitxt[i].signal = true;
                    state.ruitxt[i].per_symbol = false;
                    state.ruitxt[i].draw = true;
                    state.ruitxt[i].size = Vec2{ x: 20f32, y: 40f32};
                    state.ruitxt[i].pos = Vec3{ x: 0f32, y: state.ruitxt[0].pos.y+state.ruitxt[0].size.y+6.0+(i-1) as f32*(state.ruitxt[i].size.y+6.0), z: 0.1};
                    let txt = state.jsontext.other_nodes[0].other_nodes[0].other_nodes[3].other_nodes[newind].strval.clone();
                    if txt.len() as f32 * state.ruitxt[i].size.x > lg{
                        lg = txt.len() as f32 * state.ruitxt[i].size.x;
                    }
                    if state.ruitxt[i].exec(eng, &txt) && eng.control.mousebtn[2] && state.tm <= 0{
                        match i {
                            1 => {
                                //state.menusel = 2;
                                state.tm = 50;
                            },
                            2 => {
                                //state.menusel = 3;
                                state.tm = 50;
                            },
                            3 => {
                                //state.menusel = 4;
                                state.tm = 50;
                            },
                            4 => {
                                //state.menusel = 5;
                                state.tm = 50;
                            },
                            _ => {}
                        }
                    }
                }

                state.ruitxt[5].signal = true;
                state.ruitxt[5].per_symbol = false;
                state.ruitxt[5].draw = true;
                state.ruitxt[5].size = Vec2{ x: 20f32, y: 40f32};
                state.ruitxt[5].pos = Vec3{ x: 0f32, y: state.ruitxt[0].pos.y+state.ruitxt[0].size.y+6.0+4.0*(state.ruitxt[5].size.y+6.0), z: 0.1};
                let txt = state.jsontext.other_nodes[0].other_nodes[0].other_nodes[3].other_nodes[0].strval.clone();
                if txt.len() as f32 * state.ruitxt[5].size.x > lg{
                    lg = txt.len() as f32 * state.ruitxt[5].size.x;
                }
                if state.ruitxt[5].exec(eng, &txt) && eng.control.mousebtn[2] && state.tm <= 0{
                    state.menusel = 1;
                    state.tm = 50;
                }

                state.bluepan.object.physic_object.scale.x = lg;
                state.bluepan.object.physic_object.scale.y = eng.render.resolution_y as f32;
                state.bluepan.object.physic_object.pos.x = 0f32;
                state.bluepan.object.physic_object.pos.y = 0f32;
                state.bluepan.object.draw = true;
                state.bluepan.object.mesh.ubo[50] = 0.0;
                state.bluepan.exec(eng);
            },
            3 => {
                state.logo.object.draw = false;
                state.logo.exec(eng);

                state.ruitxt[0].signal = false;
                state.ruitxt[0].per_symbol = false;
                state.ruitxt[0].draw = true;
                state.ruitxt[0].size = Vec2{ x: 30f32, y: 60f32};
                state.ruitxt[0].pos = Vec3{ x: 0f32, y: eng.render.resolution_y as f32/2.0 - 79.0, z: 0.1};
                let txt = state.jsontext.other_nodes[0].other_nodes[0].other_nodes[3].other_nodes[8].strval.clone();
                state.ruitxt[0].exec(eng, &txt);

                let mut lg = state.ruitxt[0].size.x*txt.len() as f32;

                state.ruitxt[1].signal = true;
                state.ruitxt[1].per_symbol = false;
                state.ruitxt[1].draw = true;
                state.ruitxt[1].size = Vec2{ x: 20f32, y: 40f32};
                state.ruitxt[1].pos = Vec3{ x: 0f32, y: state.ruitxt[0].pos.y+state.ruitxt[0].size.y+6.0, z: 0.1};
                let txt = state.jsontext.other_nodes[0].other_nodes[0].other_nodes[3].other_nodes[15].strval.clone();
                if txt.len() as f32 * state.ruitxt[1].size.x > lg{
                    lg = txt.len() as f32 * state.ruitxt[1].size.x;
                }
                if state.ruitxt[1].exec(eng, &txt) && eng.control.mousebtn[2] && state.tm <= 0{
                    state.tm = 50;
                }

                state.ruitxt[2].signal = true;
                state.ruitxt[2].per_symbol = false;
                state.ruitxt[2].draw = true;
                state.ruitxt[2].size = Vec2{ x: 20f32, y: 40f32};
                state.ruitxt[2].pos = Vec3{ x: 0f32, y: state.ruitxt[0].pos.y+state.ruitxt[0].size.y+6.0+(state.ruitxt[2].size.y+6.0), z: 0.1};
                let txt = state.jsontext.other_nodes[0].other_nodes[0].other_nodes[3].other_nodes[0].strval.clone();
                if txt.len() as f32 * state.ruitxt[2].size.x > lg{
                    lg = txt.len() as f32 * state.ruitxt[5].size.x;
                }
                if state.ruitxt[2].exec(eng, &txt) && eng.control.mousebtn[2] && state.tm <= 0{
                    state.menusel = 1;
                    state.tm = 50;
                }

                for i in 3..state.ruitxt.len(){
                    state.ruitxt[i].draw = false;
                    state.ruitxt[i].exec(eng, " ");
                }

                state.bluepan.object.physic_object.scale.x = lg;
                state.bluepan.object.physic_object.scale.y = eng.render.resolution_y as f32;
                state.bluepan.object.physic_object.pos.x = 0f32;
                state.bluepan.object.physic_object.pos.y = 0f32;
                state.bluepan.object.draw = true;
                state.bluepan.object.mesh.ubo[50] = 0.0;
                state.bluepan.exec(eng);
            },
            4 => {
                state.logo.object.draw = false;
                state.logo.exec(eng);

                state.ruitxt[0].signal = false;
                state.ruitxt[0].per_symbol = false;
                state.ruitxt[0].draw = true;
                state.ruitxt[0].size = Vec2{ x: 30f32, y: 60f32};
                state.ruitxt[0].pos = Vec3{ x: 0f32, y: eng.render.resolution_y as f32/2.0 - 79.0, z: 0.1};
                let txt = state.jsontext.other_nodes[0].other_nodes[0].other_nodes[3].other_nodes[9].strval.clone();
                state.ruitxt[0].exec(eng, &txt);

                let mut lg = state.ruitxt[0].size.x*txt.len() as f32;

                state.ruitxt[1].signal = true;
                state.ruitxt[1].per_symbol = false;
                state.ruitxt[1].draw = true;
                state.ruitxt[1].size = Vec2{ x: 20f32, y: 40f32};
                state.ruitxt[1].pos = Vec3{ x: 0f32, y: state.ruitxt[0].pos.y+state.ruitxt[0].size.y+6.0, z: 0.1};
                let txt = state.jsontext.other_nodes[0].other_nodes[0].other_nodes[3].other_nodes[16].strval.clone();
                if txt.len() as f32 * state.ruitxt[1].size.x > lg{
                    lg = txt.len() as f32 * state.ruitxt[1].size.x;
                }
                if state.ruitxt[1].exec(eng, &txt) && eng.control.mousebtn[2] && state.tm <= 0{
                    state.tm = 50;
                }

                state.ruitxt[2].signal = true;
                state.ruitxt[2].per_symbol = false;
                state.ruitxt[2].draw = true;
                state.ruitxt[2].size = Vec2{ x: 20f32, y: 40f32};
                state.ruitxt[2].pos = Vec3{ x: 0f32, y: state.ruitxt[0].pos.y+state.ruitxt[0].size.y+6.0+(state.ruitxt[2].size.y+6.0), z: 0.1};
                let txt = state.jsontext.other_nodes[0].other_nodes[0].other_nodes[3].other_nodes[0].strval.clone();
                if txt.len() as f32 * state.ruitxt[2].size.x > lg{
                    lg = txt.len() as f32 * state.ruitxt[5].size.x;
                }
                if state.ruitxt[2].exec(eng, &txt) && eng.control.mousebtn[2] && state.tm <= 0{
                    state.menusel = 1;
                    state.tm = 50;
                }

                for i in 3..state.ruitxt.len(){
                    state.ruitxt[i].draw = false;
                    state.ruitxt[i].exec(eng, " ");
                }

                state.bluepan.object.physic_object.scale.x = lg;
                state.bluepan.object.physic_object.scale.y = eng.render.resolution_y as f32;
                state.bluepan.object.physic_object.pos.x = 0f32;
                state.bluepan.object.physic_object.pos.y = 0f32;
                state.bluepan.object.draw = true;
                state.bluepan.object.mesh.ubo[50] = 0.0;
                state.bluepan.exec(eng);
            },
            5 => {
                state.logo.object.draw = false;
                state.logo.exec(eng);

                state.ruitxt[0].signal = false;
                state.ruitxt[0].per_symbol = false;
                state.ruitxt[0].draw = true;
                state.ruitxt[0].size = Vec2{ x: 30f32, y: 60f32};
                state.ruitxt[0].pos = Vec3{ x: 0f32, y: eng.render.resolution_y as f32/2.0 - 102.0, z: 0.1};
                let txt = state.jsontext.other_nodes[0].other_nodes[0].other_nodes[3].other_nodes[9].strval.clone();
                state.ruitxt[0].exec(eng, &txt);

                let mut lg = state.ruitxt[0].size.x*txt.len() as f32;

                for i in 1..3{
                    let newind = i+16;
                    state.ruitxt[i].signal = true;
                    state.ruitxt[i].per_symbol = false;
                    state.ruitxt[i].draw = true;
                    state.ruitxt[i].size = Vec2{ x: 20f32, y: 40f32};
                    state.ruitxt[i].pos = Vec3{ x: 0f32, y: state.ruitxt[0].pos.y+state.ruitxt[0].size.y+6.0+(i-1) as f32*(state.ruitxt[i].size.y+6.0), z: 0.1};
                    let txt = state.jsontext.other_nodes[0].other_nodes[0].other_nodes[3].other_nodes[newind].strval.clone();
                    if txt.len() as f32 * state.ruitxt[i].size.x > lg{
                        lg = txt.len() as f32 * state.ruitxt[i].size.x;
                    }
                    if state.ruitxt[i].exec(eng, &txt) && eng.control.mousebtn[2] && state.tm <= 0{
                        match i {
                            1 => {
                                //state.menusel = 2;
                                state.tm = 50;
                            },
                            2 => {
                                //state.menusel = 3;
                                state.tm = 50;
                            }
                            _ => {}
                        }
                    }
                }

                state.ruitxt[3].signal = true;
                state.ruitxt[3].per_symbol = false;
                state.ruitxt[3].draw = true;
                state.ruitxt[3].size = Vec2{ x: 20f32, y: 40f32};
                state.ruitxt[3].pos = Vec3{ x: 0f32, y: state.ruitxt[0].pos.y+state.ruitxt[0].size.y+6.0+2.0*(state.ruitxt[3].size.y+6.0), z: 0.1};
                let txt = state.jsontext.other_nodes[0].other_nodes[0].other_nodes[3].other_nodes[0].strval.clone();
                if txt.len() as f32 * state.ruitxt[3].size.x > lg{
                    lg = txt.len() as f32 * state.ruitxt[3].size.x;
                }
                if state.ruitxt[3].exec(eng, &txt) && eng.control.mousebtn[2] && state.tm <= 0{
                    state.menusel = 1;
                    state.tm = 50;
                }

                for i in 4..state.ruitxt.len(){
                    state.ruitxt[i].draw = false;
                    state.ruitxt[i].exec(eng, " ");
                }

                state.bluepan.object.physic_object.scale.x = lg;
                state.bluepan.object.physic_object.scale.y = eng.render.resolution_y as f32;
                state.bluepan.object.physic_object.pos.x = 0f32;
                state.bluepan.object.physic_object.pos.y = 0f32;
                state.bluepan.object.draw = true;
                state.bluepan.object.mesh.ubo[50] = 0.0;
                state.bluepan.exec(eng);
            },
            _ => {},
        }
    }else{
        for i in 1..8{
            state.ruitxt[i].draw = false;
            state.ruitxt[i].exec(eng, " ");
        }
        state.psbtn.object.draw = true;
    }
}