// // Prepare to draw.
// let draw = _app.draw();

// let boundary = _app.window_rect();
// let _win_p = boundary.pad(50.0);

// let sine = _app.time.sin();
// let lowersine = (_app.time / 2.0).sin();

// // Map the sine wave functions to ranges between the boundaries of the window
// let _x = map_range(sine, -1.0, 1.0, boundary.left(), boundary.right());
// let _y = map_range(lowersine,-1.0,1.0,boundary.bottom(), boundary.top());


// // Clear the background to purple.  
// draw.background()
//     .color(rgba(0.3,0.3,0.3,1.0));

// // Draw a blue ellipse with a radius of 10 at the (x,y) coordinates of (0.0, 0.0)
// let _r = Rect::from_w_h(100.0,100.0);
// draw.ellipse()
//     .color(DARKGRAY)
//     .x_y(_x,_y)
//     .w_h(100.0,100.0);

// draw.to_frame(_app, &frame).unwrap();