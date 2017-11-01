// Copyright (c) 2015, Huitse Tai. All rights reserved.
// Use of this source code is governed by a BSD-style license
// that can be found in the LICENSE file.


//
// Magic Cube logical face number:
//       * * *
//       * 1 *
//       * * *
// * * * * * * * * * * * *
// * 2 * * 3 * * 4 * * 5 *
// * * * * * * * * * * * *
//       * * *
//       * 6 *
//       * * *
//
// code in one face
// 0 1 2
// 3 4 5
// 6 7 8

#[derive(Hash, Eq, PartialEq, Debug)]
pub struct StatT(pub [[i8; 9]; 6]);

pub fn print(stat: &StatT) {
    let StatT(s) = *stat;

    println!("       {} {} {}", s[0][0], s[0][1], s[0][2]);
    println!("       {} {} {}", s[0][3], s[0][4], s[0][5]);
    println!("       {} {} {}", s[0][6], s[0][7], s[0][8]);
    println!("");

    println!("{} {} {}  {} {} {}  {} {} {}  {} {} {}",
             s[1][0], s[1][1], s[1][2],
             s[2][0], s[2][1], s[2][2],
             s[3][0], s[3][1], s[3][2],
             s[4][0], s[4][1], s[4][2]);
    println!("{} {} {}  {} {} {}  {} {} {}  {} {} {}",
             s[1][3], s[1][4], s[1][5],
             s[2][3], s[2][4], s[2][5],
             s[3][3], s[3][4], s[3][5],
             s[4][3], s[4][4], s[4][5]);
    println!("{} {} {}  {} {} {}  {} {} {}  {} {} {}",
             s[1][6], s[1][7], s[1][8],
             s[2][6], s[2][7], s[2][8],
             s[3][6], s[3][7], s[3][8],
             s[4][6], s[4][7], s[4][8]);
    println!("");

    println!("       {} {} {}", s[5][0], s[5][1], s[5][2]);
    println!("       {} {} {}", s[5][3], s[5][4], s[5][5]);
    println!("       {} {} {}", s[5][6], s[5][7], s[5][8]);
}

pub fn spin(stat: StatT, op: i8) -> StatT {
    match op {
        0 => stat,
        1 => spin1(stat),
        -1 => spin1(spin1(spin1(stat))),
        2 => spin2(stat),
        -2 => spin2(spin2(spin2(stat))),
        3 => spin3(stat),
        -3 => spin3(spin3(spin3(stat))),
        4 => spin4(stat),
        -4 => spin4(spin4(spin4(stat))),
        5 => spin5(stat),
        -5 => spin5(spin5(spin5(stat))),
        6 => spin6(stat),
        -6 => spin6(spin6(spin6(stat))),
        _ => stat
    }
}

fn swap_cells(stat: StatT,
              f1: usize, c1: usize,
              f2: usize, c2: usize,
              f3: usize, c3: usize,
              f4: usize, c4: usize) -> StatT {
    let StatT(mut s) = stat;

    {
        let swap = s[f4-1][c4];
        s[f4-1][c4] = s[f3-1][c3];
        s[f3-1][c3] = s[f2-1][c2];
        s[f2-1][c2] = s[f1-1][c1];
        s[f1-1][c1] = swap;
    }

    return StatT(s);
}

fn swap_arris(stat: StatT,
              f1: usize, a1: [usize; 3],
              f2: usize, a2: [usize; 3],
              f3: usize, a3: [usize; 3],
              f4: usize, a4: [usize; 3]) -> StatT {
    let mut s = stat;

    for ci in 0..3 {
        s = swap_cells(s, f1, a1[ci], f2, a2[ci], f3, a3[ci], f4, a4[ci]);
    }

    return s;
}

fn spin_face(stat: StatT, fi: usize) -> StatT {
    let mut s = stat;

    s = swap_cells(s, fi, 1, fi, 5, fi, 7, fi, 3);
    s = swap_cells(s, fi, 0, fi, 2, fi, 8, fi, 6);

    return s;
}

fn spin1(stat: StatT) -> StatT {
    swap_arris(spin_face(stat, 1),
               5, [2, 1, 0],
               4, [2, 1, 0],
               3, [2, 1, 0],
               2, [2, 1, 0])
}

fn spin2(stat: StatT) -> StatT {
    swap_arris(spin_face(stat, 2),
               1, [0, 3, 6],
               3, [0, 3, 6],
               6, [0, 3, 6],
               5, [8, 5, 2])
}

fn spin3(stat: StatT) -> StatT {
    swap_arris(spin_face(stat, 3),
               1, [6, 7, 8],
               4, [0, 3, 6],
               6, [2, 1, 0],
               2, [8, 5, 2])
}

fn spin4(stat: StatT) -> StatT {
    swap_arris(spin_face(stat, 4),
               1, [8, 5, 2],
               5, [0, 3, 6],
               6, [8, 5, 2],
               3, [8, 5, 2])
}

fn spin5(stat: StatT) -> StatT {
    swap_arris(spin_face(stat, 5),
               1, [2, 1, 0],
               2, [0, 3, 6],
               6, [6, 7, 8],
               4, [8, 5, 2])
}

fn spin6(stat: StatT) -> StatT {
    swap_arris(spin_face(stat, 6),
               3, [6, 7, 8],
               4, [6, 7, 8],
               5, [6, 7, 8],
               2, [6, 7, 8])
}

use std::collections::HashMap;
use std::rc::Rc;

struct BfsNode(Option<Rc<BfsNode>>, i8);

pub fn autospin(begin_stat: &StatT, end_stat: &StatT) -> Vec<i8> {
    let mut lleaves: HashMap<StatT, Rc<BfsNode>> = HashMap::new();
    let mut rleaves: HashMap<StatT, Rc<BfsNode>> = HashMap::new();

    lleaves.insert(clone(begin_stat), Rc::new(BfsNode(None, 0)));
    rleaves.insert(clone(end_stat), Rc::new(BfsNode(None, 0)));

    match link_trees(&lleaves, &rleaves, check_linkage(&lleaves, &rleaves)) {
        Some(result) => return result,
        None => ()
    }

    for _ in 0..16 {
        lleaves = expand_leaves(lleaves);
        match link_trees(&lleaves, &rleaves, check_linkage(&lleaves, &rleaves)) {
            Some(result) => return result,
            None => ()
        }

        rleaves = expand_leaves(rleaves);
        match link_trees(&lleaves, &rleaves, check_linkage(&lleaves, &rleaves)) {
            Some(result) => return result,
            None => ()
        }
    }

    return vec![0];
}

fn clone(stat: &StatT) -> StatT {
    StatT(stat.0.clone())
}

fn expand_leaves(leaves: HashMap<StatT, Rc<BfsNode>>) -> HashMap<StatT, Rc<BfsNode>> {
    let mut fresh: HashMap<StatT, Rc<BfsNode>> = HashMap::new();

    for (stat, steps) in leaves {
        for step_op in 1..7 {
            fresh.insert(spin(clone(&stat), step_op), Rc::new(BfsNode(Some(steps.clone()), step_op)));
            fresh.insert(spin(clone(&stat), -step_op), Rc::new(BfsNode(Some(steps.clone()), -step_op)));
        }
    }

    return fresh;
}

fn check_linkage<'a>(lleaves: &'a HashMap<StatT, Rc<BfsNode>>, rleaves: &'a HashMap<StatT, Rc<BfsNode>>) -> Option<&'a StatT> {
    for stat in lleaves.keys() {
        if rleaves.contains_key(stat) {
            return Some(stat);
        }
    }
    return None;
}

fn link_trees(lleaves: &HashMap<StatT, Rc<BfsNode>>, rleaves: &HashMap<StatT, Rc<BfsNode>>, linkage: Option<&StatT>) -> Option<Vec<i8>> {
    match linkage {
        Some(key_stat) => {
            let mut llist = bfstep2list_hm(lleaves.get(key_stat));
            let mut rlist = bfstep2list_hm(rleaves.get(key_stat));
            rlist.reverse();

            for rlist_step in rlist {
                llist.push(-rlist_step);
            }

            Some(llist)
        },
        None => None
    }
}

fn bfstep2list_hm(steps: Option<&Rc<BfsNode>>) -> Vec<i8> {
    match steps {
        Some(thing) => bfstep2list(Some(thing.clone())),
        None => bfstep2list(None)
    }
}

fn bfstep2list(steps: Option<Rc<BfsNode>>) -> Vec<i8> {
    match steps {
        Some(substeps) => {
            let mut step_list = bfstep2list(substeps.0.clone());
            step_list.push(substeps.1);
            step_list
        },
        None => Vec::new()
    }
}
