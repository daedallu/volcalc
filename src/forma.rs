use read_input::prelude::*;
    
    pub fn cyvol() ->f32 {
        let pi = std::f32::consts::PI;
        println!("AND NOW, THE RADIUS, PLEASE: ");
        let r = input::<f32>().get();
        println!("AND NOW, THE HEIGHT, PLEASE: ");
        let h = input::<f32>().get();
        let volume = pi*(r.powf(2.0))*h;
        return volume;

    }

	pub fn sphvol() -> f32 {
        println!("THE RADIUS, PLEASE: ");
        let pi = std::f32::consts::PI;
        let r = input::<f32>().get();
        let volume = 4.0*pi*(r.powf(3.0))/3.0;
        return volume;
    }
	pub fn conevol() -> f32 {
		let pi = std::f32::consts::PI;
		println!("PLEASE INPUT THE HEIGHT OF CONE, PLEASE: ");
		let h = input::<f32>().get();
		println!("AND NOW, THE RADIUS OF THE CONE, PLEASE: ");
		let r = input::<f32>().get();
		let volume = pi * r.powf(2.0) * h/3.0;
	    return volume;
		}
	pub fn pyravol() -> f32 {
		println!("PLEASE INPUT THE HEIGHT OF PYRAMID, PLEASE: ");
		let h = input::<f32>().get();
		println!("AND NOW, THE BASE AREA OF THE PYRAMID, PLEASE: ");
		let ab = input::<f32>().get();
		let volume = ab * h/3.0;
		return volume;
		}
	pub fn prisvol() -> f32 {
		println!("PLEASE INPUT THE HEIGHT OF PRISM/,PLEASE: ");
		let h = input::<f32>().get();
		println!("AND NOW, THE BASE AREA OF THE PRISM, PLEASE: ");
		let ab = input::<f32>().get();
		let volume = ab * h;
		return volume;
		}
	pub fn cubevol() -> f32 {
		println!("PLEASE INPUT THE AREA OF CUBE, PLEASE: ");
		let a = input::<f32>().get();
		let volume = a.powf(3.0);
		return volume;
		}
	pub fn rectvol() -> f32 {
		println!("PLEASE INPUT THE LENGTH OF RECTANGULAR PARALELEPIPED, PLEASE: ");
		let l = input::<f32>().get();
		println!("PLEASE INPUT THE WEIGHT OF RECTANGULAR PARALELEPIPED, PLEASE: ");
		let w = input::<f32>().get();
		println!("AND NOW, THE BASE HEIGHT OF THE RECTANGULAR PARALELEPIPED, PLEASE: ");
		let h = input::<f32>().get();
		let volume = l * w * h;
		return volume;
		}


