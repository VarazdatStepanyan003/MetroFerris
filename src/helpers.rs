//
// Copyright (c) 2025-2026 Varazdat Stepanyan.
//
// This file is part of MetroFerris, an open-source engine for kinetic
// monte carlo (and beyond) simulations. MetroFerris is free software: you can redistribute
// it and/or modify it under the terms of the GNU General Public License
// version 3 as published by the Free Software Foundation.
//
// MetroFerris is distributed in the hope that it will be useful, but WITHOUT ANY WARRANTY;
// without even the implied warranty of MERCHANTABILITY or FITNESS FOR A PARTICULAR PURPOSE.
// See the GNU General Public License for more details.
//
// You should have received a copy of the GNU General Public License along with MetroFerris.
// If not, see <https://www.gnu.org/licenses/>.
//

// returns the smallest index for which arr[i] < x <= arr[i+1]
pub fn time_search<T: PartialOrd>(x: T, arr: &[T]) -> Option<usize> {
    if arr.len() < 2 || x < arr[0] {
        return None;
    }

    if x == arr[0] {
        return Some(0);
    }

    let mut b: usize = arr.len() - 1;
    if x > arr[b] {
        return None;
    }
    let mut a: usize = 0;
    while a < b {
        let mut m = (a + b) / 2;
        if arr[m + 1] <= x {
            a = m + 1;
        } else if arr[m] > x {
            b = m;
        } else {
            while m > 0 && arr[m - 1] == x {
                m -= 1;
            }
            return Some(m);
        }
    }
    panic!("This should never happen: binary_search")
}

// returns the largest index for which arr[i] <= x < arr[i+1]
pub fn rate_search<T: PartialOrd>(x: T, arr: &[T]) -> Option<usize> {
    if arr.len() < 2 || x < arr[0] {
        return None;
    }

    if x == arr[0] {
        return Some(0);
    }
    let mut b: usize = arr.len() - 1;
    if x > arr[b] {
        return None;
    }
    let mut a: usize = 0;
    while a < b {
        let mut m = (a + b) / 2;
        if arr[m + 1] < x {
            a = m + 1;
        } else if arr[m] >= x {
            b = m;
        } else {
            while m < arr.len() - 1 && arr[m + 1] == x {
                m += 1;
            }
            return Some(m);
        }
    }
    panic!("This should never happen: binary_search")
}
