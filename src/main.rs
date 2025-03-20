use ascii_arcade::{
    scene,
    user_input::{self, Cmd},
};
use ratatui::widgets::Paragraph;

fn main() {
    let mut terminal = scene::init().unwrap();
    let rx = user_input::create_data_channel();

    'menu: loop {
        terminal
            .draw(|frame| {
                let greeting = Paragraph::new("Hello World! (press 'q' to exit)");
                frame.render_widget(greeting, frame.area());
            })
            .unwrap();

        if let Cmd::EXIT = user_input::keyboard_control(&rx) {
            break 'menu;
        }
    }

    scene::close(terminal);
}
