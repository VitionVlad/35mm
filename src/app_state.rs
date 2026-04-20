#![allow(dead_code)]

use crate::engine::{
    math::{vec2::Vec2, vec3::Vec3},
    scene::Scene,
    ui::{UIplane, UItext},
};

pub const SPEED: f32 = 0.0025_f32;
pub const TICKSZ: f32 = 1.0 / 250.0;

pub struct Colectable {
    pub index: usize,
    pub ctype: u8,
    pub consumed: bool,
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
    pub stops: Vec<usize>,
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
}

pub fn distance(v1: Vec3, v2: Vec3) -> f32 {
    f32::sqrt((v2.x - v1.x).powi(2) + (v2.z - v1.z).powi(2))
}
