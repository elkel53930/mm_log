use gtk::cairo::Context;
use gtk::prelude::*;
use gtk::cairo;

// import Rc, RefCell
use std::cell::RefCell;
use std::rc::Rc;

struct Status{
    x: f64,
    y: f64,
    size : f64,
}

// Bothersome. So I made it a global variable. Forgive me...
static mut STATUS: Status = Status{x: 0.0, y: 0.0, size:3.0};

fn main() {
    let application =
        gtk::Application::new(Some("com.github.gtk-rs.examples.basic"), Default::default());
    application.connect_activate(build_ui);
    application.run();
}

fn build_ui(application: &gtk::Application) {
    let status : Rc<RefCell<Status>> = Rc::new(RefCell::new(Status{x: 0.0, y: 0.0, size:3.0}));
    let window = gtk::ApplicationWindow::new(application);

    window.set_title(Some("First GTK Program"));
    window.set_default_size(350, 70);

    let vbox = gtk::Box::builder()
        .orientation(gtk::Orientation::Vertical)
        .halign(gtk::Align::Start)
        .spacing(6)
        .margin_bottom(6)
        .margin_top(6)
        .margin_start(6)
        .margin_end(6)
        .width_request(400)
        .height_request(400)
        .build();

    vbox.append(&build_button());
    vbox.append(&build_scale());
    vbox.append(&build_draw());
    vbox.append(&build_dropdown());

    window.set_child(Some(&vbox));

    window.show();
}

fn build_button() -> gtk::Button {
    let button = gtk::Button::with_label("Click me!");
    button.connect_clicked(|_| {
        eprintln!("Clicked!");
    });
    button
}

fn build_scale() -> gtk::Scale {
    let scale = gtk::Scale::builder()
        .draw_value(true)
        .adjustment(
            &gtk::Adjustment::builder()
                .lower(0.0)
                .upper(100.0)
                .step_increment(1.0)
                .page_increment(10.0)
                .page_size(0.0)
                .build(),
        )
        .digits(0)
        .round_digits(0)
        .build();
    scale.connect_value_changed(|s| {
        println!("value changed: {}", s.value());
    });
    scale
}

fn e(r: Result<(), cairo::Error>) {
    match r {
        Ok(_) => (),
        Err(e) => eprintln!("Error: {:?}", e),
    }
}

fn draw_area(cr: &Context) {
    cr.set_source_rgba(0.0, 0.0, 0.0, 1.0); // 黒色、不透明
    cr.set_line_width(2.0);
    cr.move_to(50.0, 50.0);
    cr.line_to(200.0, 200.0);
    e(cr.stroke());

    cr.set_source_rgba(1.0, 0.0, 0.0, 0.5); // 赤色、透明度50%
    cr.rectangle(50.0, 50.0, 100.0, 100.0);
    e(cr.fill());

    cr.set_source_rgba(0.0, 0.0, 1.0, 0.8); // 青色、透明度20%
    let x = unsafe { STATUS.x };
    let y = unsafe { STATUS.y };
    let size = unsafe { STATUS.size };
    cr.arc(x, y, size, 0.0, 2.0 * std::f64::consts::PI);
    e(cr.fill());
}

fn build_dropdown() -> gtk::DropDown {
    let dropdown = gtk::DropDown::from_strings(&["One", "Two", "Three"]);
    dropdown.set_selected(1);
    dropdown.connect_selected_notify(|dropdown| {
        println!("Selected item: {:?}", dropdown.selected());
    });
    dropdown
}

fn build_draw() -> gtk::DrawingArea {
    let draw = gtk::DrawingArea::new();
    draw.set_content_height(400);
    draw.set_content_width(400);
    draw.set_draw_func(move |_, cr, _, _| {
        draw_area(&cr);
    });

    let ctrl_motion = gtk::EventControllerMotion::new();
    ctrl_motion.connect_motion(|ctx, x, y| {
        let draw = ctx.widget().downcast::<gtk::DrawingArea>().unwrap();
        unsafe {
            STATUS.x = x;
            STATUS.y = y;
        }
        draw.queue_draw();
    });
    draw.add_controller(ctrl_motion);

    let ctrl_scroll = gtk::EventControllerScroll::new(gtk::EventControllerScrollFlags::VERTICAL);
    ctrl_scroll.connect_scroll(|ctx, _: f64, dy| {
        let draw = ctx.widget().downcast::<gtk::DrawingArea>().unwrap();
        unsafe {
            STATUS.size += dy;
            if STATUS.size < 3.0 {
                STATUS.size = 3.0;
            }
        }
        draw.queue_draw();
        glib::signal::Propagation::Proceed
    });
    draw.add_controller(ctrl_scroll);

    draw
}
