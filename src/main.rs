mod textinput;
use textinput::OnUpdate;

use druid::{AppLauncher, Widget, WindowDesc};

fn main() {
    let win = WindowDesc::new(ui);

    AppLauncher::with_window(win).launch("".into()).unwrap()
}

fn ui() -> impl Widget<String> {
    druid::widget::TextBox::new().onupdate(|s: &String| println!("{}", s))
}
