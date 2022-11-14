use std::f64::consts::PI;
use std::time::Duration;
use gtk::cairo::Context;
use gtk::glib;
use gtk::{Application, ApplicationWindow, DrawingArea};
use gtk::prelude::*;

mod grid;
use crate::grid::Grid;

pub mod config;
use crate::config::Config;

const APP_ID: &str = "com.byron_pettigrew.self_avoiding_walk";

pub fn run(config: &Config) {
    let app = Application::builder().application_id(APP_ID).build();

    app.connect_activate(move |app| init_app(app, &config));

    app.run();
}

fn init_app(app: &Application, config: &Config) {

    let display = DrawingArea::new();
    let mut grid = Grid::new(config.size());
    grid.init();
    let dim = i32::try_from(grid.size).expect("DIM too large");
    let content_size = config.min_length() * dim;
    display.set_content_width(content_size);
    display.set_content_height(content_size);
    display.set_draw_func(move
        |display, context, width, height| {
            draw_loop(display, context, width, height, &mut grid)
        }
    );

    let window = ApplicationWindow::builder()
        .application(app)
        .title("Self Avoiding Random Walk")
        .child(&display)
        .build();

    window.present();

    let tick = move || {
        display.queue_draw();
        Continue(true)
    };
    glib::timeout_add_local(Duration::from_millis(1), tick);
}

fn draw_loop(
    _display: &DrawingArea,
    context: &Context,
    width: i32,
    height: i32,
    grid: &mut Grid,
) {
    // draw background
    context.set_source_rgb(0.03, 0.03, 0.03);
    context.paint().expect("Painting failed");

    // calculate scale factor for different sized windows
    let scale_factor = f64::from(width * height).sqrt()/(grid.size as f64);
    // prep for grid
    context.set_source_rgb(0.5, 0.5, 0.5);
    context.set_line_width(scale_factor/15.0);

    // draw grid
    let dim = grid.size as f64;
    let tile_w = (f64::from(width))/dim;
    let tile_h = (f64::from(height))/dim;
    for i in 0..grid.size {
        for j in 0..grid.size {
            context.rectangle(
                (i as f64)*tile_w,
                (j as f64)*tile_h,
                tile_w, tile_h
            )
        }
    }
    context.stroke().expect("Stroking failed");

    // update searcher
    for _ in 0..10000 {
        if !grid.is_done() {
            grid.next_path();
        } else {
            break
        }
    }

    // draw path
    context.set_line_width(scale_factor/5.0);
    context.set_source_rgb(1., 1., 1.);
    let center = [tile_w/2., tile_h/2.];
    for tile in grid.path.iter() {
        context.line_to(tile_w*(tile[0] as f64)+center[0], tile_h*(tile[1] as f64)+center[1])
    }
    context.stroke().expect("Stroking failed");

    // draw head of path
    if let Some(tile) = grid.path.last() {
        context.arc(tile_w*(tile[0] as f64)+center[0], tile_h*(tile[1] as f64)+center[1], scale_factor/5., 0., 2.*PI);
        context.fill().expect("Circle fill failed");
    }

    // draw tail of path
    if let Some(tile) = grid.path.first() {
        context.arc(tile_w*(tile[0] as f64)+center[0], tile_h*(tile[1] as f64)+center[1], scale_factor/8., 0., 2.*PI);
        context.fill().expect("Circle fill failed");
    }

    if grid.is_done() {
        context.set_font_size(scale_factor);
        context.set_source_rgb(0., 1., 0.);
        context.move_to(f64::from(width)/2., f64::from(height)/2.);
        context.show_text("Done!").expect("Text fill failed");
        context.stroke().expect("Stroking failed");
    }
}
