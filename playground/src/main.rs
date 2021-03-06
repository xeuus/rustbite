#![allow(non_camel_case_types)]
extern crate rustbite;
extern crate gl;

use rustbite::{vec3, mat4, quat, app, shader, math};

use std::rc::Rc;
use std::cell::RefCell;

use std::mem;
use std::ptr;


fn main() {


    let mut model = Rc::new(RefCell::new(mat4::identify(1.0)));
    let mut view = Rc::new(RefCell::new(mat4::create_trs(vec3::zero(), quat::identify(), vec3::one())));
    let mut projection = Rc::new(RefCell::new(mat4::ortho_window(1.0, 1.0, -0.1, 200.0)));

    
    let mut i = Rc::new(RefCell::new(0.0_f32));

    let sim = Rc::new(RefCell::new(shader::new(b"
        #version 150

        in vec3 position;
        in vec4 color;
        in vec2 texcoord;

        out vec4 _color;
        out vec2 _texcoord;


        uniform mat4 projection;
        uniform mat4 view;
        uniform mat4 model;

        void main() {
            _color = color;
            _texcoord = texcoord;
            gl_Position = projection * view * model * vec4(position, 1.0);
        }
    \0",b"
        #version 150

        in vec4 _color;
        in vec2 _texcoord;

        out vec4 outColor;
        uniform sampler2D tex;

        void main()
        {
            outColor = texture(tex, _texcoord) * _color;
        }
    \0")));


    let init = Box::new(|| {

    });

    let create = Box::new(|| {
        let mut data = sim.borrow_mut();
        data.compile();


        unsafe{

            let vtx: [f32; 36] = [
                -0.5, 0.5, 0.0, 1.0, 1.0, 0.0, 1.0, 0.0, 0.0,
                0.5, 0.5, 0.0, 1.0, 0.0, 0.0, 1.0, 1.0, 0.0,
                0.5, -0.5, 0.0, 0.0, 1.0, 1.0, 0.5, 1.0, 1.0,
                -0.5, -0.5, 0.0, 1.0, 1.0, 1.0, 0.2, 0.0, 1.0
            ];

            let indc: [gl::types::GLuint; 6] = [
                0, 1, 2,
                2, 3, 0
            ];

            let mut vao = mem::uninitialized();
            gl::GenVertexArrays(1, &mut vao);
            gl::BindVertexArray(vao);


            let mut vbo = mem::uninitialized();
            gl::GenBuffers(1, &mut vbo);
            gl::BindBuffer(gl::ARRAY_BUFFER, vbo);
            gl::BufferData(gl::ARRAY_BUFFER, (vtx.len() * mem::size_of::<f32>()) as isize, vtx.as_ptr() as *const _, gl::STATIC_DRAW);


            let mut ebo = mem::uninitialized();
            gl::GenBuffers(1, &mut ebo);
            gl::BindBuffer(gl::ELEMENT_ARRAY_BUFFER, ebo);
            gl::BufferData(gl::ELEMENT_ARRAY_BUFFER, (indc.len() * mem::size_of::<gl::types::GLuint>()) as isize, indc.as_ptr() as *const _, gl::STATIC_DRAW);

            data.use_here();

            let pos_attrib = gl::GetAttribLocation(data.program, b"position\0".as_ptr() as *const _);
            gl::VertexAttribPointer(pos_attrib as gl::types::GLuint, 3, gl::FLOAT,  gl::FALSE, (9 * mem::size_of::<f32>()) as gl::types::GLsizei, ptr::null());
            gl::EnableVertexAttribArray(pos_attrib as gl::types::GLuint);

            let pos_attrib = gl::GetAttribLocation(data.program, b"color\0".as_ptr() as *const _);
            gl::VertexAttribPointer(pos_attrib as gl::types::GLuint, 4, gl::FLOAT,  gl::FALSE, (9 * mem::size_of::<f32>()) as gl::types::GLsizei, (3 * mem::size_of::<f32>()) as *const () as *const _);
            gl::EnableVertexAttribArray(pos_attrib as gl::types::GLuint);

            let coord_attrib = gl::GetAttribLocation(data.program, b"texcoord\0".as_ptr() as *const _);
            gl::VertexAttribPointer(coord_attrib as gl::types::GLuint, 2, gl::FLOAT,  gl::FALSE, (9 * mem::size_of::<f32>()) as gl::types::GLsizei, (7 * mem::size_of::<f32>()) as *const () as *const _);
            gl::EnableVertexAttribArray(coord_attrib as gl::types::GLuint);


            let mut texture = mem::uninitialized();

            gl::GenTextures(1, &mut texture);
            gl::ActiveTexture(gl::TEXTURE0);
            gl::BindTexture(gl::TEXTURE_2D, texture);
            
            let pixels: [f32; 12] = [
                0.0, 0.0, 0.0,   1.0, 1.0, 1.0,
                1.0, 1.0, 1.0,   0.0, 0.0, 0.0
            ];
            
            gl::TexImage2D(gl::TEXTURE_2D, 0, gl::RGB as i32, 2, 2, 0, gl::RGB, gl::FLOAT, pixels.as_ptr() as *const _);
            

            gl::Uniform1i(gl::GetUniformLocation(data.program, b"tex".as_ptr() as *const _), 0);

            gl::TexParameteri(gl::TEXTURE_2D, gl::TEXTURE_WRAP_S, gl::CLAMP_TO_EDGE as i32);
            gl::TexParameteri(gl::TEXTURE_2D, gl::TEXTURE_WRAP_T, gl::CLAMP_TO_EDGE as i32);
            gl::TexParameteri(gl::TEXTURE_2D, gl::TEXTURE_MIN_FILTER, gl::NEAREST as i32);
            gl::TexParameteri(gl::TEXTURE_2D, gl::TEXTURE_MAG_FILTER, gl::NEAREST as i32);





        }
    });

    let mut m = 0.0;
    let update = Box::new(|| {
        let mut data = sim.borrow_mut();
        let mut proj = projection.borrow_mut();
        let viw = view.borrow_mut();
        let mut mdl = model.borrow_mut();
            

        *mdl = mat4::create_rotation(quat::from_angle_axis(m, vec3::forward()));

        m = m + 1.0;
            
        unsafe {
        
            let proj_attrib = gl::GetUniformLocation(data.program, b"projection\0".as_ptr() as *const _);
            gl::UniformMatrix4fv(proj_attrib, 1, gl::FALSE, proj.source.as_ptr() as *const _);


            let view_attrib = gl::GetUniformLocation(data.program, b"view\0".as_ptr() as *const _);
            gl::UniformMatrix4fv(view_attrib, 1, gl::FALSE, viw.source.as_ptr() as *const _);


            let model_attrib = gl::GetUniformLocation(data.program, b"model\0".as_ptr() as *const _);
            gl::UniformMatrix4fv(model_attrib, 1, gl::FALSE, mdl.source.as_ptr() as *const _);


            gl::Enable(gl::BLEND);
            gl::BlendFunc(gl::SRC_ALPHA, gl::ONE_MINUS_SRC_ALPHA);

            gl::DrawElements(gl::TRIANGLES, 6, gl::UNSIGNED_INT, ptr::null());

        }
    });

    let mut x = app {
        init: init,
        create: create,
        update: update
    };
    x.run();

}



















/*
    let nw = time::now().tm_nsec;
    for i in 0..5000{
        let m = mat4::create_trs(vec3::zero(), quat::identify(), vec3::one());

        let mut m2= mat4::inverse(m);
        m2  = m2 * m;
        m2  = m2 * m;
        m2  = m2 * m;
        m2  = m2 * m;
        m2  = mat4::create_trs(vec3::zero(), quat::identify(), vec3::one());
        m2= mat4::inverse(m2);
    }
    let ok = (time::now().tm_nsec - nw) as f32 /1000000.0;
    println!("{}ms", ok);

    return;


/*
    let mut model: mat4;
    let view = mat4::create_trs(vec3::zero(), quat::identify(), vec3::one());
    let mut projection = mat4::ortho_window(2.0, sw / sh, -0.1, 200.0);

    */


    
    let mut mx: f32 = 0.0;
    let mut my: f32 = 0.0;

    let mut sw: f32 = 500.0;
    let mut sh: f32 = 500.0;


    use glium::{DisplayBuild, Surface};
    let display = glium::glutin::WindowBuilder::new()
        .with_dimensions(sw as u32, sh as u32)
        .with_multisampling(4)
        .with_title("rust-game")
        .with_vsync()
        .build_glium()
        .unwrap();




    #[derive(Copy, Clone)]
    pub struct vert {
        position: [f32; 3],
    }

    implement_vertex!(vert, position);


    let vertex_shader_src = r#"
        #version 140

        in vec3 position;

        uniform mat4 model;
        uniform mat4 view;
        uniform mat4 projection;

        void main() {
            gl_Position = projection * view * model * vec4(position, 1.0);
        }
    "#;

    let fragment_shader_src = r#"
        #version 140

        out vec4 color;

        void main() {
            color = vec4(1,1,0,0.5);
        }
    "#;

    let program =
        glium::Program::from_source(&display, vertex_shader_src, fragment_shader_src, None)
            .unwrap();


    let shape = vec![vert { position: [-1.0, -1.0, 0.0] },
                     vert { position: [1.0, 1.0, 0.0] },
                     vert { position: [-1.0, 1.0, 0.0] }];

    let vertex_buffer = glium::VertexBuffer::new(&display, &shape).unwrap();
    let indices = glium::index::NoIndices(glium::index::PrimitiveType::TriangleStrip);

    let mut t: f32 = 0.0f32;
    loop {
        let mut target = display.draw();
        target.clear_color_and_depth((t%1.0, 0.09, 0.2, 1.0), 1.0);


        let rat = 2.0 * sh / sw;

        let mvc = mat4::inverse(view) * vec3::new(mx / sw * 2.0 - 2.0, -my / sh * rat + rat, 0.0);

        model = mat4::create_trs(mvc, quat::from_angle_axis(t, vec3::forward()), vec3::one());


        let uniforms = uniform! {
            model: model.source,
            view: view.source,
            projection: projection.source,
        };



        let params = glium::DrawParameters {
            blend: glium::Blend::alpha_blending(),
            ..Default::default()
        };

        

        target
            .draw(&vertex_buffer, &indices, &program, &uniforms, &params)
            .unwrap();

            

        t = t + 0.1;
        target.finish().unwrap();

        for ev in display.poll_events() {
            match ev {
                glium::glutin::Event::Closed => return,
                glium::glutin::Event::Resized(w, h) => {
                    sw = w as f32 / 2.0;
                    sh = h as f32 / 2.0;
                    println!("{}, {}", sw, sh);
                    projection = mat4::ortho_window(2.0, sw / sh, -0.1, 200.0);
                }
                glium::glutin::Event::MouseMoved(x, y) => {
                    mx = x as f32;
                    my = y as f32;
                }
                _ => (),
            }
        }
    }


    */