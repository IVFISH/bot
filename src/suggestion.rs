use crate::bitmatrix::*;
use crate::constants::*;
use crate::game::*;
use crate::nontrivials::*;

#[derive(Debug, PartialEq, Copy, Clone)]
pub enum Command {
    Null, // default
    MoveHorizontal(i8),
    MoveDrop,
    Rotate(u8),
    Backtrack(usize),
    Hold,
}

pub const COMMANDS: [Command; 6] = [
    Command::MoveHorizontal(1),
    Command::MoveHorizontal(-1),
    Command::Rotate(1),
    Command::Rotate(2),
    Command::Rotate(3),
    Command::MoveDrop,
];

pub fn to_command_list(before: Game, after: Game, hold: bool) -> Vec<Command> {
    // TODO: update piece based on hold
    let piece = before.peek() - 1;

    let collisions = collision(before.board, piece);
    // let reachables = reachable(collisions, piece);

    let placed = after.board ^ before.board;
    let (rr, cc) = get_initial_location(piece);

    for (r, c, rot) in get_location(placed, piece) {
        // initialize the commands
        let mut commands = vec![];
        let mut stack = vec![(Command::Null, (r, c, rot, 0))];
        let mut seen = [Bitmatrix::new(); 4];

        // whether it was reached properly
        let mut found = false;

        // do a dfs to find the rev-commands from (r, c, rot) -> (rr, cc)
        while let Some((command, (r, c, rot, depth))) = stack.pop() {
            if (r, c, rot) == (rr, cc, 0) {
                found = true;
                break;
            }

            commands.truncate(depth);
            commands.push(command);

            // try each command in reverse
            for command in COMMANDS {
                // maybe we can speed up by using reachables to filter
                // and then move towards a trivial
                if let Some((rr, cc, rrot)) =
                    inverse_command(command, piece, r, c, rot, &collisions, &seen)
                {
                    assert!(!seen[rrot].get(rr, cc));
                    seen[rrot].set(rr, cc);
                    assert!(seen[rrot].get(rr, cc));
                    stack.push((command, (rr, cc, rrot, depth + 1)));
                }
            }

            // TODO: change order so that it finds earlier
            let mut rr = r + 1;
            while (rr < COL::BITS as usize) && collisions[rot].get(rr, c) && !seen[rot].get(rr, c) {
                stack.push((Command::MoveDrop, (rr, c, rot, depth + 1)));
                seen[rot].set(rr, c);
                rr += 1;
            }
        }

        if !found {
            continue;
        }

        // append the hold piece
        if hold {
            commands.push(Command::Hold);
        }

        // reverse because we are backtracking
        commands.reverse();

        // maybe we should group movelefts/rights here

        return commands;
    }

    // TODO: what to do if did not find
    panic!(
        "did not find a way to generate movements {} -> {}",
        before, after
    )
}

pub fn inverse_command(
    command: Command,
    piece: usize,
    r: usize,
    c: usize,
    rot: usize,
    cmaps: &[Bitmatrix; 4],
    seen: &[Bitmatrix; 4],
) -> Option<(usize, usize, usize)> {
    match command {
        Command::Null => Some((r, c, rot)),
        Command::MoveHorizontal(1) => {
            if c > 0 && cmaps[rot].get(r, c - 1) && !seen[rot].get(r, c - 1) {
                Some((r, c - 1, rot))
            } else {
                None
            }
        }
        Command::MoveHorizontal(-1) => {
            if c < (W - 1) && cmaps[rot].get(r, c + 1) && !seen[rot].get(r, c + 1) {
                Some((r, c + 1, rot))
            } else {
                None
            }
        }
        Command::Rotate(1) => {
            try_inverse_rotate(piece, r as i32, c as i32, (rot + 3) % 4, rot, cmaps, seen)
        }
        Command::Rotate(2) => {
            try_inverse_rotate(piece, r as i32, c as i32, (rot + 2) % 4, rot, cmaps, seen)
        }
        Command::Rotate(3) => {
            try_inverse_rotate(piece, r as i32, c as i32, (rot + 1) % 4, rot, cmaps, seen)
        }
        _ => None,
    }
}

pub fn try_inverse_rotate(
    piece: usize,
    r: i32,
    c: i32,
    from: usize,
    to: usize,
    cmaps: &[Bitmatrix; 4],
    seen: &[Bitmatrix; 4],
) -> Option<(usize, usize, usize)> {
    let kicks = KICKS[piece][from][(to + 4 - from - 1) % 4];

    for (i, &[dx, dy]) in kicks.iter().enumerate() {
        let (rr, cc) = (r - dy, c - dx);
        if rr < 0 || cc < 0 || rr >= H as i32 || cc >= W as i32 {
            continue;
        }

        let (rr, cc) = (rr as usize, cc as usize);

        // check whether there exists a piece there
        if !cmaps[from].get(rr, cc) || seen[from].get(rr, cc) {
            continue;
        }

        // check whether the piece could kick to different location
        let mut skip = false;
        for j in 0..i {
            let [ddx, ddy] = kicks[j];
            let (rr, cc) = (r - ddy, c - ddx);
            if rr < 0 || cc < 0 || rr >= H as i32 || cc >= W as i32 {
                continue;
            }

            let (rr, cc) = (rr as usize, cc as usize);

            if cmaps[from].get(rr, cc) {
                skip = true;
                break;
            }
        }

        if skip {
            continue;
        }

        return Some((rr, cc, from));
    }

    None
}

pub fn get_location(placed: Bitmatrix, piece: usize) -> Vec<(usize, usize, usize)> {
    let mut ret = Vec::new();

    // iterate over the rotations for bitmatrix to check match
    for rot in 0..4 {
        let mut piece_matrix = Bitmatrix::new();
        for (i, &c) in PIECES[piece][rot].iter().enumerate() {
            // TODO: use eshans code that places it at bottom
            piece_matrix[i] = c << 13;
        }

        let (canon_piece, dr1, dc1) = get_canonical(piece_matrix);
        let (canon_placed, dr2, dc2) = get_canonical(placed);

        // println!("{}\n{}", canon_piece, canon_placed);
        // println!("rot: {} => ({} {}) ({} {})", rot, dr1, dc1, dr2, dc2);

        if canon_piece == canon_placed {
            // ret.push((dr2 - dr1, dc2 - dc1, rot));
            ret.push((dr2, dc2, rot));
        }
    }

    ret
}

// move piece to a canonical location (bottom left)
// return the dr, dc
pub fn get_canonical(p: Bitmatrix) -> (Bitmatrix, usize, usize) {
    let mut r = 0;
    let mut c = 0;
    let mut ret = p;

    while ret[0] == 0 {
        c += 1;
        ret = ret.lshift(1);
    }

    while ret.data.iter().all(|c| c & (1 << (COL::BITS - 1)) == 0) {
        r += 1;
        ret = ret.dshift(1);
    }

    (ret, r, c)
}

pub fn get_initial_location(piece: usize) -> (usize, usize) {
    // TODO: FIX
    (12, 4)
}
