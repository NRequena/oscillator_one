// extern crate nannou;
// use nannou::prelude::*;

// fn main() {
//     nannou::app(model)
//         .update(update)
//         //.simple_window(view)
//         .run();
// }

// struct Model {

//     texture: wgpu::Texture,

// }

// fn model(_app: &App) -> Model {
//     _app.new_window().size(512, 512).view(view).build().unwrap();
//     //Load the Image
//     let assets = _app.assets_path().unwrap();
//     let img_path = assets.join("images").join("nature").join("nature_1.jpg");
//     let texture = wgpu::Texture::from_path(_app, img_path).unwrap();

//     Model { texture }
// }
    
// fn update(_app: &App, _model: &mut Model, _update: Update) {
// }

// fn view(app: &App, model: &Model, frame: Frame) {
//     frame.clear(BLACK);
  
//     let draw = app.draw();
//     draw.texture(&model.texture);
  
//     draw.to_frame(app, &frame).unwrap();
//   }
