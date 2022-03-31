use std::thread::sleep;
use std::time::Duration;

fn main() {
    animation_demo();
}

fn animation_demo() {
    let anim_speed = Duration::from_millis(700);
    let frame_array = [
        "[    ] ( 1) ( 0) ( 8) ( 3) ( 0) ( 0) [    ]\n|  5 |                               |  4 |\n[    ] ( 1) ( 1) ( 5) (10) ( 2) ( 8) [    ]\n         A    B    C    D    E    F   STORE",
        "[    ] ( 1) ( 0) ( 8) ( 3) ( 0) ( 0) [    ]\n|  5 |                 10            |  4 |\n[    ] ( 1) ( 1) ( 5) ( 0) ( 2) ( 8) [    ]\n         A    B    C    D    E    F   STORE",
        "[    ] ( 1) ( 0) ( 8) ( 3) ( 0) ( 0) [    ]\n|  5 |                       9       |  4 |\n[    ] ( 1) ( 1) ( 5) ( 0) ( 3) ( 8) [    ]\n         A    B    C    D    E    F   STORE",
        "[    ] ( 1) ( 0) ( 8) ( 3) ( 0) ( 0) [    ]\n|  5 |                            8  |  4 |\n[    ] ( 1) ( 1) ( 5) ( 0) ( 3) ( 9) [    ]\n         A    B    C    D    E    F   STORE",
        "[    ] ( 1) ( 0) ( 8) ( 3) ( 0) ( 0) [    ]\n|  5 |                              7|  5 |\n[    ] ( 1) ( 1) ( 5) ( 0) ( 3) ( 9) [    ]\n         A    B    C    D    E    F   STORE",
        "[    ] ( 1) ( 0) ( 8) ( 3) ( 0) ( 1) [    ]\n|  5 |                            6  |  5 |\n[    ] ( 1) ( 1) ( 5) ( 0) ( 3) ( 9) [    ]\n         A    B    C    D    E    F   STORE",
        "[    ] ( 1) ( 0) ( 8) ( 3) ( 1) ( 1) [    ]\n|  5 |                       5       |  5 |\n[    ] ( 1) ( 1) ( 5) ( 0) ( 3) ( 9) [    ]\n         A    B    C    D    E    F   STORE",
        "[    ] ( 1) ( 0) ( 8) ( 4) ( 1) ( 1) [    ]\n|  5 |                  4            |  5 |\n[    ] ( 1) ( 1) ( 5) ( 0) ( 3) ( 9) [    ]\n         A    B    C    D    E    F   STORE",
        "[    ] ( 1) ( 0) ( 9) ( 4) ( 1) ( 1) [    ]\n|  5 |             3                 |  5 |\n[    ] ( 1) ( 1) ( 5) ( 0) ( 3) ( 9) [    ]\n         A    B    C    D    E    F   STORE",
        "[    ] ( 1) ( 1) ( 9) ( 4) ( 1) ( 1) [    ]\n|  5 |        2                      |  5 |\n[    ] ( 1) ( 1) ( 5) ( 0) ( 3) ( 9) [    ]\n         A    B    C    D    E    F   STORE",
        "[    ] ( 2) ( 1) ( 9) ( 4) ( 1) ( 1) [    ]\n|  5 |   1                           |  5 |\n[    ] ( 1) ( 1) ( 5) ( 0) ( 3) ( 9) [    ]\n         A    B    C    D    E    F   STORE",
        "[    ] ( 2) ( 1) ( 9) ( 4) ( 1) ( 1) [    ]\n|  5 |   0                           |  5 |\n[    ] ( 2) ( 1) ( 5) ( 0) ( 3) ( 9) [    ]\n         A    B    C    D    E    F   STORE",
        "[    ] ( 2) ( 1) ( 9) ( 4) ( 1) ( 1) [    ]\n|  5 |                               |  5 |\n[    ] ( 2) ( 1) ( 5) ( 0) ( 3) ( 9) [    ]\n         A    B    C    D    E    F   STORE",
    ];
    sleep(anim_speed);
    for frame in frame_array.iter() {
        clearscreen::clear().unwrap();
        println!("{}", frame);
        sleep(anim_speed);
    }
}
