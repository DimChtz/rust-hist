pub struct Hist {

    w:i32,
    h:i32,
    x:Vec<i32>,
    y:Vec<i32>,

}

impl Hist {

    // Function to initialize a Hist object
    pub fn new(_w:i32, _h:i32, _x:&Vec<i32>, _y:&Vec<i32>) -> Hist {

        assert!(_x.len() > 0 && _y.len() > 0, "Both len(x) and len(y) should be non-zero.");
        assert!(_x.len() == _y.len(), "len(x) should be equal to len(y).");

        Hist {
            w:_w, 
            h:_h,
            x:_x.clone(),
            y:_y.clone()
        }

    }

    // Function to set the size (width, height) of the printed histogram
    pub fn set_size(&mut self, _w:i32, _h:i32) {

        self.w = _w;
        self.h = _h;

    }

    // Function to set the width of the printed histogram
    pub fn set_width(&mut self, _w:i32) {

        self.w = _w;

    }

    // Function to set the height of the printed histogram
    pub fn set_height(&mut self, _h:i32) {

        self.h = _h;

    }

    // Function to get the size (width, height) of the printed histogram
    pub fn get_size(&self) -> (i32, i32) {

        (self.w, self.h)

    }

    // Function to get the width of the printed histogram
    pub fn get_width(&self) -> i32 {

        self.w

    }

    // Function to get the height of the printed histogram
    pub fn get_height(&self) -> i32 {

        self.h
        
    }

    // Function to get the min data value
    pub fn get_min(&self) -> i32 {

        self.y.iter().min().unwrap().clone()

    }

    // Function to get the max data value
    pub fn get_max(&self) -> i32 {

        self.y.iter().max().unwrap().clone() 

    }

    pub fn get_mean(&self) -> i32 {

        self.y.iter().fold(0i32, |sum, val| sum + val) / self.y.len() as i32

    }

    // Function to normalize the data
    fn calc(&self) -> (Vec<i32>, Vec<i32>) {
        
        let norm_value = |val:i32, min:i32, max:i32, new_min:i32, new_max:i32| -> i32 {
            new_min + ( (val - min) as f32 * ((new_max - new_min) as f32 / (max - min) as f32) as f32 ).ceil() as i32
        };

        let min_x = self.x.iter().min().unwrap().clone();
        let max_x = self.x.iter().max().unwrap().clone();
        let min_y = self.y.iter().min().unwrap().clone();
        let max_y = self.y.iter().max().unwrap().clone();

        (self.x.iter().map(|v| norm_value(*v, min_x, max_x, 0, self.w)).collect(),
         self.y.iter().map(|v| norm_value(*v, min_y, max_y, 0, self.h)).collect()) as (Vec<i32>, Vec<i32>)

    }

    // Function to print the histogram on the console
    pub fn display(&self) {

        // Normalize Data

        let (local_x, local_y) = self.calc();

        // Display histogram

        for row in 0..self.h + 1 {

            for col in 0..self.w + 1 {

                match local_x.binary_search(&col) {

                    Ok(pos) => print!("{}", if (self.h - row) <= local_y[pos] {"*"} else {" "}),

                    Err(pos) => print!("{}", if row == self.h {"*"} else {" "}),

                };

            }

            println!("");

        }

    }

}
