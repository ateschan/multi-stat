use std::net::TcpStream;
use std::io::{Read};
use macroquad::prelude::*;
use std::thread;
use std::sync::{Arc, Mutex};

#[macroquad::main("Graphically Represented Performance")]

async fn main(){
    set_fullscreen(true);
    show_mouse(false);
    let mut stream = TcpStream::connect("192.168.2.3:7878")
    .expect("Failed to connect");
    
    let mut query_vec = Vec::new();
    let mut last_RAM = (0.0, 0.0);
    let mut last_CPU = (0.0, 0.0);
    let mut last_GPU = (0.0, screen_height()/3.0);

    loop {
        clear_background(GRAY);

        draw_text("GPU", 20.0, screen_height()/6.0, 30.0, DARKGRAY);
        draw_text("RAM", 20.0, screen_height()/2.0, 30.0, DARKGRAY);
        draw_text("CPU", 20.0, screen_height()/6.0 * 5.0, 30.0, DARKGRAY);

        let (grid_size, step) = (21, screen_width() / 21.0);
        let mut ct: i32 = 0;
        for i in 0..=grid_size {
            let pos = i as f32 * step;

            if ct == 5 || ct == 9 {
                draw_line(0.0, pos, screen_width(), pos, 2.0, BLACK);
            }
            draw_line(pos, 0.0, pos, screen_height(), 1.0, DARKGRAY);
            draw_line(0.0, pos, screen_width(), pos, 1.0, DARKGRAY);
            ct += 1;
        }


        let mouse_pos = mouse_position();
        let mut data_bytes = [0; 12]; // 3 * size_of::<f32>()
        stream.read_exact(&mut data_bytes).expect("Read failed");

        let data: [f32; 3] = unsafe {
            // Convert the slice of u8 to a slice of f32
            *(data_bytes.as_ptr() as *const [u8; 12] as *const [f32; 3])
        };
        
        println!("Received data: {:?}", data);
        
        if query_vec.len() < 22 {
            query_vec.push(data);
        }
        else{
            query_vec.remove(0);
            query_vec.push(data);
        }

        let mut queries: f32 = 0.0;

        if !query_vec.is_empty() {
            let mut previous_posGPU = last_GPU;
            let mut previous_posRAM = last_RAM;
            let mut previous_posCPU = last_CPU;
            
            for i in query_vec.iter() {
                let new_posGPU = (screen_width()/21.0 * queries, screen_height()/3.0 - i[0]);
                draw_line(previous_posGPU.0, previous_posGPU.1, new_posGPU.0, new_posGPU.1, 3.0, GREEN);
                draw_line(screen_width()/21.0 * queries, screen_height()/3.0, new_posGPU.0, new_posGPU.1, 3.0, GREEN);
                previous_posGPU = new_posGPU;
        
                let new_posRAM = (screen_width()/21.0 * queries, screen_height()/4.8 * 3.0 - i[1] * 0.5);
                draw_line(previous_posRAM.0, previous_posRAM.1, new_posRAM.0, new_posRAM.1,  3.0, RED);
                draw_line(screen_width()/21.0 * queries, screen_height()/4.8 * 3.0 , new_posRAM.0, new_posRAM.1,  3.0, RED);
                previous_posRAM = new_posRAM;
        
                let new_posCPU = (screen_width()/21.0 * queries, screen_height() - i[2] * 0.5);
                draw_line(previous_posCPU.0, previous_posCPU.1, new_posCPU.0, new_posCPU.1,  3.0, DARKBLUE);
                draw_line(screen_width()/21.0 * queries, screen_height(), new_posCPU.0, new_posCPU.1,  3.0, DARKBLUE);
                previous_posCPU = new_posCPU;
        
                if queries != 22.0 {
                    queries += 1.0;
                }
            }
        }

        next_frame().await;
    }
}
