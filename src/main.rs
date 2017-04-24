
extern crate gtk;
extern crate glium;
extern crate glib; // Needed by gtk to supply a threaded fn idle_add

#[derive(Debug)]
enum GFX {
    Vulkan,
    OpenGL
}

fn main() {

    // get the location of xi-core
    let core_path = std::env::var("XI_CORE")
                .unwrap_or("../xi-editor/rust/target/debug/xi-core".into());
    let gfx_mode = match std::env::var("MODE").unwrap_or("gl".into()).as_ref() {
        "vulkan" => {
            println!("Running in Vulkan mode");
            GFX::Vulkan
        }
        _ => {
            println!("Running in OpenGL mode");
            GFX::OpenGL
        }
    };

    match gfx_mode {
        GFX::Vulkan => panic!("Sorry, Vulkan mode is not implemented yet.\
                              Please rerun using GFX=OpenGL"),
        GFX::OpenGL =>
            ::std::thread::spawn(move || {
                use glium::DisplayBuild;

                let display = glium::glutin::WindowBuilder::new()
                    .with_dimensions(800, 600)
                    .with_title("xi-vix".to_string())
                    .build_glium().unwrap();

                display.get_window().unwrap() .set_cursor(glium::glutin::MouseCursor::Text);
             }),
        _ => panic!("Invalid mode: {:?}", gfx_mode),
    };

    //::std::thread::spawn(move || {

    gtk::init().expect("Failed to initialize GTK.");
    gtk::main();

    println!("Hello World!");
}
