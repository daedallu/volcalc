pub fn default_(x:f32 ) -> f32 /*String*/{
        let res = x * 1.0;
        //return res.to_string();
        return res;
}

pub fn to_inch(x:f32 ) -> f32{
        let res = x * 0.393701;
        return res
}

pub fn to_mm(x:f32) -> f32{
        let res = x * 10.0;
        return res
}

pub fn to_feet(x:f32) -> f32{
        let res = x * 0.03280841;
        return res
}
pub fn to_meter(x:f32) -> f32{
        let res = x * 0.01;
        return res
}

