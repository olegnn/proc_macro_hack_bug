#![recursion_limit="512"]

extern crate proc_macro_hack;
extern crate proc_macro_nested;
extern crate futures;

extern crate proc_macro_hack_bug_impl;

use futures::future::ready;
use futures::join;

use proc_macro_hack::proc_macro_hack;

#[allow(dead_code)]
async fn add_nestded_joined_results() -> usize {
    let results = join!(ready(join!(ready(7usize)).0), ready(join!(ready(8usize)).0), ready(join!(ready(join!(ready(9usize)).0)).0));
    results.0 + results.1 + results.2
}

#[proc_macro_hack(support_nested, internal_macro_calls = 5)]
pub use proc_macro_hack_bug_impl::join_all;

#[allow(dead_code)]
async fn add_nested_custom_joined_results() -> usize {
    let results = join_all!(ready(join_all!(ready(7usize)).0), ready(join_all!(ready(8usize)).0), ready(join_all!(ready(join_all!(ready(9usize)).0)).0));
    results.0 + results.1 + results.2
}

#[proc_macro_hack(support_nested, internal_macro_calls = 60)]
pub use proc_macro_hack_bug_impl::join_all_x2;

#[allow(dead_code)]
async fn add_nested_custom_joined_x2_results() -> usize {
    let results = join_all_x2!(ready(join_all_x2!(ready(7usize)).0), ready(join_all_x2!(ready(8usize)).0), ready(join_all_x2!(ready(join_all_x2!(ready(9usize)).0)).0));
    results.0 + results.1 + results.2
}

#[allow(dead_code)]
async fn add_nested_custom_joined_combined_results() -> usize {
    let results = join_all_x2!(ready(join_all!(ready(7usize)).0), ready(join_all_x2!(ready(8usize)).0), ready(join_all!(ready(join_all_x2!(ready(9usize)).0)).0));
    results.0 + results.1 + results.2
}

#[cfg(test)]
mod tests {
    use super::*;
    use futures::executor::block_on;

    #[test] 
    fn checks_add_nestded_joined_results_function() {
        assert_eq!(block_on(add_nestded_joined_results()), 24);
    }

    #[test] 
    fn checks_add_nested_custom_joined_results_function() {
        assert_eq!(block_on(add_nested_custom_joined_results()), 24);
    }

    #[test] 
    fn checks_add_nested_custom_joined_x2_results() {
        assert_eq!(block_on(add_nested_custom_joined_x2_results()), 24);
    }

    #[test] 
    fn checks_add_nested_custom_joined_combined_results() {
        assert_eq!(block_on(add_nested_custom_joined_combined_results()), 24);
    }
}
