use std::fs;

use crate::{
    app_state::{AppState, Colectable},
    engine::{
        engine::Engine,
        image::Image,
        material::Material,
        math::vec2::Vec2,
        scene::Scene,
        ui::{UIplane, UItext},
    },
};

pub fn create_app() -> (Engine, AppState) {
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
    let mut pu = 0usize;
    let mut tramin = 0usize;

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
            } else if bt[0] == b'b' && bt[1] == b'w' && bt[2] == b'f' {
                scn.objects[i].physic_object.gravity = false;
                scn.objects[i].physic_object.is_static = false;
                scn.objects[i].physic_object.solid = false;
                cvec.push(Colectable {
                    index: i,
                    ctype: 1,
                    consumed: false,
                });
            } else if bt[0] == b'c' && bt[1] == b'l' && bt[2] == b'f' {
                scn.objects[i].physic_object.gravity = false;
                scn.objects[i].physic_object.is_static = false;
                scn.objects[i].physic_object.solid = false;
                cvec.push(Colectable {
                    index: i,
                    ctype: 2,
                    consumed: false,
                });
            } else if bt[0] == b'c' && bt[1] == b'm' && bt[2] == b'r' {
                destructables.push(i);
            } else if bt[0] == b't' && bt[1] == b'r' && bt[2] == b'a' && bt[3] == b'm' {
                scn.objects[i].physic_object.gravity = false;
                scn.objects[i].physic_object.is_static = false;
                scn.objects[i].physic_object.solid = false;
                tramin = i;
            } else if bt[0] == b'n' && bt[1] == b's' {
                scn.objects[i].physic_object.solid = false;
            } else if bt[0] == b'o' && bt[1] == b'p' {
                stops.push(i);
            }
        }
    }

    scn.objects[pu].physic_object.gravity = true;
    scn.objects[pu].physic_object.is_static = false;
    scn.objects[pu].physic_object.solid = true;
    scn.objects[pu].physic_object.step_height = 0.1;

    fpscnt.pos.x = 0.0;
    fpscnt.pos.y = 0.0;

    let state = AppState {
        viewport,
        bluepan,
        cambtn,
        bwbtn,
        colbtn,
        psbtn,
        fpscnt,
        phcnt,
        scn,
        cvec,
        destructables,
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
    };

    (eng, state)
}
