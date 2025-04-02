use ascii_arcade::games::SandboxGame;
use ascii_arcade::{scene, user_input};

fn main() {
    let terminal = scene::init();
    let rx = user_input::create_data_channel();
    SandboxGame::play(&rx);
    scene::close(&mut terminal.unwrap());
}
