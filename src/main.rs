mod app_state;
mod engine;
mod init;
mod loop_tick;

fn main() {
    let (mut eng, mut state) = init::create_app();

    while eng.work() {
        loop_tick::tick(&mut eng, &mut state);
    }

    eng.end();
}