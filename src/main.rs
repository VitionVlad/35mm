use std::env;

mod app_state;
mod engine;
mod init;
mod loop_tick;

fn main() {
    let args: Vec<String> = env::args().collect();

    let (mut eng, mut state) = init::create_app(
        if args.len() > 1 && args[1] == "-debug" { 
            true 
        } 
        else { 
            false }
        );

    while eng.work() {
        loop_tick::soundwork::soundwork(&mut eng, &mut state);
        loop_tick::control_handle::control_handle(&mut eng, &mut state);
        loop_tick::tick::tick(&mut eng, &mut state);
        loop_tick::per_select_tick::per_select_tick(&mut eng, &mut state);
        loop_tick::handle_scene::handle_scene(&mut eng, &mut state);
    }

    eng.end();
}