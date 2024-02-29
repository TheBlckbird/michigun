use crossterm::event::{self, Event};
use std::{io, process::exit, thread, time::Duration};

fn main() {
    stop_motion();
}

fn stop_motion() -> io::Result<()> {
    let frames = [
        "              \n#     ∆∆∆     ",
        "              \n #    ∆∆∆     ",
        "              \n  #   ∆∆∆     ",
        "              \n   #  ∆∆∆     ",
        "              \n    # ∆∆∆     ",
        "              \n     #∆∆∆     ",
        "      #       \n      ∆∆∆     ",
        "       #      \n      ∆∆∆     ",
        "        #     \n      ∆∆∆     ",
        "              \n      ∆∆∆#    ",
        "              \n      ∆∆∆ #   ",
        "              \n      ∆∆∆  #  ",
        "              \n      ∆∆∆   # ",
        "              \n      ∆∆∆    #",
    ];

    for (index, frame) in frames.iter().enumerate() {
        println!("{}", frame);

        if event::poll(Duration::from_millis(500))? {
            if let Event::Key(_) = event::read()? {
            } else if index == 5 {
                exit(1);
            }
            thread::sleep(Duration::from_millis(500));
        } else if index == 5 {
            exit(1);
        }
        println!("{index}");
    }

    Ok(())
    // println!("              \n#     ∆∆∆     ");
    // thread::sleep(Duration::from_millis(500));
    // println!("              \n #    ∆∆∆     ");
    // thread::sleep(Duration::from_millis(500));
    // println!("              \n  #   ∆∆∆     ");
    // thread::sleep(Duration::from_millis(500));
    // println!("              \n   #  ∆∆∆     ");
    // thread::sleep(Duration::from_millis(500));
    // println!("              \n    # ∆∆∆     ");
    // thread::sleep(Duration::from_millis(500));
    // println!("              \n     #∆∆∆     ");
    // thread::sleep(Duration::from_millis(500));
    // println!("      #       \n      ∆∆∆     ");
    // thread::sleep(Duration::from_millis(500));
    // println!("       #      \n      ∆∆∆     ");
    // thread::sleep(Duration::from_millis(500));
    // println!("        #     \n      ∆∆∆     ");
    // thread::sleep(Duration::from_millis(500));
    // println!("              \n      ∆∆∆#    ");
    // thread::sleep(Duration::from_millis(500));
    // println!("              \n      ∆∆∆ #   ");
    // thread::sleep(Duration::from_millis(500));
    // println!("              \n      ∆∆∆  #  ");
    // thread::sleep(Duration::from_millis(500));
    // println!("              \n      ∆∆∆   # ");
    // thread::sleep(Duration::from_millis(500));
    // println!("              \n      ∆∆∆    #");
    // thread::sleep(Duration::from_millis(500));
}

fn xmain() -> io::Result<()> {
    let length = 15;
    let height = 2;
    let spike_position = 6;
    let mut position = (0, height - 1);
    let mut is_jumping = false;
    let mut frames_in_air = 0;

    loop {
        println!("{}", render(position, length, height, spike_position));

        if event::poll(Duration::from_millis(500))? {
            if let Event::Key(_) = event::read()? {
                if !is_jumping {
                    position.0 += 1;
                    position.1 -= 1;
                    is_jumping = true;
                }
            }
        } else {
            position.0 += 1;

            if is_on_floor(position, height)
                && (position.0 == spike_position
                    || position.0 == spike_position + 1
                    || position.0 == spike_position + 2
                    || position.0 == spike_position + 3)
            {
                break;
            }

            if is_jumping {
                frames_in_air += 1;
            }

            if frames_in_air > 2 {
                position.1 = height - 1;
                frames_in_air = 0;
                is_jumping = false;
            }
        }
    }
    Ok(())
}

fn render(position: (u8, u8), length: u8, height: u8, spike_position: u8) -> String {
    let mut string = String::new();

    for i in 0..height {
        for j in 0..length {
            let char =
                if (j == spike_position || j == spike_position + 1 || j == spike_position + 2)
                    && is_on_floor((j, i), height)
                {
                    '∆'
                } else if j == position.0 && i == position.1 {
                    '#'
                } else {
                    '.'
                };
            string.push(char);
        }

        if i != height - 1 {
            string.push('\n');
        }
    }
    // for i in 0..length {
    //     string.push(if i == position.0 { '#' } else { ' ' });
    // }

    string
}

fn is_on_floor(position: (u8, u8), height: u8) -> bool {
    position.1 == height - 1
}
