use std::fmt;

#[derive(Debug)]
struct House {
    walls: i32,
    doors: i32,
    windows: i32,
}
#[derive(Debug)]
struct HouseBuilder {
    walls: i32,
    doors: i32,
    windows: i32,
    status: u8,
}

#[derive(Debug, Clone)]
struct HouseBuilderError;

impl fmt::Display for HouseBuilderError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "HouseBuilder not ready")
    }
}

impl HouseBuilder {
    pub fn new() -> HouseBuilder {
        HouseBuilder {
            walls: 0,
            doors: 0,
            windows: 0,
            status: 0b0000_0000,
        }
    }

    pub fn walls(&mut self, walls: i32) -> &mut Self {
        self.walls = walls;
        self.status = self.status | 0b0000_0001;
        self
    }
    pub fn doors(&mut self, doors: i32) -> &mut Self {
        self.doors = doors;
        self.status = self.status | 0b0000_0010;
        self
    }
    pub fn windows(&mut self, windows: i32) -> &mut Self {
        self.windows = windows;
        self.status = self.status | 0b0000_0100;
        self
    }

    pub fn build(&self) -> Result<House, HouseBuilderError> {
        if self.status != 0b0000_0111 {
            return Err(HouseBuilderError);
        }
        Ok(House {
            walls: self.walls,
            doors: self.doors,
            windows: self.windows,
        })
    }
}

fn main() {
    let house1 = HouseBuilder::new().doors(2).windows(4).walls(1).build();
    assert!(house1.is_ok());
    if let Ok(house1) = house1 {
        println!("house1 {:#?}", house1);
        assert_eq!(2, house1.doors);
        assert_eq!(4, house1.windows);
        assert_eq!(1, house1.walls);
    }
    let house2 = HouseBuilder::new().windows(4).walls(3).doors(1).build();
    assert!(house2.is_ok());
    if let Ok(house2) = house2 {
        println!("house2 {:#?}", house2);
        assert_eq!(1, house2.doors);
        assert_eq!(4, house2.windows);
        assert_eq!(3, house2.walls);
    }
    let house3 = HouseBuilder::new().windows(4).build();
    assert!(house3.is_err());
    if let Err(e) = house3 {
        println!("house3 {:#?}", e);
    }
}
