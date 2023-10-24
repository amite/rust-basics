#[derive(Debug)]
pub struct Item {
    what: String,
    present: bool,
}

#[derive(Debug)]
pub struct Hands {
    left: Item,
    right: Item,
}

impl Hands {
    pub fn new() -> Self {
        Self {
            left: Item {
                what: "an apple".to_owned(),
                present: true,
            },
            right: Item {
                what: "a banana".to_owned(),
                present: true,
            },
        }
    }

    pub fn report(&self) {
        Item::report_item(&self.left, "left");
        Item::report_item(&self.right, "right");
    }

    pub fn juggle(mut self) -> Self {
        let air = self.left;
        self.left = self.right;
        self.right = air;
        self
    }
}

impl Item {
    fn report_item(&self, which: &str) {
        if self.present {
            println!("The {} is  is holding {}", which, self.what);
        } else {
            println!("the {} hand is {} holding anything", which, "not");
        }
    }

    
}
