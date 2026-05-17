use std::fs;

use crate::{
    app_state::*,
    engine::{
        engine::Engine, image::Image, material::Material, math::vec3::Vec3, math::vec2::Vec2, scene::Scene, speaker::Speaker, ui::{UIplane, UItext}
    },
};

pub fn create_app(show_dbg_info: bool, skipl2: bool) -> (Engine, AppState) {
    let mut eng = Engine::new();
    eng.render.set_title("35mm");
    eng.render.set_new_resolution(1280, 720);

    let vert = fs::read("shaders/vert").unwrap();
    let frag = fs::read("shaders/frag").unwrap();
    let dvert = fs::read("shaders/vdeffered").unwrap();
    let dfrag = fs::read("shaders/fdeffered").unwrap();
    let shadow = fs::read("shaders/shadow").unwrap();
    let textf = fs::read("shaders/ftext").unwrap();
    let imgf = fs::read("shaders/fimg").unwrap();

    let matt = Material::new(
        &eng,
        vert.clone(),
        textf,
        vec![],
        [
            super::engine::render::render::CullMode::CullModeNone,
            super::engine::render::render::CullMode::CullModeNone,
        ],
    );
    let mati = Material::new(
        &eng,
        vert.clone(),
        imgf,
        vec![],
        [
            super::engine::render::render::CullMode::CullModeNone,
            super::engine::render::render::CullMode::CullModeNone,
        ],
    );
    let mat = Material::new(
        &eng,
        vert.clone(),
        frag,
        vec![],
        [
            super::engine::render::render::CullMode::CullModeNone,
            super::engine::render::render::CullMode::CullModeNone,
        ],
    );
    let matgeneral = Material::new(
        &eng,
        dvert.clone(),
        dfrag,
        shadow.clone(),
        [
            super::engine::render::render::CullMode::CullModeBackBit,
            super::engine::render::render::CullMode::CullModeFrontBit,
        ],
    );
    let black = Image::new_color(&eng, [11, 23, 40, u8::MAX]);
    let bti1 = Image::new_from_files(&eng, vec!["assets/ui/cam.png".to_string()]);
    let bti2 = Image::new_from_files(&eng, vec!["assets/ui/bwf.png".to_string()]);
    let bti3 = Image::new_from_files(&eng, vec!["assets/ui/bwc.png".to_string()]);
    let bti4 = Image::new_from_files(&eng, vec!["assets/ui/pas.png".to_string()]);
    let bti5 = Image::new_from_files(&eng, vec!["assets/ui/btn.png".to_string()]);
    let bti6 = Image::new_from_files(&eng, vec!["assets/ui/nk.png".to_string()]);
    let bti7 = Image::new_from_files(&eng, vec!["assets/ui/tram.png".to_string()]);
    let bti8 = Image::new_from_files(&eng, vec!["assets/ui/ne.png".to_string()]);
    let bti9 = Image::new_from_files(&eng, vec!["assets/ui/fin.png".to_string()]);

    let mut viewport = UIplane::new(&mut eng, mat, black);
    viewport.object.physic_object.pos.z = 1.0;
    viewport.signal = false;

    let mut bluepan = UIplane::new(&mut eng, mati, black);
    bluepan.object.physic_object.pos.z = 0.2;
    bluepan.signal = false;

    let mut cambtn = UIplane::new(&mut eng, mati, bti1);
    cambtn.object.physic_object.pos.z = 0.1;
    cambtn.signal = true;
    let mut bwbtn = UIplane::new(&mut eng, mati, bti2);
    bwbtn.object.physic_object.pos.z = 0.1;
    bwbtn.signal = true;
    let mut colbtn = UIplane::new(&mut eng, mati, bti3);
    colbtn.object.physic_object.pos.z = 0.1;
    colbtn.signal = true;
    let mut psbtn = UIplane::new(&mut eng, mati, bti4);
    psbtn.object.physic_object.pos.z = 0.1;
    psbtn.signal = true;

    let mut btnbtn = UIplane::new(&mut eng, mati, bti5);
    btnbtn.object.physic_object.pos.z = 0.1;
    btnbtn.signal = true;
    let mut nkbtn = UIplane::new(&mut eng, mati, bti6);
    nkbtn.object.physic_object.pos.z = 0.1;
    nkbtn.signal = true;
    let mut trambtn = UIplane::new(&mut eng, mati, bti7);
    trambtn.object.physic_object.pos.z = 0.1;
    trambtn.signal = true;
    let mut nebtn = UIplane::new(&mut eng, mati, bti8);
    nebtn.object.physic_object.pos.z = 0.1;
    nebtn.signal = true;
    let mut drbtn = UIplane::new(&mut eng, mati, bti9);
    drbtn.object.physic_object.pos.z = 0.1;
    drbtn.signal = true;

    let mut fpscnt = UItext::new_from_file(
        &mut eng,
        matt,
        "assets/textlat.png",
        "aAbBcCdDeEfFgGhHiIjJkKlLmMnNoOpPqQrRsStTuUvVwWxXyYzZ0123456789,.;:'+-<>_[]{}/*`~$%",
    );
    let phcnt = UItext::new_from_file(
        &mut eng,
        matt,
        "assets/textlat.png",
        "aAbBcCdDeEfFgGhHiIjJkKlLmMnNoOpPqQrRsStTuUvVwWxXyYzZ0123456789,.;:'+-<>_[]{}/*`~$%",
    );

    let mut scn = Scene::load_from_gltf(&mut eng, "assets/scene.glb", matgeneral);

    eng.cameras[0].physic_object.gravity = false;
    eng.cameras[0].physic_object.solid = false;
    eng.render.shadow_map_resolution = 2000;

    let mut cvec = vec![];
    let mut destructables = vec![];
    let mut stops = vec![];
    let mut btns = vec![];
    let mut doors = vec![];
    let mut ltsc = vec![]; 
    let mut pu = 0usize;
    let mut tramin = 0usize;

    let mut ekey = 0usize;
    let mut gkey = 0usize;
    let mut fdi = 0usize;

    for i in 0..scn.objects.len() {
        scn.objects[i].draw_distance = 1000_f32;
        if scn.objects[i].name == "Pivot" {
            pu = i;
        } else {
            let bt = scn.objects[i].name.as_bytes();
            if bt[0] == b'c' && bt[1] == b'a' && bt[2] == b'm' {
                scn.objects[i].physic_object.gravity = false;
                scn.objects[i].physic_object.is_static = false;
                scn.objects[i].physic_object.solid = false;
                cvec.push(Colectable {
                    index: i,
                    ctype: 0,
                    consumed: false,
                });
                if show_dbg_info{
                    println!("Camera collectible found at index {}, pos ({}, {}, {})", i, scn.objects[i].physic_object.pos.x, scn.objects[i].physic_object.pos.y, scn.objects[i].physic_object.pos.z);
                }
            } else if bt[0] == b'b' && bt[1] == b'w' && bt[2] == b'f' {
                scn.objects[i].physic_object.gravity = false;
                scn.objects[i].physic_object.is_static = false;
                scn.objects[i].physic_object.solid = false;
                cvec.push(Colectable {
                    index: i,
                    ctype: 1,
                    consumed: false,
                });
                if show_dbg_info{
                    println!("B&W film collectible found at index {}, pos ({}, {}, {})", i, scn.objects[i].physic_object.pos.x, scn.objects[i].physic_object.pos.y, scn.objects[i].physic_object.pos.z);
                }
            } else if bt[0] == b'c' && bt[1] == b'l' && bt[2] == b'f' {
                scn.objects[i].physic_object.gravity = false;
                scn.objects[i].physic_object.is_static = false;
                scn.objects[i].physic_object.solid = false;
                cvec.push(Colectable {
                    index: i,
                    ctype: 2,
                    consumed: false,
                });
                if show_dbg_info{
                    println!("Color film collectible found at index {}, pos ({}, {}, {})", i, scn.objects[i].physic_object.pos.x, scn.objects[i].physic_object.pos.y, scn.objects[i].physic_object.pos.z);
                }
            } else if bt[0] == b'c' && bt[1] == b'm' && bt[2] == b'r'{
                if bt[3] == b'e'{
                    ekey = i;
                    destructables.push(i);
                    if show_dbg_info{
                        println!("destructable ekey found at index {}, pos ({}, {}, {})", i, scn.objects[i].physic_object.pos.x, scn.objects[i].physic_object.pos.y, scn.objects[i].physic_object.pos.z);
                    }
                } else if bt[3] == b'g'{
                    gkey = i;
                    destructables.push(i);
                    if show_dbg_info{
                        println!("destructable gkey found at index {}, pos ({}, {}, {})", i, scn.objects[i].physic_object.pos.x, scn.objects[i].physic_object.pos.y, scn.objects[i].physic_object.pos.z);
                    }
                }else{
                    destructables.push(i);
                    if show_dbg_info{
                        println!("destructable found at index {}, pos ({}, {}, {})", i, scn.objects[i].physic_object.pos.x, scn.objects[i].physic_object.pos.y, scn.objects[i].physic_object.pos.z);
                    }
                }
            } else if bt[0] == b't' && bt[1] == b'r' && bt[2] == b'a' && bt[3] == b'm' {
                scn.objects[i].physic_object.gravity = false;
                scn.objects[i].physic_object.is_static = false;
                scn.objects[i].physic_object.solid = false;
                tramin = i;
            } else if bt[0] == b'f' && bt[1] == b'i' && bt[2] == b'n' && bt[3] == b'a' && bt[4] == b'l' {
                fdi = i;
            } else if bt[0] == b'n' && bt[1] == b's' {
                scn.objects[i].physic_object.solid = false;
            } else if bt[0] == b'o' && bt[1] == b'p' {
                stops.push(i);
            }else if bt[0] == b'b' && bt[1] == b't' && bt[2] == b'n' {
                btns.push(Ingbutton {
                    index: i,
                    axis: match bt[3] {
                        b'x' => 0,
                        b'y' => 1,
                        b'z' => 2,
                        _ => 0,
                    },
                    pressed: false,
                    scene_index: (bt[5]-b'0') as u32,
                    in_scene_index: (bt[4]-b'0') as u32,
                });
                if show_dbg_info{
                    println!("button found at index {}, pos ({}, {}, {}), scene_index: {}, in_scene_index: {}", i, scn.objects[i].physic_object.pos.x, scn.objects[i].physic_object.pos.y, scn.objects[i].physic_object.pos.z, btns.last().unwrap().scene_index, btns.last().unwrap().in_scene_index);
                }
            }else if bt[0] == b'd' {
                doors.push(Door {
                    index: i,
                    axis: match bt[1] {
                        b'x' => 0,
                        b'y' => 1,
                        b'z' => 2,
                        _ => 0,
                    },
                    movement: match bt[2] {
                        b'-' => -1.0 * (bt[3]-b'0') as f32,
                        _ =>(bt[3]-b'0') as f32,
                    },
                    initial_pos: Vec3 {
                        x: scn.objects[i].physic_object.pos.x,
                        y: scn.objects[i].physic_object.pos.y,
                        z: scn.objects[i].physic_object.pos.z,
                    },
                });
                if show_dbg_info{
                    println!("door found at index {}, pos ({}, {}, {}), axis: {}, movement: {}", i, scn.objects[i].physic_object.pos.x, scn.objects[i].physic_object.pos.y, scn.objects[i].physic_object.pos.z, doors.last().unwrap().axis, doors.last().unwrap().movement);
                }
            }else if bt[0] == b's' && bt[1] == b't' && bt[2] == b'l' {
                btns.push(Ingbutton {
                    index: i,
                    axis: match bt[3] {
                        b'x' => 4,
                        b'y' => 5,
                        b'z' => 6,
                        _ => 4,
                    },
                    pressed: false,
                    scene_index: (bt[5]-b'0') as u32,
                    in_scene_index: (bt[4]-b'0') as u32,
                });
                if show_dbg_info{
                    println!("switch found at index {}, pos ({}, {}, {}), scene_index: {}, in_scene_index: {}", i, scn.objects[i].physic_object.pos.x, scn.objects[i].physic_object.pos.y, scn.objects[i].physic_object.pos.z, btns.last().unwrap().scene_index, btns.last().unwrap().in_scene_index);
                }
            }else if bt[0] == b'l' && bt[1] == b't'{
                scn.objects[i].draw = false;
                ltsc.push(Scenelightsource {
                    pos: Vec3 {
                        x: scn.objects[i].physic_object.pos.x,
                        y: scn.objects[i].physic_object.pos.y,
                        z: scn.objects[i].physic_object.pos.z,
                    },
                });
                if show_dbg_info{
                    println!("light source found at index {}, pos ({}, {}, {})", i, scn.objects[i].physic_object.pos.x, scn.objects[i].physic_object.pos.y, scn.objects[i].physic_object.pos.z);
                }
            }
        }
    }

    scn.objects[pu].physic_object.gravity = true;
    scn.objects[pu].physic_object.is_static = false;
    scn.objects[pu].physic_object.solid = true;
    scn.objects[pu].physic_object.step_height = 0.1;

    fpscnt.pos.x = 0.0;
    fpscnt.pos.y = 0.0;

    let mut sfx = vec![];

    sfx.push(Speaker::new(&mut eng, "assets/audio/walking.mp3"));
    sfx[0].loopsound = true;
    sfx[0].play = false;
    sfx[0].pos_dependency = false;
    sfx[0].use_pan = false;
    sfx.push(Speaker::new(&mut eng, "assets/audio/shutter.mp3"));
    sfx[1].loopsound = false;
    sfx[1].play = false;
    sfx[1].pos_dependency = false;
    sfx[1].use_pan = false;
    sfx.push(Speaker::new(&mut eng, "assets/audio/buzz.mp3"));
    sfx[2].loopsound = true;
    sfx[2].play = false;
    sfx[2].pos_dependency = false;
    sfx[2].use_pan = false;
    sfx.push(Speaker::new(&mut eng, "assets/audio/switch.mp3"));
    sfx[3].loopsound = false;
    sfx[3].play = false;
    sfx[3].pos_dependency = false;
    sfx[3].use_pan = false;
    sfx.push(Speaker::new(&mut eng, "assets/audio/tension.mp3"));
    sfx[4].loopsound = false;
    sfx[4].play = false;
    sfx[4].pos_dependency = false;
    sfx[4].use_pan = false;
    sfx.push(Speaker::new(&mut eng, "assets/audio/tram.mp3"));
    sfx[5].loopsound = true;
    sfx[5].play = false;
    sfx[5].pos_dependency = false;
    sfx[5].use_pan = false;
    sfx.push(Speaker::new(&mut eng, "assets/audio/pickup.mp3"));
    sfx[6].loopsound = false;
    sfx[6].play = false;
    sfx[6].pos_dependency = false;
    sfx[6].use_pan = false;

    for i in 0..sfx.len() {
        sfx[i].volume = 1.0;
    }

    eng.audio.vol = 1.0;

    let initial_pivot_pos = scn.objects[pu].physic_object.pos;

    let state = AppState {
        viewport,
        bluepan,
        cambtn,
        bwbtn,
        colbtn,
        psbtn,
        btnbtn,
        nkbtn,
        trambtn,
        nebtn,
        drbtn,
        fpscnt,
        phcnt,
        scn,
        cvec,
        destructables,
        ekey: ekey,
        gkey: gkey,
        btns: btns,
        scenelightsources: ltsc,
        doors: doors,
        stops,
        cstop: 0_u32,
        intram: false,
        tm: 0,
        pu,
        pivotr: 0.0_f32,
        pkbf: 1_f32,
        tramin,
        bwfilm: 0_u32,
        clfilm: 0_u32,
        cme: false,
        selp: 0_u8,
        locls: 1,
        aproxpoint: [
            Vec2 { x: 0.0, y: 0.0 },
            Vec2 { x: 0.0, y: 0.0 },
            Vec2 { x: 0.0, y: 0.0 },
            Vec2 { x: 0.0, y: 0.0 },
        ],
        lsp: (Vec2 { x: 0.0, y: 0.0 }, false),
        sfx: sfx,
        dbg: show_dbg_info,
        switch_states: [false; 6],
        switched_1_4: false,
        switched_5_6: false,
        sc3state: 0,
        finaldooridx: fdi,
        initial_pivot_pos,
        skp2: skipl2,
    };

    (eng, state)
}
