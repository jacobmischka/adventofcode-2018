use std::{io, marker::PhantomPinned, pin::Pin};

use regex::Regex;

fn main() {
    let input_re =
        Regex::new(r"(?P<players>\d+) players; last marble is worth (?P<last_marble_points>\d+)")
            .unwrap();
    let mut input = String::new();
    io::stdin().read_line(&mut input).unwrap();

    let caps = input_re.captures(&input).unwrap();

    let num_players: usize = caps.name("players").unwrap().as_str().parse().unwrap();
    let last_marble_points: usize = caps
        .name("last_marble_points")
        .unwrap()
        .as_str()
        .parse()
        .unwrap();

    unsafe {
        let mut marbles: Vec<Pin<Box<Marble>>> = Vec::with_capacity(last_marble_points * 100);
        let mut marble = Marble {
            val: 0,
            next: std::ptr::null_mut(),
            prev: std::ptr::null_mut(),
            _pin: PhantomPinned,
        };
        marble.next = &mut marble;
        marble.prev = &mut marble;
        marbles.push(Box::pin(marble));

        let mut current_marble: *mut Marble = marbles[0].as_mut().get_unchecked_mut();

        let mut points: Vec<usize> = vec![0; num_players];

        let mut t: usize = 1;
        while t <= last_marble_points * 100 {
            if t % 23 == 0 {
                let mut to_remove: *mut Marble = current_marble;
                for _ in 0..7 {
                    to_remove = &mut *(*to_remove).prev;
                }

                current_marble = &mut *(*to_remove).next;
                (*(*to_remove).prev).next = current_marble;
                (*(*current_marble).prev).next = current_marble;
                (*current_marble).prev = (*to_remove).prev;

                points[t % num_players] += t + (*to_remove).val;
            } else {
                let next = (*current_marble).next;
                marbles.push(Box::pin(Marble {
                    val: t,
                    prev: next,
                    next: (*next).next,
                    _pin: PhantomPinned,
                }));
                let added: *mut Marble = marbles.last_mut().unwrap().as_mut().get_unchecked_mut();
                (*(*next).next).prev = added;
                (*next).next = added;
                current_marble = added;
            }

            if t == last_marble_points {
                println!("Part 1: {}", points.iter().copied().max().unwrap());
            }

            t += 1;
        }
        println!("Part 2: {}", points.iter().copied().max().unwrap());
    }
}

#[derive(Debug, Clone)]
struct Marble {
    val: usize,
    prev: *mut Marble,
    next: *mut Marble,
    _pin: PhantomPinned,
}
