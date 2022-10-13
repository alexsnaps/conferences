use std::marker::PhantomData;

struct Grounded;
struct Launched;

struct Rocket<Stage> {
    stage: PhantomData<Stage>,
    mass_kgs: u32,
}

impl Rocket<Grounded> {

    pub fn new(mass: u32) -> Self {
        Self {
            stage: PhantomData::default(),
            mass_kgs: mass,
        }
    }

    pub fn launch(self) -> Rocket<Launched> {
        Rocket {
            stage: PhantomData::default(),
            mass_kgs: self.mass_kgs,
        }
    }
}

impl Rocket<Launched> {
    pub fn accelerate(&mut self) {}
    pub fn decelerate(&mut self) {}
}

impl<Stage> Rocket<Stage> {
    pub fn mass_kgs(&self) -> u32 {
        self.mass_kgs
    }
    pub fn explode(self) { }
}

#[cfg(test)]
mod test {
    use crate::Rocket;

    #[test]
    fn test() {
        let rocket = Rocket::new(2_000_000);
        let mass = rocket.mass_kgs();
        let mut launched = rocket.launch();
        launched.accelerate();
        launched.decelerate();
        assert_eq!(mass, launched.mass_kgs());
        assert_eq!(2_000_000, launched.mass_kgs());
        launched.explode();
        // rocket.launch();
    }

































    use crate::Container;

    #[test]
    fn test_container() {
        let value = 42;
        let _reference = {
            let mut container = Container::new(value);
            let r1 = container.as_mut();
            let r2 = container.as_mut();
            // assert_eq!(value, *r1);
            assert_eq!(value, *r2);
        };
    }

    // fn foo<'a, 'b: 'a>(one: &'a u32, two: &'b u32) {
    //
    // }













    // #[test]
    // fn path_buf() {
    //     let pb = Path::new("./main.rs");
    //     let p = pb.parent();
    //     let mut components = pb.components();
    //     println!("{:?}", components);
    //     let c = components.next_back().unwrap();
    //     println!("{:?}", components);
    //     let finished = components.next_back().is_none();
    //     println!("Finished = {}", finished);
    //     match c {
    //         Component::Prefix(_) => {}
    //         Component::RootDir => {}
    //         Component::CurDir => {}
    //         Component::ParentDir => {}
    //         Component::Normal(_) => {}
    //     }
    //     assert_eq!(pb.parent().unwrap().parent(), None);
    // }
}


// mod more {
//     struct Bottle {
//         capacity: u8,
//     }
//
//     impl Bottle {
//         pub fn full() -> Self {
//             Bottle {
//                 capacity: u8::MAX,
//             }
//         }
//     }
//
//
//     pub fn max(num1: i32, num2: i32) -> i32 {
//         let nums = vec![num1, num2];
//         nums
//           .iter()
//           .fold(std::i32::MIN, |a,b| a.max(*b))
//     }
// }


fn main() {
    // fn think() -> u8 {
    //     let base = 0;
    //
    //     let answer_to_everything = {
    //         let base: u8 = 30;
    //         base + 12
    //     };
    //
    //     answer_to_everything
    // }
    //
    // println!("And the answer is {}", think());
}


struct Container<T> {
    value: T,
}

impl <T> Container<T> {

    pub fn new(value: T) -> Self {
        Self {
            value,
        }
    }

    pub fn reference(&self) -> &T {
        &self.value
    }

    pub fn as_mut(&mut self) -> &mut T {
        &mut self.value
    }
}

