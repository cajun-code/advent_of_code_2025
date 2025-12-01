advent_of_code::solution!(1);

const START_OF_DIAL:i32 = 0;
const END_OF_DIAL: i32 = 100;


fn rotate_dial(position: i32, rotation_string: String) -> (i32, i32){
    let (direction, amount) = rotation_string.split_at(1);
    //println!("Direction {} Amount {}", direction, amount);
    let len = amount.trim().len();
    let amt:i32 = if len < 3 {
        amount.trim().parse().unwrap()
    }else{
        amount.trim()[len-2..].parse().unwrap()
    };
    let mut rotational_passes = if len < 3 {0} else {
        amount.trim().parse::<i32>().unwrap() / END_OF_DIAL
    };
    let mut pos: i32;
    match direction {
        "L" => {
            pos = position - amt;
            if pos < START_OF_DIAL {
                pos = END_OF_DIAL + pos;
                if position != 0{
                    rotational_passes += 1;
                }
            }
        }
        "R" => {
            pos = position + amt;
            if pos > END_OF_DIAL {
                pos = END_OF_DIAL - pos;
                rotational_passes += 1;
            }
        }
        &_ => todo!()
    }
    if pos == END_OF_DIAL{
        pos = START_OF_DIAL
    }
    println!("starting position {} rotation code {} new position {} with {} passes", position, rotation_string.trim(), pos.abs(), rotational_passes);
    (pos.abs(), rotational_passes)
}


pub fn part_one(input: &str) -> Option<u64> {
    let mut password:u64 = 0;
    let start_pos = 50;
    let mut current_pos = start_pos;
    for rotation in input.trim().split("\n"){
        (current_pos, _)= rotate_dial(current_pos, String::from(rotation));
        if current_pos == 0{
            password += 1;
        }
    }
    println!("*{}*", password);
    Some(password)
}

pub fn part_two(input: &str) -> Option<u64> {
    let mut password:u64 = 0;
    let start_pos = 50;
    let mut current_pos = start_pos;
    for rotation in input.trim().split("\n"){
        let (pos, passes)= rotate_dial(current_pos, String::from(rotation));
        if pos == 0{
            password += 1;
        }
        password += passes as u64;
        current_pos = pos;
    }
    println!("*{}*", password);
    Some(password)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_split(){
        let s = "L65";
        let (direction, amount) = s.split_at(1);
        assert_eq!(direction, "L");
        assert_eq!(amount, "65");
    }

    #[test]
    fn test_rotation(){
        let mut current_pos: i32;
        (current_pos, _) = rotate_dial(50, String::from("L68"));
        assert_eq!(current_pos, 82);
        (current_pos, _) = rotate_dial(current_pos, String::from("L30"));
        assert_eq!(current_pos, 52);
        (current_pos, _)= rotate_dial(current_pos, String::from("R48"));
        assert_eq!(current_pos, 0);
    }

    #[test]
    fn test_3_digit_rotation(){
        let (current_pos, _) = rotate_dial(50, String::from("L668"));
        assert_eq!(current_pos, 82);
    }

    #[test]
    fn test_1_digit_rotation(){
        let (current_pos, _) = rotate_dial(50, String::from("L2"));
        assert_eq!(current_pos, 48);
    }

    #[test]
    fn test_part_one() {
        let result = part_one(&advent_of_code::template::read_file("examples", DAY));
        
        assert_eq!(result, Some(3));
    }

    #[test]
    fn test_part_two() {
        let result = part_two(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Some(6));
    }
}
