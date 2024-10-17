use read_input::prelude::*;
use std::env;

fn main() {
    let args: Vec<String> = env::args().collect();
    let choice_solid = &args[1];
    if choice_solid == "cyvol"{
    cyvol();
}else if choice_solid == "sphvol"{
    sphvol();
}else if choice_solid == "conevol"{
    conevol();
}else if choice_solid == "cubevol"{
    cubevol();
}else if choice_solid == "prisvol"{
    prisvol();
}else if choice_solid == "rectvol"{
    rectvol();
}else if choice_solid == "pyravol"{
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
    println!("AND NOW, THE RADIUS OF THE SPHERE /(in mm)/, PLEASE: ");
    let r = input::<f32>().get();
    let volume = 4.0*pi*(r.powf(3.0))/3.0;
    println!("THE SPHERE HAS {}in³.", volume);
    }
fn conevol(){
    println!("----VOLUME OF A CONE----");
    let pi = std::f32::consts::PI;
    println!("PLEASE INPUT THE HEIGHT OF CONE/(in mm)/, PLEASE: ");
    let h = input::<f32>().get();
    println!("AND NOW, THE RADIUS OF THE CONE/(in mm)/, PLEASE: ");
    let r = input::<f32>().get();
    let volume = pi * r.powf(2.0) * h/3.0;
    println!("THE CONE HAS {}mm³.", volume);
    }
fn pyravol(){
    println!("----VOLUME OF A PYRAMID----");
    println!("PLEASE INPUT THE HEIGHT OF PYRAMID/(in mm)/, PLEASE: ");
    let h = input::<f32>().get();
    println!("AND NOW, THE BASE AREA OF THE PYRAMID/(in mm)/, PLEASE: ");
    let ab = input::<f32>().get();
    let volume = ab * h/3.0;
    println!("THE PYRAMID HAS {}mm³.", volume);
    }
fn prisvol(){
    println!("----VOLUME OF A PRISM----");
    println!("PLEASE INPUT THE HEIGHT OF PRISM/(in mm)/, PLEASE: ");
    let h = input::<f32>().get();
    println!("AND NOW, THE BASE AREA OF THE PRISM/(in mm)/, PLEASE: ");
    let ab = input::<f32>().get();
    let volume = ab * h;
    println!("THE PRISM HAS {}mm³.", volume);
    }
fn cubevol(){
    println!("----VOLUME OF A CUBE----");
    println!("PLEASE INPUT THE AREA OF CUBE/(in mm)/, PLEASE: ");
    let a = input::<f32>().get();
    let volume = a.powf(3.0);
    println!("THE CUBE HAS {}mm³.", volume);
    }
fn rectvol(){
    println!("----VOLUME OF A RECTANGULAR PARALELEPIPED---");
    println!("PLEASE INPUT THE LENGTH OF RECTANGULAR PARALELEPIPED/(in mm)/, PLEASE: ");
    let l = input::<f32>().get();
    println!("PLEASE INPUT THE WEIGHT OF RECTANGULAR PARALELEPIPED/(in mm)/, PLEASE: ");
    let w = input::<f32>().get();
    println!("AND NOW, THE BASE HEIGHT OF THE RECTANGULAR PARALELEPIPED/(in mm)/, PLEASE: ");
    let h = input::<f32>().get();
    let volume = l * w * h;
    println!("THE RECTANGULAR PARALELEPIPED HAS {}mm³.", volume);
    }
