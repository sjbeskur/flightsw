
#[allow(dead_code)]

#[derive(Debug,  Clone)]
pub struct Satellite{
    coords: u32,
}

impl Satellite{

    pub fn new() -> Satellite{
        Self{
            //coords: vec![ 1.2, 2.5, 5.5 ],
            coords: 12,
        }
    }
    
    pub fn get_coordinates(&mut self) -> u32{
        self.coords = 12;
        //self.coords = vec![ 1.2, 2.5, 5.5 ];
        let coords = self.coords.clone();
        coords
    }

    pub fn launch_app(){

    }

}

trait SatalliteApp{
    
}
