use macroquad::prelude::*;
use nvml_wrapper::error::NvmlError;
use nvml_wrapper::Nvml;
use sysinfo::{CpuExt, System, SystemExt};


#[macroquad::main("Graphically Represented Performance")]

async fn main() {
    let mut circle_vec = Vec::new();
    let mut circle_render = false;
    let mut call_timer = 0;

    fn compute_overall_percentage(numerator: u64, denominator: u64) -> Result<f64, String> {
        // Check if the denominator is zero
        if denominator == 0 {
            return Err(String::from("Division by zero"));
        }
        // Compute the overall percentage
        let overall_percentage = (numerator as f64 / denominator as f64) * 100.0;

        // Return the result
        Ok(overall_percentage)
    }

    fn get_gpu_usage() -> Result<f32, NvmlError> {
        // Initialize NVML
        let nvml = Nvml::init()?;
        // Get the first GPU
        let gpu = nvml.device_by_index(0)?;
        // Get GPU utilization
        let utilization = gpu.utilization_rates()?;
        // Return GPU load as a percentage
        Ok(utilization.gpu as f32)
    }

    fn get_ram_usage() -> Result<f64, String> {
        //init system
        let sys = System::new_all();
        //retrieve use and unused memory
        let used_memory = sys.used_memory();
        let unused_memory = sys.total_memory();

        //pass into percentage function
        compute_overall_percentage(used_memory, unused_memory)
    }

    fn get_cpu_usage() -> Result<f32, String> {
        let mut sys = System::new_all();
        sys.refresh_cpu(); // Refreshing CPU information.

        //CPU load data is queryd from all cores, then the mediean is taken out of it
        let mut count: i32 = 0;
        let mut usage: f32 = 0.0;
        for cpu in sys.cpus() {
            usage += cpu.cpu_usage();
            count += 1;
        }

        if usage == 0.0 {
            return Err(String::from("CPU usage logged as zero"));
        }
        Ok(usage / count as f32)
    }

    loop {
        let mouse_pos = mouse_position();
        clear_background(DARKGRAY);
        draw_circle(mouse_pos.0, mouse_pos.1, 15.0, RED);

        if call_timer > 300 {
            circle_render = true;
            circle_vec = Vec::new();
            match get_gpu_usage() {
                // matching the return type of the get gpu load function
                Ok(percentage) => {
                    println!("GPU Load: {:.2}%", percentage);
                    circle_vec.push((
                        (screen_width() / 4.0, screen_height() / 4.0),
                        (percentage as f32, GREEN),
                    ));
                }
                Err(err) => {
                    eprintln!("Failed to get GPU load: {:?}", err);
                }
            }

            match get_ram_usage() {
                //print memory percentage
                Ok(percentage) => {
                    println!("RAM Load: {:.2}%", percentage);
                    circle_vec.push((
                        (screen_width() / 4.0 * 3.0, screen_height() / 4.0),
                        (percentage as f32, RED),
                    ));
                }
                Err(err) => println!("Error: {}", err),
            }

            match get_cpu_usage() {
                //print memory percentage
                Ok(percentage) => {
                    println!("CPU Load: {:.2}%", percentage);
                    circle_vec.push((
                        (screen_width() / 2.0, screen_height() / 4.0 * 3.0),
                        (percentage as f32, BLUE),
                    ));
                }
                Err(err) => println!("Error: {}", err),
            }

            call_timer = 0;
        }

        // GAME CODE DEMO
        if is_mouse_button_down(MouseButton::Left) {
            circle_vec.push((mouse_pos.clone(), (10.0, RED)));
            circle_vec.push((mouse_pos.clone(), (10.0, RED)));
            circle_vec.push((mouse_pos.clone(), (10.0, RED)));
            circle_vec.push((mouse_pos.clone(), (10.0, RED)));
            circle_vec.push((mouse_pos.clone(), (10.0, RED)));
            circle_vec.push((mouse_pos.clone(), (10.0, RED)));
            circle_vec.push((mouse_pos.clone(), (10.0, RED)));
            circle_vec.push((mouse_pos.clone(), (10.0, RED)));
            circle_vec.push((mouse_pos.clone(), (10.0, RED)));
            circle_render = true;
        }

        if circle_render == true {
            for circle in &circle_vec {
                draw_circle(circle.0 .0, circle.0 .1, circle.1 .0, circle.1 .1);
            }
        }
        call_timer += 1;
        next_frame().await
    }
}
