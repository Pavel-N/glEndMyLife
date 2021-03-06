use glutin::event::{Event, KeyboardInput, VirtualKeyCode, WindowEvent, ElementState};
use glutin::event_loop::{ControlFlow, EventLoop};
use glutin::window::WindowBuilder;
use glutin::{Api, ContextBuilder, GlRequest};

use std::mem::{size_of, size_of_val};

use gl33::global_loader::*;
use gl33::*;

use nalgebra::{Matrix4, Rotation3, Vector, Vector3, Point3};

mod shader;
use shader::Shader;

const TITLE: &str = "This took me just like 25 hours ~ OpenGL";

fn main() {

    //
    // ─── WINDOW SETUP ───────────────────────────────────────────────────────────────
    //

    let event_loop = EventLoop::new();                     // Create event loop
    let window_builder = WindowBuilder::new()              // Set window attributes 
        .with_title(TITLE);

    let context = ContextBuilder::new()
        .with_gl(GlRequest::Specific(Api::OpenGl, (3, 3))) // OpenGL 3.3
        .with_vsync(true)                                  // Enable vsync 
        .build_windowed(window_builder, &event_loop)       // Build window with OpenGL context 
        .unwrap();
    let context = unsafe { context.make_current().unwrap() };

    let window_size = context.window().inner_size();
    let width = window_size.width as f32;
    let height = window_size.height as f32;


    unsafe {
        // Load OpenGL functions globally
        load_global_gl(&|ptr| {
            let c_str = std::ffi::CStr::from_ptr(ptr as *const i8);
            let r_str = c_str.to_str().unwrap();
            context.get_proc_address(r_str) as _
        });
    }

    unsafe {

        //
        // ─── VERTEX INPUT ────────────────────────────────────────────────
        //

        type Vertex = [f32; 8];
        /*let vertices: [Vertex; 4] = [ // Rectangle
            // Positions         // Colors       // Texture coordinates
            [  0.5,  0.5,  0.0,  1.0, 0.0, 0.0,  1.0, 0.0  ], // Top-right corner
            [  0.5, -0.5,  0.0,  0.0, 1.0, 0.0,  1.0, 1.0  ], // Bottom-right corner
            [ -0.5, -0.5,  0.0,  0.0, 0.0, 1.0,  0.0, 1.0  ], // Bottom-left corner
            [ -0.5,  0.5,  0.0,  1.1, 1.1, 1.1,  0.0, 0.0  ], // Top-left corner
        ];

        let indices: [u32; 6] = [
            0, 1, 3, // First triangle
            1, 2, 3,  // Second triangle
        ];*/

        let vertices: [Vertex; 8] = [
            [ -0.5,  0.5,  0.5,  1.0, 1.0, 1.0,  1.0, 0.0  ],
            [  0.5,  0.5,  0.5,  1.0, 1.0, 1.0,  1.0, 0.0  ],
            [ -0.5, -0.5,  0.5,  1.0, 1.0, 1.0,  1.0, 0.0  ],
            [  0.5, -0.5,  0.5,  1.0, 1.0, 1.0,  1.0, 0.0  ],

            [ -0.5,  0.5, -0.5,  1.0, 1.0, 0.0,  1.0, 0.0  ],
            [  0.5,  0.5, -0.5,  1.0, 1.0, 0.0,  1.0, 0.0  ],
            [ -0.5, -0.5, -0.5,  1.0, 1.0, 0.0,  1.0, 0.0  ],
            [  0.5, -0.5, -0.5,  1.0, 1.0, 0.0,  1.0, 0.0  ],
        ];

        let indices: [u32; 36] = [
            0, 2, 3,
            0, 1, 3,

            4, 5, 7,
            4, 6, 7,

            0, 4, 5,
            0, 1, 5,

            2, 6, 7,
            2, 3, 7,

            0, 4, 6,
            0, 2, 6,

            1, 5, 7,
            1, 3, 7
        ];

        /*
        let vertices: [[f32; 5]; 36] = [ // Cube
            [ -0.5, -0.5, -0.5,  0.0, 0.0 ],
            [  0.5, -0.5, -0.5,  1.0, 0.0 ],
            [  0.5,  0.5, -0.5,  1.0, 1.0 ],
            [  0.5,  0.5, -0.5,  1.0, 1.0 ],
            [ -0.5,  0.5, -0.5,  0.0, 1.0 ],
            [ -0.5, -0.5, -0.5,  0.0, 0.0 ],

            [ -0.5, -0.5,  0.5,  0.0, 0.0 ],
            [  0.5, -0.5,  0.5,  1.0, 0.0 ],
            [  0.5,  0.5,  0.5,  1.0, 1.0 ],
            [  0.5,  0.5,  0.5,  1.0, 1.0 ],
            [ -0.5,  0.5,  0.5,  0.0, 1.0 ],
            [ -0.5, -0.5,  0.5,  0.0, 0.0 ],

            [ -0.5,  0.5,  0.5,  1.0, 0.0 ],
            [ -0.5,  0.5, -0.5,  1.0, 1.0 ],
            [ -0.5, -0.5, -0.5,  0.0, 1.0 ],
            [ -0.5, -0.5, -0.5,  0.0, 1.0 ],
            [ -0.5, -0.5,  0.5,  0.0, 0.0 ],
            [ -0.5,  0.5,  0.5,  1.0, 0.0 ],

            [  0.5,  0.5,  0.5,  1.0, 0.0 ],
            [  0.5,  0.5, -0.5,  1.0, 1.0 ],
            [  0.5, -0.5, -0.5,  0.0, 1.0 ],
            [  0.5, -0.5, -0.5,  0.0, 1.0 ],
            [  0.5, -0.5,  0.5,  0.0, 0.0 ],
            [  0.5,  0.5,  0.5,  1.0, 0.0 ],

            [ -0.5, -0.5, -0.5,  0.0, 1.0 ],
            [  0.5, -0.5, -0.5,  1.0, 1.0 ],
            [  0.5, -0.5,  0.5,  1.0, 0.0 ],
            [  0.5, -0.5,  0.5,  1.0, 0.0 ],
            [ -0.5, -0.5,  0.5,  0.0, 0.0 ],
            [ -0.5, -0.5, -0.5,  0.0, 1.0 ],

            [ -0.5,  0.5, -0.5,  0.0, 1.0 ],
            [  0.5,  0.5, -0.5,  1.0, 1.0 ],
            [  0.5,  0.5,  0.5,  1.0, 0.0 ],
            [  0.5,  0.5,  0.5,  1.0, 0.0 ],
            [ -0.5,  0.5,  0.5,  0.0, 0.0 ],
            [ -0.5,  0.5, -0.5,  0.0, 1.0 ]
        ];
        */

        //
        // ─── ELEMENT OBJECT BUFFER ───────────────────────────────────────
        //

        let mut ebo = 0u32;
        glGenBuffers(1, &mut ebo);

        glBindBuffer(GL_ELEMENT_ARRAY_BUFFER, ebo);
        glBufferData(
            GL_ELEMENT_ARRAY_BUFFER,
            size_of_val(&indices) as isize,
            indices.as_ptr().cast(),
            GL_STATIC_DRAW
        );


        //
        // ─── VERTEX BUFFER OBJECT ────────────────────────────────────────
        //

        let mut vbo = 0u32;                  // Vertex Buffer Object
        glGenBuffers(1, &mut vbo);           // Generate ID corresponging to VBO
        assert_ne!(vbo, 0);                  // VBO is succesfully created
        glBindBuffer(GL_ARRAY_BUFFER, vbo);  // Binding the buffer to GL_ARRAY_BUFFER object
        
        glBufferData(                        // Copy vertex data to memory of GPU
            GL_ARRAY_BUFFER,                 // Current vertex is bound to this object
            size_of_val(&vertices) as isize, // Size of data in bytes
            vertices.as_ptr().cast(),        // Data to be send to buffer
            GL_STATIC_DRAW                   // How will GPU mannage the data:
                                             //  * GL_STREAM_DRAW - data is set only once 
                                             //                     and used at most few times
                                             //  * GL_STATIC_DRAW - data is set only once
                                             //                     and used many times
                                             //  * GL_DYNAMIC_DRAW - data us changed a lot
                                             //                      and used many times
        );


        //
        // ─── VERTEX BUFFER ARRAY OBJECT ──────────────────────────────────
        //

        let mut vao = 0u32;
        // Create vertex object id - generate one vertex array store it to VAO
        glGenVertexArrays(1, &mut vao);
        glBindVertexArray(vao);
        assert_ne!(vao, 0); // VAO is succesfully created


        //
        // ─── LINKING VERTEX ATTRIBUTES ───────────────────────────────────
        //

        let vertex_size = size_of::<Vertex>() as i32; // 3 * size_of::<f32>() as i32;

        // Position attribute
        glVertexAttribPointer( // Linking vertex attributes
            0,                 // In vertex shader (location = 0)
            3,                 // Size of vertex attribute. vec3 => 3 values
            GL_FLOAT,
            0,                 // If 1 is supplied tha data will be normalized between 0.0 and 1.0
            vertex_size,       // Stride - https://shorturl.me/GyNm - here size of one vertex
            0 as *const _      // Offset
        );
        glEnableVertexAttribArray(0); // Enable attribute

        // Color attribute
        glVertexAttribPointer( // Linking vertex attributes
            1,                 // In vertex shader (location = 1)
            3,                 // Size of vertex attribute. vec3 => 3 values
            GL_FLOAT,
            0,                 // If 1 is supplied tha data will be normalized between 0.0 and 1.0
            vertex_size,       // Stride - https://shorturl.me/GyNm - here size of one vertex
            (3 * size_of::<f32>()) as *const _      // Offset
        );
        glEnableVertexAttribArray(1); // Enable attribute

        // Texture coordinates attribute
        glVertexAttribPointer(
            2,                 // In vertex shader (location = 2)
            2,                 // Size of vertex attribute. vec2 => 2 values
            GL_FLOAT,
            0,                 // If 1 is supplied tha data will be normalized between 0.0 and 1.0
            vertex_size,       // Stride - https://shorturl.me/GyNm - here size of one vertex
            (6 * size_of::<f32>()) as *const _      // Offset
                                                    // 0     3     6   8
                                                    // |-----|-----|---|
                                                    //    |     |    |
                                                    //   pos  color texture
        );
        glEnableVertexAttribArray(2); // Enable attribute


        //
        // ─── TEXTURE ─────────────────────────────────────────────────────
        //

        let border_color: [f32; 4] = [ 1.0, 0.0, 0.0, 1.0 ];    

        glTexParameterfv(GL_TEXTURE_2D, GL_TEXTURE_BORDER_COLOR, border_color.as_ptr().cast());
        glTexParameteri(GL_TEXTURE_2D, GL_TEXTURE_WRAP_S, GL_REPEAT.0 as i32);
        glTexParameteri(GL_TEXTURE_2D, GL_TEXTURE_WRAP_T, GL_REPEAT.0 as i32);
        glTexParameteri(GL_TEXTURE_2D, GL_TEXTURE_MIN_FILTER, GL_NEAREST.0 as i32);
        glTexParameteri(GL_TEXTURE_2D, GL_TEXTURE_MAG_FILTER, GL_NEAREST.0 as i32);

        // Loading image
        let car_img = {
            let img_bytes = include_bytes!("images/car.png");
            let cursor = std::io::Cursor::new(img_bytes);
            let decoder = png::Decoder::new(cursor);

            let (info, mut reader) = decoder.read_info().expect("You are using JPEG againg, aren't you");
            let buf_size = (info.width * info.height) as usize * 4;
            let mut img_data = vec![0; buf_size];
            reader.next_frame(&mut img_data).unwrap();
            (img_data, info)
        };

        /*
        let other_img = {
            let img_bytes = include_bytes!("images/img.png");
            let cursor = std::io::Cursor::new(img_bytes);
            let decoder = png::Decoder::new(cursor);

            let (info, mut reader) = decoder.read_info().expect("You are using JPEG againg, aren't you");
            let buf_size = (info.width * info.height) as usize * 4;
            let mut img_data = vec![0; buf_size];
            reader.next_frame(&mut img_data).unwrap();
            (img_data, info)
        };
        */
            
        // Generating texture
        let mut texture1 = 0u32;
        //let mut texture2 = 0u32;
        glGenTextures(1, &mut texture1);
        //glGenTextures(1, &mut texture2);


        glActiveTexture(GL_TEXTURE0);
        glBindTexture(GL_TEXTURE_2D, texture1);
        glTexImage2D(
            GL_TEXTURE_2D,              // Texture type TEXTURE_3D AND TEXTURE_1D arent affected.
            0,                          // Mipmap level 0 => base level
            0x1908, // GL_RGBA          // Format for storing the texture
            car_img.1.width as i32,     // Image width
            car_img.1.height as i32,    // Image height
            0,                          // Some legacy shit => always 0
            GL_RGB,                     // Image color format
            GL_UNSIGNED_BYTE,           // Image datatype; this one is stored as bytes
            car_img.0.as_ptr().cast()   // Image data
        );
        glGenerateMipmap(GL_TEXTURE_2D);
        
        
        /*
        glActiveTexture(GL_TEXTURE1);
        glBindTexture(GL_TEXTURE_2D, texture2);
        glTexImage2D(
            GL_TEXTURE_2D,              // Texture type TEXTURE_3D AND TEXTURE_1D arent affected.
            0,                          // Mipmap level 0 => base level
            GL_RGBA.0 as i32,           // Format for storing the texture
            other_img.1.width as i32,   // Image width
            other_img.1.height as i32,  // Image height
            0,                          // Some legacy shit => always 0
            GL_RGB,                     // Image color format
            GL_UNSIGNED_BYTE,           // Image datatype; this one is stored as bytes
            other_img.0.as_ptr().cast() // Image data
        );
        glGenerateMipmap(GL_TEXTURE_2D);
        */


        //
        // ─── SHADERS ─────────────────────────────────────────────────────
        //

        // Shader program
        let shader_program = Shader::new("shaders/vertex.vert", "shaders/fragment.frag");


        //
        // ─── ETC ─────────────────────────────────────────────────────────
        //

        // Set background color
        glClearColor(0.275, 0.51, 0.706, 1.0);

        // Set polygon mode
        glPolygonMode(GL_FRONT_AND_BACK, GL_FILL); // Filled 
        //glPolygonMode(GL_FRONT_AND_BACK, GL_LINE); // Wireframe

        /* Maximum number of possible shader attributes => in my case: 16
        let mut attrib_num = 0i32;
        glGetIntegerv(GL_MAX_VERTEX_ATTRIBS, &mut attrib_num);
        println!(">> {}", attrib_num);
        */

        //let mut delta_time = 0.0;
        let mut last_frame = 0.0;
        let mut last_fps = 0.0;

        fn radians(degrees: f32) -> f32 {
            degrees * (std::f32::consts::PI / 180.0)
        }

        let mut pitch = 0.0;
        let mut yaw = 1.0;
        let mut direction = Vector3::new(0.0, 0.0, 0.0);
        let mut last_x = 0.0;
        let mut last_y = 0.0;

        // Projection
        let mut projection_matrix = Matrix4::new_perspective(width/height, 45.0, 0.1, 100.0);


        //
        // ─── CAMERA ──────────────────────────────────────────────────────
        //

        let mut camera_pos = Point3::new(0.0, 0.0, 3.0);
        let camera_pos_vec = Vector3::new(camera_pos.x, camera_pos.y, camera_pos.z);
        let mut camera_speed = 0.05;
        let mut camera_front = Vector3::new(0.0, 0.0, -1.0);
        let up = Vector3::new(0.0, 1.0, 0.0);
        let camera_target = Vector3::new(0.0, 0.0, 0.0);
        let camera_direction: Vector3::<f32> = (camera_pos_vec - camera_target).normalize();
        let camera_right = (cross_v3f32(up, camera_direction)).normalize();
        let camera_up = cross_v3f32(camera_direction, camera_right);

        fn cross_v3f32(v1: Vector3::<f32>, v2: Vector3::<f32>) -> Vector3::<f32> {
            // TODO Proper function
            Vector3::new(v1.y*v2.z - v2.y*v1.z, -(v1.x*v2.z - v2.x*v1.z), v1.x*v2.y - v2.x*v1.y)
        }


        //
        // ─── EVENT LOOP ──────────────────────────────────────────────────
        //
        
        let t0 = std::time::Instant::now();
        event_loop.run(move |event, _, control_flow| {
            match event {

                //
                // INPUT
                //

                Event::WindowEvent { event, .. } => match event {

                    // When window close is requested
                    WindowEvent::CloseRequested => *control_flow = ControlFlow::Exit,

                    // When window is resized
                    WindowEvent::Resized(new_size) => {
                        context.resize(new_size); // Resize window context
                        glViewport( // Resize OpenGL viewport
                            0,
                            0,
                            new_size.width as i32,
                            new_size.height as i32
                        );
                        projection_matrix = Matrix4::new_perspective( // To remove distrotion
                            new_size.width as f32 / new_size.height as f32,
                            45.0, // POV
                            0.1,
                            100.0
                        );
                    }

                    // Handling keyboard input
                    WindowEvent::KeyboardInput {
                        input: KeyboardInput { virtual_keycode: Some(virtual_code), state, .. },
                        ..
                    } => match (virtual_code, state) {
                        (VirtualKeyCode::Escape, ElementState::Pressed) => {
                             *control_flow = ControlFlow::Exit;
                        }
                        (VirtualKeyCode::X, ElementState::Pressed) => glPolygonMode(GL_FRONT_AND_BACK, GL_LINE),
                        (VirtualKeyCode::X, ElementState::Released) => glPolygonMode(GL_FRONT_AND_BACK, GL_FILL),

                        (VirtualKeyCode::W, ElementState::Pressed) => camera_pos += camera_speed * camera_front,
                        (VirtualKeyCode::S, ElementState::Pressed) => camera_pos -= camera_speed * camera_front,
                        (VirtualKeyCode::A, ElementState::Pressed) => camera_pos -= cross_v3f32(camera_front, camera_up).normalize() * camera_speed,
                        (VirtualKeyCode::D, ElementState::Pressed) => camera_pos += cross_v3f32(camera_front, camera_up).normalize() * camera_speed,

                        _ => ()
                    },

                    /*WindowEvent::AxisMotion { axis, value, ..} => match axis {
                        _ => println!("ax: {}, val: {}", axis, value)//value as f32,
                    }*/
                    WindowEvent::CursorMoved { position, .. } => {
                        pitch += (last_y - position.y as f32) * 0.8;
                        yaw += (position.x as f32 - last_x) * 0.8;
                        if pitch > 89.0 {
                            pitch =  89.0;
                        }
                        if pitch < -89.0 {
                            pitch = -89.0;
                        }
                        last_x = position.x as f32;
                        last_y = position.y as f32;
                    }

                    _ => ()
                }
                

                //
                // RENDERING / DRAWING
                //

                Event::RedrawEventsCleared => {
                    // Clear buffers
                    glEnable(GL_DEPTH_TEST); 
                    glClear(GL_COLOR_BUFFER_BIT | GL_DEPTH_BUFFER_BIT);                    
                    
                    shader_program.use_shader();

                    context.resize(window_size);


                    //
                    // TEXTURES
                    //

                    shader_program.set_int("texture1", 0);
                    //shader_program.set_int("texture2", 1);
                    
                    
                    //
                    // CAMERA
                    //

                    let current_frame = t0.elapsed().as_secs_f32();
                    let delta_time = current_frame - last_frame;
                    last_frame = current_frame;

                    camera_speed = 5.0 * delta_time;

                    direction.x = radians(yaw).cos() * radians(pitch).cos();
                    direction.y = radians(pitch).sin();
                    direction.z = radians(yaw).sin() * radians(pitch).cos();

                    camera_front = direction.normalize();
                    
                    //
                    // FPS
                    //
                    
                    let fps = (1.0 / delta_time) * 0.05 + last_fps * 0.95; // Smooth values
                    let title = String::from(TITLE) + " ~ FPS: " + (fps as u32).to_string().as_str();
                    context.window().set_title(title.as_str());
                    last_fps = fps; 


                    let cam_view = Matrix4::look_at_rh(
                        &camera_pos,//&Point3::new(cam_x, 0.0, cam_z),
                        &(camera_pos + camera_front),//&Point3::new(0.0, 0.0, 0.0),
                        &camera_up//&Vector3::new(0.0, 1.0, 0.0)
                    );

                    
                    //
                    // TRANSFORMATION
                    //

                    let rotation = Rotation3::from_axis_angle(
                        &Vector3::x_axis(),
                        0.0//20.0 * (std::f32::consts::PI / 180.0) // 20° to rad
                    ).to_homogeneous();
                   
                    // Scaling
                    let scale = Matrix4::new_scaling(1.0);

                    // Translation
                    let translation = Matrix4::new_translation(&Vector::from([0.0, 0.0, 0.0]));

                    // Together
                    let model = translation * scale * rotation;
                    let view = cam_view;
                    let projection = projection_matrix;
                    let final_transformation = projection * view * model;
                    
                    let transform_location = glGetUniformLocation(shader_program.id, "transform\0".as_ptr());
                    if transform_location == -1 { panic!("Transform uniform not found!") }
                    glUniformMatrix4fv(transform_location, 1, 0, final_transformation.as_ptr());


                    //
                    // DRAWING
                    //

                    glBindBuffer(GL_ELEMENT_ARRAY_BUFFER, ebo);

                    let locations: [Vector3::<f32>; 10] = [
                        Vector3::new( 0.0,  0.0,  0.0), 
                        Vector3::new( 2.0,  5.0, -15.0), 
                        Vector3::new(-1.5, -2.2, -2.5),  
                        Vector3::new(-3.8, -2.0, -12.3),
                        Vector3::new( 2.4, -0.4, -3.5),  
                        Vector3::new(-1.7,  3.0, -7.5),  
                        Vector3::new( 1.3, -2.0, -2.5),  
                        Vector3::new( 1.5,  2.0, -2.5), 
                        Vector3::new( 1.5,  0.2, -1.5), 
                        Vector3::new(-1.3,  1.0, -1.5)  
                    ];

                    let transform_location = glGetUniformLocation(shader_program.id, "offset\0".as_ptr());
                        if transform_location == -1 { panic!("Offset uniform not found!") }

                    for loc in locations.iter() {
                        glUniformMatrix4fv(transform_location, 1, 0,  Matrix4::new_translation(loc).as_ptr());

                        glDrawElements(
                            GL_TRIANGLES,         // Drawing mode
                            indices.len() as i32, // Number of elements
                            GL_UNSIGNED_INT,      // Type of indices
                            0 as *const _         // Offset
                        );
                    }

                    
                    //glDrawArrays(GL_TRIANGLES, 0, vertices.len() as i32);



                    // ... and finally swap the buffers
                    context.swap_buffers().unwrap();
                }
                _ => {}
            }
        });
    }
}
