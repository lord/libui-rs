//! An example control gallery: a port of the same `ui` example.

extern crate ui;

use ui::{BoxControl, Button, Checkbox, ColorButton, Combobox, DateTimePicker, Entry};
use ui::{FontButton, Group, InitOptions, Label, Menu, MenuItem, ProgressBar, RadioButtons};
use ui::{Separator, Slider, Spinbox, Tab, Window, AreaHandler, Area, AreaDrawParams, AreaMouseEvent, AreaKeyEvent};
use ui::draw::text::{Layout, FontDescriptor, Weight, Italic, Stretch};

struct Foo {
  text: String,
}

impl AreaHandler for Foo {
  fn draw(&mut self, area: &Area, area_draw_params: &AreaDrawParams) {
    let ctx = &area_draw_params.context;
    let w = Weight::Normal;
    let i = Italic::Normal;
    let s = Stretch::Normal;
    let font = FontDescriptor::new("Operator Mono Ssm", 11.0, w, i, s).load_closest_font();
    self.text = "chicken!".to_string();
    let lay = Layout::new(&self.text, &font, 1000.0);
    lay.set_color(1,3,0.0,1.0,0.0,1.0);
    ctx.draw_text(0.0, 0.0, &lay);
  }
  fn mouse_event(&mut self, _area: &Area, e: &AreaMouseEvent) {
    println!("{:?}", e);
  }
  // fn mouse_crossed(&mut self, _area: &Area, _left: bool) { ... }
  // fn drag_broken(&mut self, _area: &Area) { ... }
  fn key_event(&mut self, _area: &Area, e: &AreaKeyEvent) -> bool {
    println!("{:?} : {:?} : {:?}", e.key, e.modifiers, e.up);
    true
  }
}

fn run() {
    let menu = Menu::new("File");
    menu.append_item("Open").on_clicked(Box::new(open_clicked));
    menu.append_item("Save").on_clicked(Box::new(save_clicked));

    let menu = Menu::new("Edit");
    menu.append_check_item("Checkable Item");
    menu.append_separator();
    let item = menu.append_item("Disabled Item");
    item.disable();
    menu.append_preferences_item();

    let menu = Menu::new("Help");
    menu.append_item("Help");
    menu.append_about_item();

    let mainwin = Window::new("ui Control Gallery", 640, 480, true);
    mainwin.set_margined(false);
    mainwin.on_closing(Box::new(|_| {
        ui::quit();
        false
    }));
    let area = Area::new_scrolling(Box::new(Foo{text: "meow".to_string()}), 640, 500);
    mainwin.set_child(area.into());

    mainwin.show();
    ui::main();
}

pub fn main() {
    ui::init(InitOptions).unwrap();
    run();
    ui::uninit();
}

fn open_clicked(_: &MenuItem, mainwin: &Window) {
    match ui::open_file(mainwin) {
        Some(filename) => ui::msg_box(mainwin, "File selected", &*filename),
        None => ui::msg_box_error(mainwin, "No file selected", "Don't be alarmed!"),
    }
}

fn save_clicked(_: &MenuItem, mainwin: &Window) {
    match ui::open_file(mainwin) {
        Some(filename) => {
            ui::msg_box(mainwin, "File selected (don't worry, it's still there)", &*filename)
        }
        None => ui::msg_box_error(mainwin, "No file selected", "Don't be alarmed!"),
    }
}

fn update(_: i64) {
    // TODO(pcwalton)
}

