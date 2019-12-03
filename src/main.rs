fn main() {
    let mut computer : Vec<u64> = std::fs::read_to_string("inputs.txt").unwrap().split(",").map(|input| input.clone().parse::<u64>().unwrap()).collect();
    let mut index = 0;

    computer[1] = 12;
    computer[2] = 2;

    loop {
        let clone = computer.clone();
        match computer[index] {
            1 => {
                computer[clone[index + 3] as usize] = clone[clone[index + 1] as usize] + clone[clone[index + 2] as usize]
            },
            2 => {
                computer[clone[index + 3] as usize] = clone[clone[index + 1] as usize] * clone[clone[index + 2] as usize]
            }
            99 => {
                break;
            }
            _ => {
                panic!("WTF! {:?}", computer);
            }
        }

        index += 4;
    }

    println!("{}", computer[0]);
}
