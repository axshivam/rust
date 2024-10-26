fn main() {
    let config_max: Option<u8> = Some(3u8);
    // match config_max {
    //     Some(max) => println!("The maximum is configured to be {max}"),
    //     None => (),
    //     _ => (),
    // }

    if let Some(max) = config_max {
        println!("The maximum is configured to be {max}");
    }

    let config_max_1: Option<u8> = None;

    if let None = config_max_1 {
        println!("There is no config");
    }
}
