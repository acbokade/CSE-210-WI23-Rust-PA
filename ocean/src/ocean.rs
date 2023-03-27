use crate::beach::Beach;
use crate::prey::{Algae, Clam, Minnow, Shrimp};
use crate::reef::Reef;
use std::cell::RefCell;
// use std::ops::Deref;
use std::rc::Rc;
use std::slice::Iter;

#[derive(Debug)]
pub struct Ocean {
    // TODO: Fill in fields here.
    pub reefs: Vec<Rc<RefCell<Reef>>>,
    pub beaches: Vec<Beach>,
}

impl Ocean {
    pub fn new() -> Ocean {
        Ocean {
            reefs: Vec::new(),
            beaches: Vec::new(),
        }
    }

    pub fn add_beach(&mut self, beach: Beach) {
        self.beaches.push(beach);
    }

    pub fn beaches(&self) -> Iter<Beach> {
        self.beaches.iter()
    }

    pub fn reefs(&self) -> Iter<Rc<RefCell<Reef>>> {
        self.reefs.iter()
    }

    /**
     * Generate a reef with the specified number of each concrete type of prey, and then add it to the ocean.
     *   - Minnows should have a speed of 25.
     *   - Shrimp should have an energy of 1.
     *
     * Returns a reference to the newly created reef.
     */
    pub fn generate_reef(
        &mut self,
        n_minnows: u32,
        n_shrimp: u32,
        n_clams: u32,
        n_algae: u32,
    ) -> Rc<RefCell<Reef>> {
        let reef = Rc::new(RefCell::new(Reef::new()));
        for _ in 0..n_minnows {
            let prey = Minnow::new(25);
            reef.as_ref().borrow_mut().add_prey(Box::new(prey));
        }
        for _ in 0..n_shrimp {
            let prey = Shrimp::new(1);
            reef.as_ref().borrow_mut().add_prey(Box::new(prey));
        }
        for _ in 0..n_clams {
            let prey = Clam::new();
            reef.as_ref().borrow_mut().add_prey(Box::new(prey));
        }
        for _ in 0..n_algae {
            let prey = Algae::new();
            reef.as_ref().borrow_mut().add_prey(Box::new(prey));
        }
        self.reefs.push(reef.clone());
        return reef;
    }
}
