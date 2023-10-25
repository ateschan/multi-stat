#![windows_subsystem = "windows"]
use std::net::{TcpStream, TcpListener};
use std::io::Write;
use std::thread;
use std::time::Duration;
use nvml_wrapper::error::NvmlError;
use nvml_wrapper::Nvml;
use sysinfo::{CpuExt, System, SystemExt};

fn main() -> std::io::Result<()> {

    let listener = TcpListener::bind("0.0.0.0:PORT BOUND")?;
    //bind a dog to the port 7878 on the LOCAL MACHINE
    for stream in listener.incoming() {
        let stream = stream?; 
        // Convert the incoming connection into a TcpStream
        // Handle the stream in a separate thread or async task.

        thread::spawn(move || {
            //accounting for error handling, opens a thread upon a successful connection
            handle_client(stream).unwrap_or_else(|error| {
                eprintln!("Error handling client: {}", error);
            });
        });
    }
    
    Ok(())
}


fn handle_client(mut stream: TcpStream) -> std::io::Result<()> {

    fn compute_overall_percentage(numerator: u64, denominator: u64) -> Result<f32, String> {
        // Check if the denominator is zero
        if denominator == 0 {
            return Err(String::from("Division by zero"));
        }
        // Compute the overall percentage
        let overall_percentage = (numerator as f32 / denominator as f32) * 100.0;

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

    fn get_ram_usage() -> Result<f32, String> {
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


    // Sending code
    let mut data: [f32; 3] = [1.0, 0.0, 0.0];
    
    loop {
        match get_gpu_usage() {
            // matching the return type of the get gpu load function
            Ok(percentage) => {
                data[0] = percentage;
            }
            Err(err) => {
                eprintln!("Failed to get GPU load: {:?}", err);
            }
        }

        match get_ram_usage() {
            //print memory percentage
            Ok(percentage) => {
                data[1] = percentage;
            }
            Err(err) => println!("Error: {}", err),
        }

        match get_cpu_usage() {
            //print memory percentage
            Ok(percentage) => {
                data[2] = percentage;
            }
            Err(err) => println!("Error: {}", err),
        }



        let data_bytes: &[u8] = unsafe {
            // Convert the slice of f32 to a slice of u8
            std::slice::from_raw_parts(
                data.as_ptr() as *const u8,
                data.len() * std::mem::size_of::<f32>(),
            )
        };

        stream.write_all(data_bytes)?;
        
        thread::sleep(Duration::from_secs(1)); // Wait for 1 second
    }
}
