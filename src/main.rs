use read_input::prelude::*;
use std::env;

fn main() {
    let args: Vec<String> = env::args().collect();
    let choice = &args[1];
    if choice == "cyvol"{
    cyvol();
}else if choice == "sphvol"{
    sphvol();
    }
   
    
    
}
fn cyvol(){
    println!("----VOLUME OF A CYLINDER----");
    let pi = std::f32::consts::PI;
    println!("PLEASE INPUT THE HEIGHT /(in mm)/, PLEASE: ");
    let h = input::<f32>().get();
    println!("AND NOW, THE RADIUS OF THE CYLINDER /(in mm)/, PLEASE: ");
    let r = input::<f32>().get();
    let volume = pi * r.powf(2.0) * h;
    println!("THE CYLINDER HAS {}mm³.", volume);
}
fn sphvol(){
    println!("----VOLUME OF A SPHERE----");
    let pi = std::f32::consts::PI;
    println!("AND NOW, THE RADIUS OF THE CYLINDER /(in mm)/, PLEASE: ");
    let r = input::<f32>().get();
    let volume = 4.0*pi*(r.powf(3.0))/3.0;
    println!("THE SPHERE HAS {}in³.", volume);
    }
