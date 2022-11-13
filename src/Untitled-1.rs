
struct Crossing {
    size: usize,
    required_speed: u32,
    exit_speed: u32
}

fn fuse(left: Crossing, right: Crossing) -> Crossing {
    let total_size = left.size + right.size;
    let required_speed = if right.required_speed <= left.exit_speed {
        left.required_speed
    } else {
        left.size + right.required_speed
    };
    let exit_speed = left
        .exit_speed
        .checked_sub(right.size as u32)
        .unwrap_or_default()
        .max(right.exit_speed);
    Crossing {
        size: total_size,
        required_speed,
        exit_speed,
    }
}

fn par_crossing(nums: &[u32]) -> Crossing {
    nums
        .iter()
        .fold(
            || default::Default(),
            |crossing, num| {
                let exit_speed = crossing
                    .exit_speed
                    .checked_sub(1)
                    .unwrap_or_default()
                    .max(*num);
                let required_speed = if crossing.exit_speed == 0 {
                    return crossing.size as u32 + 1
                }
            }
        )
}

// Class about stopping a fold in the middle of the process: https://leetcode.com/problems/jump-game/
fn can_jump (nums: &[u32]) -> bool {
    nums
        .iter()
        .try_fold(1u32, |incoming_speed, num| {
            if incoming_speed == 0 {
                None
            } else {
                let existing_speed: u32 = 
                    incoming_speed.checked_sub(1).unwrap_or_default().max(*num);
                    Some(existing_speed)
            }
        })
        .is_some()
}

// fn main() {
//     let v = [1, 2, 4, 2, 1, 0, 1, 0];
//     let v: Vec<_> = std::iter::repeat_with(|| rand::random::<u32>() % 100)
            // .take(10_000_000)
            // .collect()
    let start = std::time::Instant::now();
    let seq_res = can_jump(&v);

    let par_res = 
    // diam::display_svg(|| par_crossing(&v).required_speed == 1;

//     println!("{}", can_jump(&v));
// }


// Algorithms for interrupting parallel computations:
// 