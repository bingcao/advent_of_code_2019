use intcode::Program;
use std::cell::RefCell;

const NUM_COMPUTERS: usize = 50;

pub fn run_network(program_str: &str) {
    let mut programs = vec![];
    for i in 0..NUM_COMPUTERS {
        let program = Program::new(program_str, &vec![i as i128]);
        programs.push(RefCell::new(program));
    }
    let mut packets = vec![];
    for _ in 0..NUM_COMPUTERS {
        let empty_packet = vec![];
        packets.push(empty_packet);
    }

    let mut nat = (0, 0);
    let mut last_nat_y = -1;
    let mut consecutive_blocked_inputs = [0; NUM_COMPUTERS];

    loop {
        for (i, program_cell) in programs.iter().enumerate() {
            let mut program = program_cell.borrow_mut();
            if program.needs_input() {
                if program.num_inputs() == 0 {
                    program.send_input(-1);
                    consecutive_blocked_inputs[i] += 1;
                } else {
                    consecutive_blocked_inputs[i] = 0;
                }
            }
            let (done, output) = program.execute();
            assert_eq!(done, false);
            if let Some(o) = output {
                let packet = packets.get_mut(i).unwrap();
                packet.push(o);
                if packet.len() == 3 {
                    if packet[0] == 255 {
                        nat = (packet[1], packet[2]);
                        if last_nat_y == -1 {
                            println!("Part 1 Result: {}", nat.1);
                        }
                    } else {
                        let mut receiver = programs[packet[0] as usize].borrow_mut();
                        receiver.send_input(packet[1]);
                        receiver.send_input(packet[2]);
                    }
                    *packet = vec![];
                }
            }
        }
        if consecutive_blocked_inputs.iter().all(|c| *c >= 2) {
            if nat.1 == last_nat_y {
                println!("Part 2 Result: {}", nat.1);
                return;
            } else {
                last_nat_y = nat.1;
            }
            println!("Handling all idle by sending {:?}", nat);
            let mut receiver = programs[0].borrow_mut();
            receiver.send_input(nat.0);
            receiver.send_input(nat.1);
            consecutive_blocked_inputs[0] = 0;
        }
    }
}
