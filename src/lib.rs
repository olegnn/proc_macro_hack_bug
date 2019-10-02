extern crate proc_macro_hack;
extern crate proc_macro_nested;
extern crate futures;

extern crate proc_macro_hack_bug_impl;

use futures::future::ready;
use futures::join;

use proc_macro_hack::proc_macro_hack;

#[proc_macro_hack(support_nested)]
pub use proc_macro_hack_bug_impl::join_all;

#[allow(dead_code)]
async fn add_custom_joined_results() -> usize {
    let results = join_all!(ready(7), ready(8), ready(9));
    results.0 + results.1 + results.2
}

#[allow(dead_code)]
async fn add_joined_results() -> usize {
    let results = join!(ready(7), ready(8), ready(9));
    results.0 + results.1 + results.2
}

#[cfg(test)]
mod tests {
    use super::*;
    use futures::executor::block_on;

    #[test] 
    fn checks_add_custom_joined_results_function() {
        assert_eq!(block_on(add_custom_joined_results()), 24);
    }

    #[test] 
    fn checks_add_joined_results_function() {
        assert_eq!(block_on(add_joined_results()), 24);
    }
}