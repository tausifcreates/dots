use std::cmp::{max, min};

#[derive(Debug)]
pub struct Holder<'list> {
    list_a: &'list [i32],
    list_b: &'list [i32],
    len_a: usize,
    len_b: usize,
    median: Option<(i32, i32)>,
}

impl<'list> Holder<'list> {
    pub fn new(list_a: &'list [i32], list_b: &'list [i32]) -> Self {
        let len_a = list_a.len();
        let mut len_b = list_b.len();

        if (len_a + len_b) % 2 != 0 {
            len_b -= 1;
        }

        Self {
            list_a,
            list_b,
            len_a,
            len_b,
            median: None,
        }
    }

    pub fn run(&mut self, mut left_boundary_a: usize, mut right_boundary_a: usize) {
        let list_a = self.list_a;
        let list_b = self.list_b;
        let len_a = self.len_a;
        let len_b = self.len_b;

        if right_boundary_a == 1 {
            let divider_a_left = 0;
            let mut divider_a_right = 1;

            let mut divider_b_left = (len_a + len_b) / 2 - 2;
            let mut divider_b_right = divider_b_left + 1;

            if list_a[divider_a_left] <= list_b[divider_b_right]
                && list_b[divider_b_left] <= list_a[divider_a_right]
            {
                let median_left = max(list_a[divider_a_left], list_b[divider_b_left]);
                let median_right = min(list_a[divider_a_right], list_b[divider_b_right]);
                self.median = Some((median_left, median_right));
                return;
            } else {
                divider_a_right = 0;
                divider_b_left += 1;

                if len_a == len_b {
                    let median_left = list_b[divider_b_left];
                    let median_right = list_a[divider_a_right];
                    self.median = Some((median_left, median_right));
                    return;
                } else {
                    divider_b_right = divider_b_left + 1;
                    let median_left = list_b[divider_b_left];
                    let median_right = min(list_a[divider_a_right], list_b[divider_b_right]);
                    self.median = Some((median_left, median_right));
                    return;
                }
            }
        }

        if left_boundary_a == len_a - 2 {
            let mut divider_a_left = len_a - 2;
            let divider_a_right = len_a - 1;

            let mut divider_b_left = (len_a + len_b) / 2 - (divider_a_left + 1) - 1;
            let mut divider_b_right = divider_b_left + 1;

            if list_a[divider_a_left] <= list_b[divider_b_right]
                && list_b[divider_b_left] <= list_a[divider_a_right]
            {
                let median_left = max(list_a[divider_a_left], list_b[divider_b_left]);
                let median_right = min(list_a[divider_a_right], list_b[divider_b_right]);
                self.median = Some((median_left, median_right));
                return;
            } else {
                divider_a_left = len_a - 1;
                divider_b_right -= 1;

                if len_a == len_b {
                    let median_left = list_a[divider_a_left];
                    let median_right = list_b[divider_b_right];
                    self.median = Some((median_left, median_right));
                    return;
                } else {
                    divider_b_left = divider_b_right - 1;
                    let median_left = max(list_a[divider_a_left], list_b[divider_b_left]);
                    let median_right = list_b[divider_b_right];
                    self.median = Some((median_left, median_right));
                    return;
                }
            }
        }

        let divider_a_left = (right_boundary_a + left_boundary_a) / 2;
        let divider_b_left = (len_a + len_b) / 2 - (divider_a_left + 1) - 1;

        let divider_a_right = divider_a_left + 1;
        let divider_b_right = divider_b_left + 1;

        println!(
            "dAL {}, dBL {}, dAR {}, dBR {}",
            divider_a_left, divider_b_left, divider_a_right, divider_b_right
        );

        if list_a[divider_a_left] > list_b[divider_b_right] {
            right_boundary_a = divider_a_left;
            self.run(left_boundary_a, right_boundary_a)
        } else if list_b[divider_b_left] > list_a[divider_a_right] {
            left_boundary_a = divider_a_left;
            self.run(left_boundary_a, right_boundary_a)
        } else {
            let median_left = max(list_a[divider_a_left], list_b[divider_b_left]);
            let median_right = min(list_a[divider_a_right], list_b[divider_b_right]);
            self.median = Some((median_left, median_right));
            return;
        }
    }

    pub fn init(&mut self) -> &Self {
        let list_a = self.list_a;
        let list_b = self.list_b;
        let len_a = self.len_a;
        let len_b = self.len_b;
        let left_boundary_a: usize = 0;
        let right_boundary_a: usize = len_a - 1;

        if len_a == 1 {
            if len_b == 1 {
                if list_a[0] <= list_b[0] {
                    self.median = Some((list_a[0], list_b[0]));
                } else {
                    self.median = Some((list_b[0], list_a[0]));
                }
            } else {
                let median_b_idx = len_b / 2;

                let mut make_median = [0; 4];

                make_median[0] = list_a[0];
                make_median[1] = list_b[median_b_idx - 1];
                make_median[2] = list_b[median_b_idx];
                make_median[3] = list_b[median_b_idx + 1];

                make_median.sort();

                self.median = Some((make_median[1], make_median[2]));
            }
        } else if len_a == 2 {
            if len_b == 2 {
                let midean_left = max(list_a[0], list_b[0]);
                let midean_right = min(list_a[1], list_b[1]);

                self.median = Some((midean_left, midean_right));
            } else {
                let median_left_b_idx = (len_b / 2) - 1;
                let median_right_b_idx = median_left_b_idx + 1;

                let mut make_median = [0; 6];

                make_median[0] = list_a[0];
                make_median[1] = list_a[1];
                make_median[2] = list_b[median_left_b_idx - 1];
                make_median[3] = list_b[median_left_b_idx];
                make_median[4] = list_b[median_right_b_idx];
                make_median[5] = list_b[median_right_b_idx + 1];

                make_median.sort();

                self.median = Some((make_median[2], make_median[3]));
            }
        } else {
            self.run(left_boundary_a, right_boundary_a);
        }

        if self.list_b.len() != self.len_b {
            let list_b = self.list_b;
            let len_b = list_b.len();
            let unpaired_elem = list_b[len_b - 1];

            let median_left = self.median.unwrap().0;
            let median_right = self.median.unwrap().1;

            if unpaired_elem <= median_left {
                self.median = Some((median_left, median_left));
            } else if unpaired_elem >= median_right {
                self.median = Some((median_right, median_right));
            } else {
                self.median = Some((unpaired_elem, unpaired_elem))
            }
        }

        self
    }

    pub fn result(&self) -> (i32, i32) {
        self.median.unwrap()
    }
}
