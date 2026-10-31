//! Floating charge islands of a pole-zero descriptor.
//!
//! Charge depends on voltage differences, so coordinates that are coupled to
//! one another only through charge form a set whose common mode stores none:
//! every `C` row of the set sums to zero over it. A BJT whose base, internal
//! collector and internal emitter carry only junction and diffusion charge is
//! such a set. The stamped sums round, however, and exact descriptor closure
//! reads that rounding as a genuine state with an enormous finite pole whose
//! eigensolve error then swamps the rest of the spectrum. Re-expressing the
//! set in differences from one of its coordinates makes the common-mode column
//! exactly zero, so closure counts it as the infinite mode it is.
use crate::Value;
use crate::analysis::pole_zero::Matrix;
use crate::engine::Engine;

/// Rounding allowance for one stored coefficient of a row, in units of the
/// machine epsilon; a cancelling row may carry this much per coefficient.
const ROUNDING_ULPS_PER_COEFFICIENT: Value = 64.0;

/// Exact sum of `values` as a nonoverlapping expansion: the returned
/// components add up to the sum without rounding, and none is zero.
fn exact_sum(values: &[Value]) -> Vec<Value> {
    let mut components = Vec::<Value>::new();
    for &value in values {
        let mut carry = value;
        let mut kept = Vec::with_capacity(components.len() + 1);
        for &component in &components {
            let sum = carry + component;
            let virtual_component = sum - carry;
            let error = (carry - (sum - virtual_component)) + (component - virtual_component);
            if error != 0.0 {
                kept.push(error);
            }
            carry = sum;
        }
        if carry != 0.0 {
            kept.push(carry);
        }
        components = kept;
    }
    components
}

/// Sets of coordinates, each sorted and of two or more, that `C` couples only
/// to one another, whose rows sum to zero over the set up to the rounding of
/// their stored coefficients, and in which rounding has left at least one row
/// sum nonzero. A set whose rows cancel exactly is already singular for exact
/// closure and is not returned. `entries` holds each nonzero `C` coefficient
/// once as (row, column, value).
pub(super) fn floating_charge_islands(
    order: usize,
    entries: &[(usize, usize, Value)],
) -> Vec<Vec<usize>> {
    fn root(parent: &mut [usize], mut index: usize) -> usize {
        while parent[index] != index {
            parent[index] = parent[parent[index]];
            index = parent[index];
        }
        index
    }

    let mut parent = (0..order).collect::<Vec<_>>();
    let mut charged = vec![false; order];
    let mut coefficients = vec![Vec::<Value>::new(); order];
    for &(row, col, value) in entries {
        if value == 0.0 || row >= order || col >= order {
            continue;
        }
        charged[row] = true;
        charged[col] = true;
        coefficients[row].push(value);
        let (a, b) = (root(&mut parent, row), root(&mut parent, col));
        if a != b {
            parent[a.max(b)] = a.min(b);
        }
    }

    let mut sets = std::collections::BTreeMap::<usize, Vec<usize>>::new();
    for (index, _) in charged.iter().enumerate().filter(|(_, charged)| **charged) {
        let key = root(&mut parent, index);
        sets.entry(key).or_default().push(index);
    }
    // Per row: whether it cancels up to rounding, and whether exactly.
    let residue = |row: usize| {
        let values = &coefficients[row];
        let exact = exact_sum(values);
        let magnitude = values.iter().map(|value| value.abs()).sum::<Value>();
        let sum = exact.iter().sum::<Value>().abs();
        let cancels = sum
            <= ROUNDING_ULPS_PER_COEFFICIENT * Value::EPSILON * values.len() as Value * magnitude;
        (cancels, exact.is_empty())
    };
    sets.into_values()
        .filter(|set| {
            if set.len() < 2 {
                return false;
            }
            let rows = set.iter().map(|&row| residue(row)).collect::<Vec<_>>();
            rows.iter().all(|&(cancels, _)| cancels) && rows.iter().any(|&(_, exact)| !exact)
        })
        .collect()
}

/// Rewrite every floating charge island of the dense descriptor in
/// differences `d_j = x_j - x_r` from its first coordinate `r`. Each `d_j` is
/// a new algebraic coordinate; the island's charge moves onto the `d_j`
/// columns and its own columns of `C` become exactly zero. The transfer
/// between the original coordinates is unchanged. Returns the number of
/// coordinates appended.
pub(super) fn deflate_floating_charge_islands(g: &mut Matrix, c: &mut Matrix) -> usize {
    let (order, _) = c.dims();
    let mut entries = Vec::new();
    for row in 0..order {
        for col in 0..order {
            let value = c.get(row, col);
            if value != 0.0 {
                entries.push((row, col, value));
            }
        }
    }
    let islands = floating_charge_islands(order, &entries);
    let appended = islands.iter().map(|set| set.len() - 1).sum::<usize>();
    if appended == 0 {
        return 0;
    }
    let mut next = Engine::descriptor_expand_square(g, c, appended);
    for island in &islands {
        let reference = island[0];
        for &coordinate in &island[1..] {
            // Only the island's own rows hold coefficients in its columns.
            for &row in island {
                c.set(row, next, c.get(row, coordinate));
            }
            g.set(next, next, 1.0);
            g.set(next, coordinate, -1.0);
            g.set(next, reference, 1.0);
            next += 1;
        }
        for &row in island {
            for &col in island {
                c.set(row, col, 0.0);
            }
        }
    }
    appended
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn a_rounded_floating_island_is_found_and_a_grounded_set_is_not() {
        // Two capacitors from node 0 to nodes 1 and 2, with the diagonal
        // carrying the rounding a stamped sum leaves behind.
        let (a, b) = (1.0e-13, 2.3e-14);
        let diagonal = (a + b) * (1.0 + 2.0 * Value::EPSILON);
        let floating = [
            (0, 0, diagonal),
            (0, 1, -a),
            (0, 2, -b),
            (1, 0, -a),
            (1, 1, a),
            (2, 0, -b),
            (2, 2, b),
            (3, 3, 1.0e-7),
        ];
        assert_eq!(floating_charge_islands(4, &floating), vec![vec![0, 1, 2]]);

        // Exact cancellation is left to exact closure.
        let mut exact = floating;
        exact[0].2 = 1.0;
        exact[1].2 = -0.5;
        exact[2].2 = -0.5;
        exact[3].2 = -0.5;
        exact[4].2 = 0.5;
        exact[5].2 = -0.5;
        exact[6].2 = 0.5;
        assert!(floating_charge_islands(4, &exact).is_empty());

        // A grounded capacitor far above rounding keeps the set dynamic.
        let mut grounded = floating;
        grounded[0].2 = a + b + 1.0e-18;
        assert!(floating_charge_islands(4, &grounded).is_empty());
    }

    #[test]
    fn deflation_zeroes_the_island_columns_and_keeps_its_differences() {
        let (a, b) = (1.0e-13, 2.3e-14);
        let mut c = Matrix::from_dense(vec![
            vec![(a + b) * (1.0 + 2.0 * Value::EPSILON), -a, -b],
            vec![-a, a, 0.0],
            vec![-b, 0.0, b],
        ]);
        let mut g = Matrix::identity(3);
        assert_eq!(deflate_floating_charge_islands(&mut g, &mut c), 2);
        assert_eq!(c.dims(), (5, 5));
        for row in 0..3 {
            for col in 0..3 {
                assert_eq!(c.get(row, col), 0.0);
            }
        }
        // d_1 = x_1 - x_0 carries the old column 1, d_2 = x_2 - x_0 column 2.
        assert_eq!((c.get(0, 3), c.get(1, 3), c.get(2, 3)), (-a, a, 0.0));
        assert_eq!((c.get(0, 4), c.get(1, 4), c.get(2, 4)), (-b, 0.0, b));
        assert_eq!((g.get(3, 3), g.get(3, 1), g.get(3, 0)), (1.0, -1.0, 1.0));
        assert_eq!((g.get(4, 4), g.get(4, 2), g.get(4, 0)), (1.0, -1.0, 1.0));
    }
}
