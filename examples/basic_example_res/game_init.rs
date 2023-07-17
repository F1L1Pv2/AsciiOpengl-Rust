use ascii_opengl_rust::engine::core::Game;
use ascii_opengl_rust::engine::matrices::model_matrix;
use ascii_opengl_rust::engine::object::{ Object, TextureFilter };
use ascii_opengl_rust::engine::scene::Scene;
use ascii_opengl_rust::engine::ui::{ draw_rect, draw_text };
use fontdue::Font;

pub fn game_init(terminal_res: (u32, u32), game: &mut Game, display: &glium::Display) {
    let mut scene: Scene = Scene::new();

    game.add_scene(
        Scene::load_from_json("/scenes/scene1.json", game.assets_path.as_str(), display).unwrap()
    );

    scene.add_object(
        Object::new(
            "examples/basic_example_res/assets/models/monke.obj",
            None,
            TextureFilter::Linear,
            model_matrix(&[0.0, 0.0, 2.0], &[0.0, 0.0, 0.0], &[1.0, 1.0, 1.0]),
            display,
            vec!["monke".to_string()]
        )
    );

    game.add_scene(scene);

    display
        .gl_window()
        .window()
        .set_cursor_position(
            glium::glutin::dpi::PhysicalPosition::new(
                (terminal_res.0 as f64) / 2.0,
                (terminal_res.1 as f64) / 2.0
            )
        )
        .unwrap();

    game.camera.player_rot = [0.0, 0.0, 0.0];

    // let mut scene = Scene::new();

    // scene.add_object(Object::new(
    //     "examples/basic_example_res/assets/models/cube.obj",
    //     "examples/basic_example_res/assets/sprites/exampletexture.png".into(),
    //     TextureFilter::Linear,
    //     model_matrix(&[-4.0, 0.0, 2.0], &[0.0, 0.0, 0.0], &[1.0, 1.0, 1.0]),
    //     display,
    //     vec!["cube".to_string()]
    // ));

    // scene.add_object(Object::new(
    //     "examples/basic_example_res/assets/models/cube.obj",
    //     None,
    //     TextureFilter::Linear,
    //     model_matrix(&[4.0, 0.0, 2.0], &[0.0, 0.0, 0.0], &[1.0, 1.0, 1.0]),
    //     display,
    //     vec!["cube".to_string()]
    // ));

    // scene.add_object(Object::new(
    //     "examples/basic_example_res/assets/models/cube.obj",
    //     None,
    //     TextureFilter::Linear,
    //     model_matrix(&[0.0, 0.0, 6.0], &[0.0, 0.0, 0.0], &[1.0, 1.0, 1.0]),
    //     display,
    //     vec!["cube".to_string()]
    // ));

    // scene.add_object(Object::new(
    //     "examples/basic_example_res/assets/models/cube.obj",
    //     None,
    //     TextureFilter::Linear,
    //     model_matrix(&[0.0, 0.0, -2.0], &[0.0, 0.0, 0.0], &[1.0, 1.0, 1.0]),
    //     display,
    //     vec!["cube".to_string()]
    // ));

    // game.add_scene(scene);

    let font = Font::from_bytes(
        include_bytes!("assets/fonts/Roboto-Regular.ttf") as &[u8],
        fontdue::FontSettings::default()
    ).unwrap();

    game.add_ui_elems(draw_text(0.0, 0.0, "Text", 3.0, &font, display));
    // game.add_ui_elems(draw_text(0.0, 0.5, "b", 20.0, &font, &display));

    // return;

    game.add_ui_elem(
        draw_rect(
            0.0,
            0.5,
            0.25,
            0.5,
            "examples/basic_example_res/assets/sprites/exampletexture.png",
            TextureFilter::Linear,
            display
        )
    );
}
