#![allow(dead_code)]

use crate::engine::{
    math::{vec2::Vec2, vec3::Vec3}, scene::Scene, speaker::Speaker, ui::{UIplane, UItext}
};

pub const SPEED: f32 = 0.0025_f32;
pub const TICKSZ: f32 = 1.0 / 250.0;

pub struct Colectable {
    pub index: usize,
    pub ctype: u8,
    pub consumed: bool,
}

pub struct Ingbutton{
    pub index: usize,
    pub axis: u8,
    pub pressed: bool,
    pub scene_index: u32,
    pub in_scene_index: u32,
}

pub struct Scenelightsource{
    pub pos: Vec3,
}

pub struct Door{
    pub index: usize,
    pub axis: u8,
    pub movement: f32,
    pub initial_pos: Vec3,
}

pub struct AppState {
    pub viewport: UIplane,
    pub bluepan: UIplane,
    pub cambtn: UIplane,
    pub bwbtn: UIplane,
    pub colbtn: UIplane,
    pub psbtn: UIplane,
    pub fpscnt: UItext,
    pub phcnt: UItext,
    pub scn: Scene,
    pub cvec: Vec<Colectable>,
    pub destructables: Vec<usize>,
    pub ekey: usize,
    pub gkey: usize,
    pub stops: Vec<usize>,
    pub btns: Vec<Ingbutton>,
    pub scenelightsources: Vec<Scenelightsource>,
    pub doors: Vec<Door>,
    pub cstop: u32,
    pub intram: bool,
    pub tm: i32,
    pub pu: usize,
    pub pivotr: f32,
    pub pkbf: f32,
    pub tramin: usize,
    pub bwfilm: u32,
    pub clfilm: u32,
    pub cme: bool,
    pub selp: u8,
    pub locls: u32,
    pub aproxpoint: [Vec2; 4],
    pub lsp: (Vec2, bool),
    pub sfx: Vec<Speaker>,
    pub dbg: bool,
    pub switch_states: [bool; 6],
    pub switched_1_4: bool,
    pub switched_5_6: bool,
    pub sc3state: u8,
    pub finaldooridx: usize,
    pub initial_pivot_pos: Vec3,
}

pub fn distance(v1: Vec3, v2: Vec3) -> f32 {
    f32::sqrt((v2.x - v1.x).powi(2) + (v2.z - v1.z).powi(2))
}
