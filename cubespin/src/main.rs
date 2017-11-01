// Copyright (c) 2015, Huitse Tai. All rights reserved.
// Use of this source code is governed by a BSD-style license
// that can be found in the LICENSE file.


mod mcube;

fn main() {
    let end_stat = mcube::StatT(
        [
            [
                1, 1, 1,
                1, 1, 1,
                1, 1, 1
            ],
            [
                2, 2, 2,
                2, 2, 2,
                2, 2, 2
            ],
            [
                3, 3, 3,
                3, 3, 3,
                3, 3, 3
            ],
            [
                4, 4, 4,
                4, 4, 4,
                4, 4, 4
            ],
            [
                5, 5, 5,
                5, 5, 5,
                5, 5, 5
            ],
            [
                6, 6, 6,
                6, 6, 6,
                6, 6, 6
            ]
        ]
    );
    let begin_stat = mcube_mix(mcube::StatT(end_stat.0.clone()),
        &[
            -1,
            5,
            6,
            -3,
            6,
            -3,
            6,
            -2,
            1,
            -4,
            -6
        ][..]
    );
    let _ = mcube::StatT(
        [
            [
                5, 5, 4,
                3, 1, 1,
                2, 3, 1
            ],
            [
                2, 4, 6,
                3, 2, 6,
                3, 4, 5
            ],
            [
                3, 6, 4,
                2, 3, 6,
                1, 3, 6
            ],
            [
                5, 4, 3,
                5, 4, 5,
                4, 1, 1
            ],
            [
                6, 1, 6,
                2, 5, 2,
                3, 4, 4
            ],
            [
                2, 1, 5,
                5, 6, 2,
                1, 6, 2
            ]
        ]
    );

    let mut curr_stat = mcube::StatT(begin_stat.0.clone());
    mcube::print(&curr_stat);

    let solutions = mcube::autospin(&begin_stat, &end_stat);
    println!("Auto spinning steps:");

    for s in solutions {
        println!("{}", s);
        curr_stat = mcube::spin(curr_stat, s);
        mcube::print(&curr_stat);
    }
}

fn mcube_mix(stat: mcube::StatT, steps: &[i8]) -> mcube::StatT {
    let mut s = stat;
    for st in steps {
        s = mcube::spin(s, *st);
    }
    return s;
}
