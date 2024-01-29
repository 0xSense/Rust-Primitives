fn main() {
    struct Cat {
        color: String,
        age: u8,
        name: String,
        alive: bool,
    }

    // call a struct with the stuct name an your desired variable name
    let spartan_the_cat = Cat {
        color: "orange",
        age: 9,
        name: "Spartan",
        alive: true,
    }

    // you can update mutable values using dot notation
    let mut mute_the_cat = Cat {
        color: "black",
        age: 4,
        name: "mutie",
        alive: true,
    }
    
    mute_the_cat.name = String::from("Mutie")
}
