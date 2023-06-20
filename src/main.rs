#[macro_use]
extern crate glium;

// -----------------------------------------------------
use fontdue::Font;
//------------------ My stuff --------------------------
mod engine;
use engine::scene::Scene;
use engine::core::{ init, run_event_loop};
use engine::matrices::model_matrix;
use engine::object::Object;
use engine::ui::{draw_text, draw_rect};
// -----------------------------------------------------

fn main() {
    let (
        terminal_res,
        terminal_fb,
        event_loop,
        display,
        program,
        ui_program,
        params,
        ui_params,
        mut game,
    ) = init();

    let mut scene: Scene = Scene::new();

    scene.add_object(
        Object::new(
            "assets/models/monke.obj",
            None,
            model_matrix(&[0.0, 0.0, 2.0], &[0.0, 0.0, 0.0], &[1.0, 1.0, 1.0]),
            &display
        )
    );

    game.add_scene(scene);

    let mut scene = Scene::new();

    scene.add_object(
        Object::new(
            "assets/models/cube.obj",
            "assets/sprites/align.png".into(),
            model_matrix(&[-4.0, 0.0, 2.0], &[0.0, 0.0, 0.0], &[1.0, 1.0, 1.0]),
            &display
        )
    );

    scene.add_object(
        Object::new(
            "assets/models/cube.obj",
            None,
            model_matrix(&[4.0, 0.0, 2.0], &[0.0, 0.0, 0.0], &[1.0, 1.0, 1.0]),
            &display
        )
    );

    scene.add_object(
        Object::new(
            "assets/models/cube.obj",
            None,
            model_matrix(&[0.0, 0.0, 6.0], &[0.0, 0.0, 0.0], &[1.0, 1.0, 1.0]),
            &display
        )
    );

    scene.add_object(
        Object::new(
            "assets/models/cube.obj",
            None,
            model_matrix(&[0.0, 0.0, -2.0], &[0.0, 0.0, 0.0], &[1.0, 1.0, 1.0]),
            &display
        )
    );

    game.add_scene(scene);

    let font = Font::from_bytes(
        include_bytes!("../assets/fonts/Roboto-Regular.ttf") as &[u8],
        fontdue::FontSettings::default()
    ).unwrap();

    game.add_ui_elems(draw_text(0.0, 0.0, "ballin", 3.0, &font, &display));
    // game.add_ui_elems(draw_text(0.0, 0.5, "b", 20.0, &font, &display));

    // return;


    game.add_ui_elem(draw_rect(0.0, 0.75, 0.25, 0.25, "assets/sprites/align.png", &display));

    // Main loop
    run_event_loop(terminal_res, terminal_fb, event_loop, display, program, ui_program, params, ui_params, game);
    
}
