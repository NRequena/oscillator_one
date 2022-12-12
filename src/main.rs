use nannou::prelude::*;
use nannou_audio as audio;
use nannou_audio::Buffer;
use std::f64::consts::PI;



fn main() {
    nannou::app(model)
        .event(event)
        .run();
}

struct Model {
    stream: audio::Stream<Audio>,
}

struct Audio {
    phase: f64,
    hz: f64,
}

fn model(app: &App) -> Model {
    // Create a window to receive key pressed events.
    app.new_window()
        // .key_pressed(key_pressed)
        .view(view)
        .build()
        .unwrap();

    // Initialise the audio API so we can spawn an audio stream.
    let audio_host = audio::Host::new();

    

    // Initialise the state that we want to live on the audio thread.
    let model = Audio {
        phase: 0.0,
        hz: 440.0,
    };

    let stream = audio_host
        .new_output_stream(model)
        .render(audio)
        .build()
        .unwrap();

    stream.play().unwrap();

    Model { stream }
}

// A function that renders the given `Audio` to the given `Buffer`.
// In this case we play a simple sine wave at the audio's current frequency in `hz`.
fn audio(audio: &mut Audio, buffer: &mut Buffer) {
    let sample_rate = buffer.sample_rate() as f64;
    let volume = 0.5;
    for frame in buffer.frames_mut() {
        let sine_amp = (2.0 * PI * audio.phase).sin() as f32;
        audio.phase += audio.hz / sample_rate;
        audio.phase %= sample_rate;
        for channel in frame {
            *channel = sine_amp * volume;
        }
    }
}

// fn key_pressed(_app: &App, model: &mut Model, key: Key) {

//     match key {
//         // Pause or unpause the audio when Space is pressed.
//         Key::Space => {
//             if model.stream.is_playing() {
//                 model.stream.pause().unwrap();
//             } else {
//                 model.stream.play().unwrap();
//             }
//         }
//         _ => {}
//     }
// }

fn event(_app: &App, model: &mut Model, _event: Event) {

    let boundary = _app.window_rect();
    let hertz = _app.mouse.y ;
    let y = map_range(hertz, boundary.bottom(), boundary.top(), 20.0, 440.0);

    model.stream.send(move |audio|{
        //audio.hz = y as f64;
        audio.hz = y as f64;
    })
    .unwrap();

}

fn view(_app: &App, _model: &Model, frame: Frame,) {

    // Prepare to draw.
    let draw = _app.draw();
    let boundary = _app.window_rect();
    let mouse = _app.mouse.y;
    let circle_size = map_range(mouse,boundary.bottom(), boundary.top(), 150.0, 20.0);

    draw.background().color(rgba(0.6,0.6,0.6,1.0));

    if _app.mouse.buttons.left().is_down() {
            _model.stream.play().unwrap();
        } else {
            _model.stream.pause().unwrap();
        }
    
    if _app.mouse.buttons.left().is_down() {
        draw.ellipse()
            .color(rgba(0.6,0.6,0.6,1.0))
            .w_h(circle_size,circle_size)
            .x_y(_app.mouse.x, _app.mouse.y);
        draw.background().color(rgba(0.2,0.2,0.2,1.0));
        } else {
            draw.ellipse()
            .color(rgba(0.2,0.2,0.2,1.0))
            .w_h(50.0,50.0)
            .x_y(_app.mouse.x, _app.mouse.y);
        draw.background().color(rgba(0.6,0.6,0.6,1.0));
        }
    
    draw.to_frame(_app, &frame).unwrap();
    
}