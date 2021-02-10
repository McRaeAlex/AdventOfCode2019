use std::cmp::Ordering;
use std::io::{self, prelude::*};

fn main() {
    let (map, mut carts) = parse_input();
    display_map(&map);
    display_map_and_carts(&map, &carts);
    let mut tick = 0;
    while carts.len() > 1 {
        carts.sort();
        let mut collisions: Vec<usize> = vec![];
        let mut next_iter = vec![];
        for (i, old_cart) in carts.iter().enumerate() {
            // transform the cart
            let mut cart = move_cart(old_cart);
            match map[cart.pos.1][cart.pos.0] {
                '\\' => match cart.direction {
                    0 => cart.direction = 3,
                    1 => cart.direction = 2,
                    2 => cart.direction = 1,
                    3 => cart.direction = 0,
                    _ => {}
                },
                '/' => match cart.direction {
                    0 => cart.direction = 1,
                    1 => cart.direction = 0,
                    2 => cart.direction = 3,
                    3 => cart.direction = 2,
                    _ => {}
                },
                '+' => match cart.step {
                    0 => {
                        match cart.direction {
                            0 => cart.direction = 3,
                            1 => cart.direction = 0,
                            2 => cart.direction = 1,
                            3 => cart.direction = 2,
                            _ => {}
                        }
                        cart.step += 1;
                    }
                    1 => {
                        cart.step += 1;
                    }
                    2 => {
                        match cart.direction {
                            0 => cart.direction = 1,
                            1 => cart.direction = 2,
                            2 => cart.direction = 3,
                            3 => cart.direction = 0,
                            _ => {}
                        }
                        cart.step = 0;
                    }
                    _ => {}
                },
                _ => {}
            }
            // compare the cart to the other carts
            if !collisions.contains(&i) {
                for j in i..carts.len() {
                    let other_cart = &carts[j];
                    if cart == *other_cart && i != j {
                        if !collisions.contains(&i) {
                            collisions.push(i);
                        }
                        if !collisions.contains(&j) {
                            collisions.push(j);
                        }
                    }
                }
            }
            next_iter.push(cart);
        }
        collisions.sort();
        collisions.reverse();
        for i in collisions {
            next_iter.remove(i);
        }
        remove_collided_carts(&mut next_iter);
        carts = next_iter;
        tick += 1;
        println!("{}", tick);
        //display_map_and_carts(&map, &carts);
    }
    for cart in &mut carts {
        println!("{:?}", cart);
    }
}

fn move_cart_mut(cart: &mut Cart) {
    match cart.direction {
        0 => cart.pos = (cart.pos.0, cart.pos.1 - 1),
        1 => cart.pos = (cart.pos.0 + 1, cart.pos.1),
        2 => cart.pos = (cart.pos.0, cart.pos.1 + 1),
        3 => cart.pos = (cart.pos.0 - 1, cart.pos.1),
        _ => {}
    }
}

fn remove_collided_carts(carts: &mut Vec<Cart>) {
    let mut collisions: Vec<usize> = vec![];
    for (i, cart) in carts.iter().enumerate() {
        for (j, other_cart) in carts.iter().enumerate() {
            if *cart == *other_cart && i != j {
                collisions.push(i);
                break;
            }
        }
    }
    collisions.sort();
    collisions.reverse();
    for col in collisions {
        carts.remove(col);
    }
}

fn move_cart(cart: &Cart) -> Cart {
    match cart.direction {
        0 => Cart {
            pos: (cart.pos.0, cart.pos.1 - 1),
            direction: cart.direction,
            step: cart.step,
        },
        1 => Cart {
            pos: (cart.pos.0 + 1, cart.pos.1),
            direction: cart.direction,
            step: cart.step,
        },
        2 => Cart {
            pos: (cart.pos.0, cart.pos.1 + 1),
            direction: cart.direction,
            step: cart.step,
        },
        3 => Cart {
            pos: (cart.pos.0 - 1, cart.pos.1),
            direction: cart.direction,
            step: cart.step,
        },
        _ => {
            eprintln!("Error invalid direction");
            Cart {
                pos: (cart.pos.0 - 1, cart.pos.1),
                direction: cart.direction,
                step: cart.step,
            }
        }
    }
}

fn display_map(map: &Vec<Vec<char>>) {
    for y in map {
        for x in y {
            print!("{}", x);
        }
        println!("");
    }
}

fn display_map_and_carts(map: &Vec<Vec<char>>, carts: &Vec<Cart>) {
    let mut display = map.clone();
    for cart in carts {
        println!("{:?}", cart);
        display[cart.pos.1][cart.pos.0] = match cart.direction {
            0 => '^',
            1 => '>',
            2 => 'v',
            3 => '<',
            _ => 'O',
        };
    }
    for y in display {
        for x in y {
            print!("{}", x);
        }
        println!("");
    }
}

fn parse_input() -> (Vec<Vec<char>>, Vec<Cart>) {
    let stdin = io::stdin();
    let mut tracks: Vec<Vec<char>> = vec![];
    let mut carts: Vec<Cart> = vec![];
    for (y, line) in stdin.lock().lines().enumerate() {
        let mut new_track: Vec<char> = vec![];
        for (x, track) in line.unwrap().chars().enumerate() {
            match track {
                '>' => {
                    carts.push(Cart {
                        direction: 1,
                        pos: (x, y),
                        step: 0,
                    });
                    new_track.push('-');
                }
                '<' => {
                    carts.push(Cart {
                        direction: 3,
                        pos: (x, y),
                        step: 0,
                    });
                    new_track.push('-');
                }
                '^' => {
                    carts.push(Cart {
                        direction: 0,
                        pos: (x, y),
                        step: 0,
                    });
                    new_track.push('|');
                }
                'v' => {
                    carts.push(Cart {
                        direction: 2,
                        pos: (x, y),
                        step: 0,
                    });
                    new_track.push('|');
                }
                _ => new_track.push(track),
            }
        }
        tracks.push(new_track);
    }
    (tracks, carts)
}

#[derive(Debug, Eq, Clone)]
struct Cart {
    direction: u8, // 0:up, 1:right, 2:down, 3:left
    pos: (usize, usize),
    step: usize,
}

impl Ord for Cart {
    fn cmp(&self, other: &Cart) -> Ordering {
        match self.pos.1.cmp(&other.pos.1) {
            Ordering::Less => Ordering::Less,
            Ordering::Greater => Ordering::Greater,
            Ordering::Equal => self.pos.0.cmp(&other.pos.0),
        }
    }
}

impl PartialOrd for Cart {
    fn partial_cmp(&self, other: &Cart) -> Option<Ordering> {
        Some(self.cmp(other))
    }
}

impl PartialEq for Cart {
    fn eq(&self, other: &Cart) -> bool {
        self.pos == other.pos
    }
}
