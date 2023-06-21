use fontdue::Font;
use super::core::Game;
use super::scene::Scene;
use super::object::Object;
use super::ui::{draw_text, draw_rect};
use super::matrices::model_matrix;

pub fn game_init(
    game: &mut Game,
    display : &glium::Display,
){
    let mut scene: Scene = Scene::new();

    scene.add_object(
        Object::new(
            "assets/models/monke.obj",
            None,
            model_matrix(&[0.0, 0.0, 2.0], &[0.0, 0.0, 0.0], &[1.0, 1.0, 1.0]),
            display
        )
    );

    game.add_scene(scene);

    let mut scene = Scene::new();

    scene.add_object(
        Object::new(
            "assets/models/cube.obj",
            "assets/sprites/align.png".into(),
            model_matrix(&[-4.0, 0.0, 2.0], &[0.0, 0.0, 0.0], &[1.0, 1.0, 1.0]),
            display
        )
    );

    scene.add_object(
        Object::new(
            "assets/models/cube.obj",
            None,
            model_matrix(&[4.0, 0.0, 2.0], &[0.0, 0.0, 0.0], &[1.0, 1.0, 1.0]),
            display
        )
    );

    scene.add_object(
        Object::new(
            "assets/models/cube.obj",
            None,
            model_matrix(&[0.0, 0.0, 6.0], &[0.0, 0.0, 0.0], &[1.0, 1.0, 1.0]),
            display
        )
    );

    scene.add_object(
        Object::new(
            "assets/models/cube.obj",
            None,
            model_matrix(&[0.0, 0.0, -2.0], &[0.0, 0.0, 0.0], &[1.0, 1.0, 1.0]),
            display
        )
    );

    game.add_scene(scene);

    let font = Font::from_bytes(
        include_bytes!("../../assets/fonts/Roboto-Regular.ttf") as &[u8],
        fontdue::FontSettings::default()
    ).unwrap();

    game.add_ui_elems(draw_text(0.0, 0.0, "ballin", 3.0, &font, display));
    // game.add_ui_elems(draw_text(0.0, 0.5, "b", 20.0, &font, &display));

    // return;

    game.add_ui_elem(draw_rect(0.0, 0.75, 0.25, 0.25, "assets/sprites/align.png", display));
}