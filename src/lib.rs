use num::Integer;
use rayon::prelude::*;
use serde::{Deserialize, Serialize};
use std::fmt::Display;
use tracing::{debug, info};

/// Structure to hold results of even/odd checks
#[derive(Serialize, Deserialize, Debug, PartialEq)]
pub struct NumberCheck {
    pub number: i32,
    pub is_even: bool,
    pub is_odd: bool,
}

/// Checks if a number is even.
///
/// # Arguments
/// * `n` - A number implementing the Integer and Display traits.
///
/// # Returns
/// * `true` if the number is even, `false` otherwise.
///
/// # Examples
/// ```
/// use pido_rs::is_even;
/// assert_eq!(is_even(2), true);
/// assert_eq!(is_even(3), false);
/// ```
pub fn is_even<T: Integer + Display>(n: T) -> bool {
    debug!("Checking if {} is even", n);
    n.is_even()
}

/// Checks if a number is odd.
///
/// # Arguments
/// * `n` - A number implementing the Integer and Display traits.
///
/// # Returns
/// * `true` if the number is odd, `false` otherwise.
///
/// # Examples
/// ```
/// use pido_rs::is_odd;
/// assert_eq!(is_odd(3), true);
/// assert_eq!(is_odd(2), false);
/// ```
pub fn is_odd<T: Integer + Display>(n: T) -> bool {
    debug!("Checking if {} is odd", n);
    !n.is_even()
}

/// Parallel check for even/odd on a vector of numbers.
///
/// # Arguments
/// * `numbers` - A slice of i32 numbers to check.
///
/// # Returns
/// * A vector of NumberCheck structs containing results.
pub fn parallel_check(numbers: &[i32]) -> Vec<NumberCheck> {
    info!("Starting parallel check for {} numbers", numbers.len());
    numbers
        .par_iter()
        .map(|&n| NumberCheck {
            number: n,
            is_even: is_even(n),
            is_odd: is_odd(n),
        })
        .collect()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_is_even() {
        assert_eq!(is_even(0i32), true);
        assert_eq!(is_even(2i32), true);
        assert_eq!(is_even(-4i32), true);
        assert_eq!(is_even(3i32), false);
        assert_eq!(is_even(-7i32), false);
    }

    #[test]
    fn test_is_odd() {
        assert_eq!(is_odd(1i32), true);
        assert_eq!(is_odd(-3i32), true);
        assert_eq!(is_odd(0i32), false);
        assert_eq!(is_odd(4i32), false);
        assert_eq!(is_odd(-2i32), false);
    }

    #[test]
    fn test_parallel_check() {
        let numbers = vec![1, 2, 3, 4];
        let results = parallel_check(&numbers);
        assert_eq!(results.len(), 4);
        assert_eq!(
            results[0],
            NumberCheck {
                number: 1,
                is_even: false,
                is_odd: true
            }
        );
        assert_eq!(
            results[1],
            NumberCheck {
                number: 2,
                is_even: true,
                is_odd: false
            }
        );
    }
}
