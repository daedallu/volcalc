use read_input::prelude::*;
use std::env;
mod forma;
mod centimeter;

fn main() {
    let args: Vec<String> = env::args().collect();
    let choice_solid = &args[1];
    let choice_unity = &args[2];

// Cyllinder

	if choice_solid == "cy" && choice_unity == "-cm"{
        println!("----VOLUME OF A CYLLINDER /( insert values in cm, please)/----");
		let res = centimeter::default_(forma::cyvol());
		println!("{} cm³", res);
		}
    else if choice_solid == "cy" && choice_unity == "-i"{
        println!("----VOLUME OF A CYLLINDER /( insert values in inch, please)/----");
		let res = centimeter::default_(forma::cyvol());
		println!("{} Inch³", res);
		}
    else if choice_solid == "cy" && choice_unity == "-f"{
        println!("----VOLUME OF A CYLLINDER /( insert values in feet, please)/----");
		let res = centimeter::default_(forma::cyvol());
		println!("{}ft³", res);
		}
    else if choice_solid == "cy" && choice_unity == "-m"{
        println!("----VOLUME OF A CYLLINDER /( insert values in meters, please)/----");
		let res = centimeter::default_(forma::cyvol());
		println!("{} m³", res);
		}
     else if choice_solid == "cy" && choice_unity == "-mm"{
        println!("----VOLUME OF A CYLLINDER /( insert values in mm, please)/----");
		let res = centimeter::default_(forma::cyvol());
		println!("{} mm³", res);
		}

// Sphere

    else if choice_solid == "sp" && choice_unity == "-cm"{
        println!("----VOLUME OF A SPHERE /( insert values in cm, please)/----");
		let res = centimeter::default_(forma::sphvol());
		println!("{} cm³", res);
		}
    else if choice_solid == "sp" && choice_unity == "-i"{
        println!("----VOLUME OF A SPHERE /( insert values in inch, please)/----");
		let res = centimeter::default_(forma::sphvol());
		println!("{} Inch³", res);
		}
    else if choice_solid == "sp" && choice_unity == "-f"{
        println!("----VOLUME OF A SPHERE /( insert values in feet, please)/----");
		let res = centimeter::default_(forma::sphvol());
		println!("{}ft³", res);
		}
    else if choice_solid == "sp" && choice_unity == "-m"{
        println!("----VOLUME OF A SPHERE /( insert values in meters, please)/----");
		let res = centimeter::default_(forma::sphvol());
		println!("{} m³", res);
		}
     else if choice_solid == "sp" && choice_unity == "-mm"{
        println!("----VOLUME OF A SPHERE /( insert values in mm, please)/----");
		let res = centimeter::default_(forma::sphvol());
		println!("{} mm³", res);
		}

// Cone

    else if choice_solid == "co" && choice_unity == "-cm"{
        println!("----VOLUME OF A CONE/( insert values in cm, please)/----");
		let res = centimeter::default_(forma::conevol());
		println!("{} cm³", res);
		}
    else if choice_solid == "co" && choice_unity == "-i"{
        println!("----VOLUME OF A CONE /( insert values in inch, please)/----");
		let res = centimeter::default_(forma::conevol());
		println!("{} Inch³", res);
		}
    else if choice_solid == "co" && choice_unity == "-f"{
        println!("----VOLUME OF A CONE /( insert values in feet, please)/----");
		let res = centimeter::default_(forma::conevol());
		println!("{}ft³", res);
		}
    else if choice_solid == "co" && choice_unity == "-m"{
        println!("----VOLUME OF A CONE /( insert values in meters, please)/----");
		let res = centimeter::default_(forma::conevol());
		println!("{} m³", res);
		}
     else if choice_solid == "co" && choice_unity == "-mm"{
        println!("----VOLUME OF A CONE /( insert values in mm, please)/----");
		let res = centimeter::default_(forma::conevol());
		println!("{} mm³", res);
		}

// Pyramid

    else if choice_solid == "py" && choice_unity == "-cm"{
        println!("----VOLUME OF A PYRAMID /( insert values in cm, please)/----");
		let res = centimeter::default_(forma::pyravol());
		println!("{} cm³", res);
		}
    else if choice_solid == "py" && choice_unity == "-i"{
        println!("----VOLUME OF A PYRAMID /( insert values in inch, please)/----");
		let res = centimeter::default_(forma::pyravol());
		println!("{} Inch³", res);
		}
    else if choice_solid == "py" && choice_unity == "-f"{
        println!("----VOLUME OF A PYRAMID /( insert values in feet, please)/----");
		let res = centimeter::default_(forma::pyravol());
		println!("{}ft³", res);
		}
    else if choice_solid == "py" && choice_unity == "-m"{
        println!("----VOLUME OF A PYRAMID /( insert values in meters, please)/----");
		let res = centimeter::default_(forma::pyravol());
		println!("{} m³", res);
		}
     else if choice_solid == "py" && choice_unity == "-mm"{
        println!("----VOLUME OF A PYRAMID /( insert values in mm, please)/----");
		let res = centimeter::default_(forma::pyravol());
		println!("{} mm³", res);
		}

// Prism

    else if choice_solid == "pr" && choice_unity == "-cm"{
        println!("----VOLUME OF A PRISM /( insert values in cm, please)/----");
		let res = centimeter::default_(forma::prisvol());
		println!("{} cm³", res);
		}
    else if choice_solid == "pr" && choice_unity == "-i"{
        println!("----VOLUME OF A PRISM/( insert values in inch, please)/----");
		let res = centimeter::default_(forma::prisvol());
		println!("{} Inch³", res);
		}
    else if choice_solid == "pr" && choice_unity == "-f"{
        println!("----VOLUME OF A PRISM /( insert values in feet, please)/----");
		let res = centimeter::default_(forma::prisvol());
		println!("{}ft³", res);
		}
    else if choice_solid == "pr" && choice_unity == "-m"{
        println!("----VOLUME OF A PRISM /( insert values in meters, please)/----");
		let res = centimeter::default_(forma::prisvol());
		println!("{} m³", res);
		}
     else if choice_solid == "pr" && choice_unity == "-mm"{
        println!("----VOLUME OF A PRISM /( insert values in mm, please)/----");
		let res = centimeter::default_(forma::prisvol());
		println!("{} mm³", res);
		}

// Cube

    else if choice_solid == "cb" && choice_unity == "-cm"{
        println!("----VOLUME OF A CUBE /( insert values in cm, please)/----");
		let res = centimeter::default_(forma::cubevol());
		println!("{} cm³", res);
		}
    else if choice_solid == "cb" && choice_unity == "-i"{
        println!("----VOLUME OF A CUBE /( insert values in inch, please)/----");
		let res = centimeter::default_(forma::cubevol());
		println!("{} Inch³", res);
		}
    else if choice_solid == "cb" && choice_unity == "-f"{
        println!("----VOLUME OF A CUBE /( insert values in feet, please)/----");
		let res = centimeter::default_(forma::cubevol());
		println!("{}ft³", res);
		}
    else if choice_solid == "cb" && choice_unity == "-m"{
        println!("----VOLUME OF A CUBE /( insert values in meters, please)/----");
		let res = centimeter::default_(forma::cubevol());
		println!("{} m³", res);
		}
     else if choice_solid == "cb" && choice_unity == "-mm"{
        println!("----VOLUME OF A CUBE /( insert values in mm, please)/----");
		let res = centimeter::default_(forma::cubevol());
		println!("{} mm³", res);
		}

// Rectangle

    else if choice_solid == "rec" && choice_unity == "-cm"{
        println!("----VOLUME OF A RECTANGLE /( insert values in cm, please)/----");
		let res = centimeter::default_(forma::rectvol());
		println!("{} cm³", res);
		}
    else if choice_solid == "rec" && choice_unity == "-i"{
        println!("----VOLUME OF A RECTANGLE /( insert values in inch, please)/----");
		let res = centimeter::default_(forma::rectvol());
		println!("{} Inch³", res);
		}
    else if choice_solid == "rec" && choice_unity == "-f"{
        println!("----VOLUME OF A RECTANGLE /( insert values in feet, please)/----");
		let res = centimeter::default_(forma::rectvol());
		println!("{}ft³", res);
		}
    else if choice_solid == "rec" && choice_unity == "-m"{
        println!("----VOLUME OF A RECTANGLE /( insert values in meters, please)/----");
		let res = centimeter::default_(forma::rectvol());
		println!("{} m³", res);
		}
     else if choice_solid == "rec" && choice_unity == "-mm"{
        println!("----VOLUME OF A RECTANGLE /( insert values in mm, please)/----");
		let res = centimeter::default_(forma::rectvol());
		println!("{} mm³", res);
		}

// from cm to other measurement unities
// Cyllinder

	if choice_solid == "cy" && choice_unity == "--cm"{
        println!("----VOLUME OF A CYLLINDER /( insert values in cm, please)/----");
		let res = centimeter::default_(forma::cyvol());
		println!("{} cm³", res);
		}
    else if choice_solid == "cy" && choice_unity == "--i"{
        println!("----VOLUME OF A CYLLINDER /( insert values in cm, please)/----");
		let res = centimeter::to_inch(forma::cyvol());
		println!("{} Inch³", res);
		}
    else if choice_solid == "cy" && choice_unity == "--f"{
        println!("----VOLUME OF A CYLLINDER /( insert values in cm, please)/----");
		let res = centimeter::to_feet(forma::cyvol());
		println!("{}ft³", res);
		}
    else if choice_solid == "cy" && choice_unity == "--m"{
        println!("----VOLUME OF A CYLLINDER /( insert values in cm, please)/----");
		let res = centimeter::to_meter(forma::cyvol());
		println!("{} m³", res);
		}
     else if choice_solid == "cy" && choice_unity == "--mm"{
        println!("----VOLUME OF A CYLLINDER /( insert values in cm, please)/----");
		let res = centimeter::to_mm(forma::cyvol());
		println!("{} mm³", res);
		}

// Sphere

    else if choice_solid == "sp" && choice_unity == "--cm"{
        println!("----VOLUME OF A SPHERE /( insert values in cm, please)/----");
		let res = centimeter::default_(forma::sphvol());
		println!("{} cm³", res);
		}
    else if choice_solid == "sp" && choice_unity == "--i"{
        println!("----VOLUME OF A SPHERE /( insert values in cm, please)/----");
		let res = centimeter::to_inch(forma::sphvol());
		println!("{} Inch³", res);
		}
    else if choice_solid == "sp" && choice_unity == "--f"{
        println!("----VOLUME OF A SPHERE /( insert values in cm, please)/----");
		let res = centimeter::to_feet(forma::sphvol());
		println!("{}ft³", res);
		}
    else if choice_solid == "sp" && choice_unity == "--m"{
        println!("----VOLUME OF A SPHERE /( insert values in cm, please)/----");
		let res = centimeter::to_meter(forma::sphvol());
		println!("{} m³", res);
		}
     else if choice_solid == "sp" && choice_unity == "--mm"{
        println!("----VOLUME OF A SPHERE /( insert values in cm, please)/----");
		let res = centimeter::to_mm(forma::sphvol());
		println!("{} mm³", res);
		}

// Cone

    else if choice_solid == "co" && choice_unity == "--cm"{
        println!("----VOLUME OF A CONE/( insert values in cm, please)/----");
		let res = centimeter::default_(forma::conevol());
		println!("{} cm³", res);
		}
    else if choice_solid == "co" && choice_unity == "--i"{
        println!("----VOLUME OF A CONE /( insert values in cm, please)/----");
		let res = centimeter::to_inch(forma::conevol());
		println!("{} Inch³", res);
		}
    else if choice_solid == "co" && choice_unity == "--f"{
        println!("----VOLUME OF A CONE /( insert values in cm, please)/----");
		let res = centimeter::to_feet(forma::conevol());
		println!("{}ft³", res);
		}
    else if choice_solid == "co" && choice_unity == "--m"{
        println!("----VOLUME OF A CONE /( insert values in cm, please)/----");
		let res = centimeter::to_meter(forma::conevol());
		println!("{} m³", res);
		}
     else if choice_solid == "co" && choice_unity == "--mm"{
        println!("----VOLUME OF A CONE /( insert values in cm, please)/----");
		let res = centimeter::to_mm(forma::conevol());
		println!("{} mm³", res);
		}

// Pyramid

    else if choice_solid == "py" && choice_unity == "--cm"{
        println!("----VOLUME OF A PYRAMID /( insert values in cm, please)/----");
		let res = centimeter::default_(forma::pyravol());
		println!("{} cm³", res);
		}
    else if choice_solid == "py" && choice_unity == "--i"{
        println!("----VOLUME OF A PYRAMID /( insert values in cm, please)/----");
		let res = centimeter::to_inch(forma::pyravol());
		println!("{} Inch³", res);
		}
    else if choice_solid == "py" && choice_unity == "--f"{
        println!("----VOLUME OF A PYRAMID /( insert values in cm, please)/----");
		let res = centimeter::to_feet(forma::pyravol());
		println!("{}ft³", res);
		}
    else if choice_solid == "py" && choice_unity == "--m"{
        println!("----VOLUME OF A PYRAMID /( insert values in cm, please)/----");
		let res = centimeter::to_meter(forma::pyravol());
		println!("{} m³", res);
		}
     else if choice_solid == "py" && choice_unity == "--mm"{
        println!("----VOLUME OF A PYRAMID /( insert values in cm, please)/----");
		let res = centimeter::to_mm(forma::pyravol());
		println!("{} mm³", res);
		}

// Prism

    else if choice_solid == "pr" && choice_unity == "--cm"{
        println!("----VOLUME OF A PRISM /( insert values in cm, please)/----");
		let res = centimeter::default_(forma::prisvol());
		println!("{} cm³", res);
		}
    else if choice_solid == "pr" && choice_unity == "--i"{
        println!("----VOLUME OF A PRISM /( insert values in cm, please)/----");
		let res = centimeter::to_inch(forma::prisvol());
		println!("{} Inch³", res);
		}
    else if choice_solid == "pr" && choice_unity == "--f"{
        println!("----VOLUME OF A PRISM /( insert values in cm, please)/----");
		let res = centimeter::to_feet(forma::prisvol());
		println!("{}ft³", res);
		}
    else if choice_solid == "pr" && choice_unity == "--m"{
        println!("----VOLUME OF A PRISM /( insert values in cm, please)/----");
		let res = centimeter::to_meter(forma::prisvol());
		println!("{} m³", res);
		}
     else if choice_solid == "pr" && choice_unity == "--mm"{
        println!("----VOLUME OF A PRISM /( insert values in cm, please)/----");
		let res = centimeter::to_mm(forma::prisvol());
		println!("{} mm³", res);
		}

// Cube

    else if choice_solid == "cb" && choice_unity == "--cm"{
        println!("----VOLUME OF A CUBE /( insert values in cm, please)/----");
		let res = centimeter::default_(forma::cubevol());
		println!("{} cm³", res);
		}
    else if choice_solid == "cb" && choice_unity == "--i"{
        println!("----VOLUME OF A CUBE /( insert values in cm, please)/----");
		let res = centimeter::to_inch(forma::cubevol());
		println!("{} Inch³", res);
		}
    else if choice_solid == "cb" && choice_unity == "--f"{
        println!("----VOLUME OF A CUBE /( insert values in cm, please)/----");
		let res = centimeter::to_feet(forma::cubevol());
		println!("{}ft³", res);
		}
    else if choice_solid == "cb" && choice_unity == "--m"{
        println!("----VOLUME OF A CUBE /( insert values in cm, please)/----");
		let res = centimeter::to_meter(forma::cubevol());
		println!("{} m³", res);
		}
     else if choice_solid == "cb" && choice_unity == "--mm"{
        println!("----VOLUME OF A CUBE /( insert values in cm, please)/----");
		let res = centimeter::to_mm(forma::cubevol());
		println!("{} mm³", res);
		}

// Rectangle

    else if choice_solid == "rec" && choice_unity == "--cm"{
        println!("----VOLUME OF A RECTANGLE /( insert values in cm, please)/----");
		let res = centimeter::default_(forma::rectvol());
		println!("{} cm³", res);
		}
    else if choice_solid == "rec" && choice_unity == "--i"{
        println!("----VOLUME OF A RECTANGLE /( insert values in cm, please)/----");
		let res = centimeter::to_inch(forma::rectvol());
		println!("{} Inch³", res);
		}
    else if choice_solid == "rec" && choice_unity == "--f"{
        println!("----VOLUME OF A RECTANGLE /( insert values in cm, please)/----");
		let res = centimeter::to_feet(forma::rectvol());
		println!("{}ft³", res);
		}
    else if choice_solid == "rec" && choice_unity == "--m"{
        println!("----VOLUME OF A RECTANGLE /( insert values in cm, please)/----");
		let res = centimeter::to_meter(forma::rectvol());
		println!("{} m³", res);
		}
     else if choice_solid == "rec" && choice_unity == "--mm"{
        println!("----VOLUME OF A RECTANGLE /( insert values in cm, please)/----");
		let res = centimeter::to_mm(forma::rectvol());
		println!("{} mm³", res);
		}

}
