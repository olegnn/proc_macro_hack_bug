__Run tests:__

```shell
cargo test
```

__Expand macros:__ 

```shell
cargo rustc --profile=check -- -Zunstable-options --pretty=expanded -Z external-macro-backtrace
```

__Tests error:__

```
error[E0659]: `proc_macro_call_0` is ambiguous (macro-expanded name vs less macro-expanded name from outer scope during import/macro resolution)
  --> src/lib.rs:17:19
   |
17 |     let results = join_all!(ready(7), ready(8), ready(9));
   |                   ^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^
   |                   |
   |                   ambiguous name
   |                   in this macro invocation
   |
note: `proc_macro_call_0` could refer to the macro defined here
  --> src/lib.rs:17:19
   |
17 |     let results = join_all!(ready(7), ready(8), ready(9));
   |                   ^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^ in this macro invocation
note: `proc_macro_call_0` could also refer to the macro defined here
  --> src/lib.rs:12:1
   |
12 | / #[proc_macro_hack(support_nested)]
13 | | pub use proc_macro_hack_bug_impl::join_all;
   | |__________________________________________^
...
17 |       let results = join_all!(ready(7), ready(8), ready(9));
   |                     --------------------------------------- in this macro invocation
   = note: this error originates in a macro outside of the current crate (in Nightly builds, run with -Z external-macro-backtrace for more info)

error[E0659]: `proc_macro_call_0` is ambiguous (macro-expanded name vs less macro-expanded name from outer scope during import/macro resolution)
  --> src/lib.rs:17:19
   |
17 |     let results = join_all!(ready(7), ready(8), ready(9));
   |                   ^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^
   |                   |
   |                   ambiguous name
   |                   in this macro invocation
   |
note: `proc_macro_call_0` could refer to the macro defined here
  --> src/lib.rs:17:19
   |
17 |     let results = join_all!(ready(7), ready(8), ready(9));
   |                   ^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^ in this macro invocation
note: `proc_macro_call_0` could also refer to the macro defined here
  --> src/lib.rs:12:1
   |
12 | / #[proc_macro_hack(support_nested)]
13 | | pub use proc_macro_hack_bug_impl::join_all;
   | |__________________________________________^
...
17 |       let results = join_all!(ready(7), ready(8), ready(9));
   |                     --------------------------------------- in this macro invocation
   = note: this error originates in a macro outside of the current crate (in Nightly builds, run with -Z external-macro-backtrace for more info)

error: aborting due to previous error

For more information about this error, try `rustc --explain E0659`.
error: could not compile `proc_macro_hack_bug`.
warning: build failed, waiting for other jobs to finish...
error: aborting due to previous error

For more information about this error, try `rustc --explain E0659`.
error: could not compile `proc_macro_hack_bug`.

To learn more, run the command again with --verbose.
```

__Expanded macros error:__
```
error[E0659]: `proc_macro_call_0` is ambiguous (macro-expanded name vs less macro-expanded name from outer scope during import/macro resolution)
   --> <::proc_macro_nested::count macros>:1:9
    |
1   |                        () => { proc_macro_call_0 ! () } ; (!) => { proc_macro_call_1 ! () } ; (! !)
    |                        -       ^^^^^^^^^^^^^^^^^-----
    |                        |       |
    |                        |       ambiguous name
    |    ____________________|       in this macro invocation (#10)
    |   |
2   |   |                    => { proc_macro_call_2 ! () } ; (! ! !) => { proc_macro_call_3 ! () } ;
3   |   |                    (! ! ! !) => { proc_macro_call_4 ! () } ; (! ! ! ! !) =>
4   |   |                    { proc_macro_call_5 ! () } ; (! ! ! ! ! !) => { proc_macro_call_6 ! () } ;
...     |
104 |   |                     ! ! ! ! ! ! ! ! ! ! ! ! ! ! ! ! ! ! ! ! ! ! ! ! !) =>
105 |   |                    { proc_macro_call_64 ! () } ;
    |   |                                                -
    |   |                                                |
    |   |________________________________________________in this expansion of `$crate::count!` (#9)
    |                                                    in this expansion of `$crate::count!` (#22)
    |
   ::: src/lib.rs:12:1
    |
12  |                        #[proc_macro_hack(support_nested)]
    |                        ----------------------------------
    |                        |
    |                        in this expansion of `join_all!` (#1)
    |  ______________________in this macro invocation (#2)
    | |______________________|
    | |
13  | |                      pub use proc_macro_hack_bug_impl::join_all;
    | |                                                               -
    | |_______________________________________________________________|
    | |_______________________________________________________________in this expansion of `proc_macro_call_0!` (#10)
    |                                                                 in this macro invocation (#11)
...
17  | |                          let results = join_all!(ready(7), ready(8), ready(9));
    | |                                        --------------------------------------- in this macro invocation (#1)
    |
   ::: <::proc_macro_nested::dispatch macros>:1:29
    |
1   |                        (() $ ($ bang : tt) *) => { $ crate :: count ! ($ ($ bang) *) } ;
    |                        -                           ---------------------------------
    |                        |                           |
    |                        |                           in this macro invocation (#9)
    |      __________________|                           in this macro invocation (#22)
    |     |
2   |     |                  ((($ ($ first : tt) *) $ ($ rest : tt) *) $ ($ bang : tt) *) =>
3   |     |                  { $ crate :: dispatch ! (($ ($ first) * $ ($ rest) *) $ ($ bang) *) } ;
    |     |                    -----------------------------------------------------------------
    |     |                    |
    |     |                    in this macro invocation (#3)
    |     |                    in this macro invocation (#5)
    |     |                    in this macro invocation (#7)
    |     |                    in this macro invocation (#14)
    |     |                    in this macro invocation (#16)
    |     |                    in this macro invocation (#18)
    |     |                    in this macro invocation (#20)
4   |     |                  (([$ ($ first : tt) *] $ ($ rest : tt) *) $ ($ bang : tt) *) =>
...       |
12  |     |                  (($ first : tt $ ($ rest : tt) *) $ ($ bang : tt) *) =>
13  |     |                  { $ crate :: dispatch ! (($ ($ rest) *) $ ($ bang) *) } ;
    |     |                    ---------------------------------------------------   -
    |     |                    |                                                     |
    |     |                    |                                                     in this expansion of `$crate::proc_macro_call_join_all!` (#2)
    |     |                    |                                                     in this expansion of `$crate::dispatch!` (#3)
    |     |                    |                                                     in this expansion of `$crate::dispatch!` (#4)
    |     |                    |                                                     in this expansion of `$crate::dispatch!` (#5)
    |     |                    |                                                     in this expansion of `$crate::dispatch!` (#6)
    |     |                    |                                                     in this expansion of `$crate::dispatch!` (#7)
    |     |                    |                                                     in this expansion of `$crate::dispatch!` (#8)
    |     |                    |                                                     in this expansion of `$crate::proc_macro_call_join!` (#13)
    |     |                    |                                                     in this expansion of `$crate::dispatch!` (#14)
    |     |                    |                                                     in this expansion of `$crate::dispatch!` (#15)
    |     |                    |                                                     in this expansion of `$crate::dispatch!` (#16)
    |     |                    |                                                     in this expansion of `$crate::dispatch!` (#17)
    |     |                    |                                                     in this expansion of `$crate::dispatch!` (#18)
    |     |                    |                                                     in this expansion of `$crate::dispatch!` (#19)
    |     |____________________|_____________________________________________________in this expansion of `$crate::dispatch!` (#20)
    |                          |                                                     in this expansion of `$crate::dispatch!` (#21)
    |                          in this macro invocation (#4)
    |                          in this macro invocation (#6)
    |                          in this macro invocation (#8)
    |                          in this macro invocation (#15)
    |                          in this macro invocation (#17)
    |                          in this macro invocation (#19)
    |                          in this macro invocation (#21)
    |
   ::: <::futures::join macros>:1:1
    |
1   |                      / ($ ($ tokens : tt) *) =>
2   |                      | {
3   |                      |     $ crate :: inner_macro :: join !
    |  ____________________|_____-
4   | |                    |     { futures_crate_path (:: futures) $ ($ tokens) * }
    | |____________________|______________________________________________________- in this macro invocation (#12)
5   |                      | }
    |                      |_- in this expansion of `::futures::join!` (#11)
    |
   ::: <::futures_util::async_await::join_mod::join macros>:1:1
    |
1   |                     /  ($ ($ proc_macro : tt) *) =>
2   |                     |  {
3   |                     |      {
4   |                     |          # [derive ($ crate :: proc_macro_hack_join)] enum ProcMacroHack
5   |                     |          { Nested = (stringify ! { $ ($ proc_macro) * }, 0) . 1, } $ crate ::
    |  ___________________|____________________________________________________________________-
6   | |                   |          proc_macro_call_join ! { ($ ($ proc_macro) *) }
    | |___________________|________________________________________________________- in this macro invocation (#13)
7   |                     |      }
8   |                     |  } ;
    |                     |____- in this expansion of `$crate::inner_macro::join!` (#12)
    |
note: `proc_macro_call_0` could refer to the macro defined here
    |
   ::: src/lib.rs:12:1
    |
12  |             #[proc_macro_hack(support_nested)]
    |             ----------------------------------
    |             |
    |             in this expansion of `join_all!` (#1)
    |  ___________in this macro invocation (#2)
    | |___________|
    | |
13  | |           pub use proc_macro_hack_bug_impl::join_all;
    | |                                                    -
    | |____________________________________________________|
    | |____________________________________________________in this expansion of `proc_macro_call_0!` (#10)
    |                                                      in this macro invocation (#11)
...
17  | |               let results = join_all!(ready(7), ready(8), ready(9));
    | |                             --------------------------------------- in this macro invocation (#1)
    |
   ::: <::proc_macro_nested::dispatch macros>:1:29
    |
1   |             (() $ ($ bang : tt) *) => { $ crate :: count ! ($ ($ bang) *) } ;
    |             -                           --------------------------------- in this macro invocation (#9)
    |    _________|
    |   |
2   |   |         ((($ ($ first : tt) *) $ ($ rest : tt) *) $ ($ bang : tt) *) =>
3   |   |         { $ crate :: dispatch ! (($ ($ first) * $ ($ rest) *) $ ($ bang) *) } ;
    |   |           -----------------------------------------------------------------
    |   |           |
    |   |           in this macro invocation (#3)
    |   |           in this macro invocation (#5)
    |   |           in this macro invocation (#7)
4   |   |         (([$ ($ first : tt) *] $ ($ rest : tt) *) $ ($ bang : tt) *) =>
...     |
12  |   |         (($ first : tt $ ($ rest : tt) *) $ ($ bang : tt) *) =>
13  |   |         { $ crate :: dispatch ! (($ ($ rest) *) $ ($ bang) *) } ;
    |   |           ---------------------------------------------------   -
    |   |           |                                                     |
    |   |           |                                                     in this expansion of `$crate::proc_macro_call_join_all!` (#2)
    |   |           |                                                     in this expansion of `$crate::dispatch!` (#3)
    |   |           |                                                     in this expansion of `$crate::dispatch!` (#4)
    |   |           |                                                     in this expansion of `$crate::dispatch!` (#5)
    |   |           |                                                     in this expansion of `$crate::dispatch!` (#6)
    |   |___________|_____________________________________________________in this expansion of `$crate::dispatch!` (#7)
    |               |                                                     in this expansion of `$crate::dispatch!` (#8)
    |               in this macro invocation (#4)
    |               in this macro invocation (#6)
    |               in this macro invocation (#8)
    |
   ::: <::proc_macro_nested::count macros>:1:9
    |
1   |             () => { proc_macro_call_0 ! () } ; (!) => { proc_macro_call_1 ! () } ; (! !)
    |             -       ---------------------- in this macro invocation (#10)
    |   __________|
    |  |
2   |  |          => { proc_macro_call_2 ! () } ; (! ! !) => { proc_macro_call_3 ! () } ;
3   |  |          (! ! ! !) => { proc_macro_call_4 ! () } ; (! ! ! ! !) =>
4   |  |          { proc_macro_call_5 ! () } ; (! ! ! ! ! !) => { proc_macro_call_6 ! () } ;
...    |
104 |  |           ! ! ! ! ! ! ! ! ! ! ! ! ! ! ! ! ! ! ! ! ! ! ! ! !) =>
105 |  |          { proc_macro_call_64 ! () } ;
    |  |______________________________________- in this expansion of `$crate::count!` (#9)
    |
   ::: /Users/oleg/.cargo/registry/src/github.com-1ecc6299db9ec823/futures-join-macro-preview-0.3.0-alpha.19/src/lib.rs:78:1
    |
78  |             #[proc_macro_hack]
    |             ------------------ in this expansion of `#[derive($crate::proc_macro_hack_join)]` (#13)
   --> <::futures_util::async_await::join_mod::join macros>:4:20
    |
1   |          /  ($ ($ proc_macro : tt) *) =>
2   |          |  {
3   |          |      {
4   |          |          # [derive ($ crate :: proc_macro_hack_join)] enum ProcMacroHack
    |          |                     ^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^ in this macro invocation (#13)
...            |
7   |          |      }
8   |          |  } ;
    |          |____- in this expansion of `$crate::inner_macro::join!` (#12)
    |
   ::: <::futures::join macros>:1:1
    |
1   |           / ($ ($ tokens : tt) *) =>
2   |           | {
3   |           |     $ crate :: inner_macro :: join !
    |  _________|_____-
4   | |         |     { futures_crate_path (:: futures) $ ($ tokens) * }
    | |_________|______________________________________________________- in this macro invocation (#12)
5   |           | }
    |           |_- in this expansion of `::futures::join!` (#11)
note: `proc_macro_call_0` could also refer to the macro defined here
   --> src/lib.rs:12:1
    |
12  |   #[proc_macro_hack(support_nested)]
    |   ^---------------------------------
    |   |
    |  _in this expansion of `join_all!`
    | |
13  | | pub use proc_macro_hack_bug_impl::join_all;
    | |                                          ^ in this macro invocation
    | |__________________________________________|
    |
...
17  |       let results = join_all!(ready(7), ready(8), ready(9));
    |                     --------------------------------------- in this macro invocation
    |
   ::: /Users/oleg/Documents/macro_dev/proc_macro_hack_bug/proc_macro_hack_bug_impl/src/lib.rs:41:1
    |
41  |   #[proc_macro_hack]
    |   ------------------ in this expansion of `#[derive($crate::proc_macro_hack_join_all)]`

#![feature(prelude_import)]
#[prelude_import]
use std::prelude::v1::*;
#[macro_use]
extern crate std;
extern crate proc_macro_hack;
extern crate proc_macro_nested;
extern crate futures;

extern crate proc_macro_hack_bug_impl;

use futures::future::ready;
use futures::join;

use proc_macro_hack::proc_macro_hack;

enum _24proc_macro_hack_bug_impl_8join_all {
    Value =
        ("# [doc (hidden)] pub use proc_macro_hack_bug_impl :: proc_macro_hack_join_all\n; # [doc (hidden)] pub use proc_macro_nested :: dispatch as\nproc_macro_call_join_all ; # [macro_export] macro_rules ! join_all\n{\n    ($ ($ proc_macro : tt) *) =>\n    {\n        {\n            # [derive ($ crate :: proc_macro_hack_join_all)] enum\n            ProcMacroHack\n            { Nested = (stringify ! { $ ($ proc_macro) * }, 0) . 1, } $ crate\n            :: proc_macro_call_join_all ! { ($ ($ proc_macro) *) }\n        }\n    } ;\n}",
         0).1,
}
#[doc(hidden)]
pub use proc_macro_hack_bug_impl::proc_macro_hack_join_all;
#[doc(hidden)]
pub use proc_macro_nested::dispatch as proc_macro_call_join_all;
#[macro_export]
macro_rules! join_all {
    ($ ($ proc_macro : tt) *) =>
    {
        {
            # [derive ($ crate :: proc_macro_hack_join_all)] enum
            ProcMacroHack
            { Nested = (stringify ! { $ ($ proc_macro) * }, 0) . 1, } $ crate
            :: proc_macro_call_join_all ! { ($ ($ proc_macro) *) }
        }
    } ;
}

#[allow(dead_code)]
async fn add_custom_joined_results() -> usize {
    let results =
        {
            enum ProcMacroHack {
                Nested = ("ready (7), ready (8), ready (9)", 0).1,
            }
            macro_rules! proc_macro_call_0 {
                () =>
                { :: futures :: join ! (ready (7), ready (8), ready (9)) }
            }




            {
                enum ProcMacroHack {
                    Nested =
                        ("futures_crate_path (:: futures) ready (7), ready (8), ready (9)",
                         0).1,
                }
                macro_rules! proc_macro_call_0 {
                    () =>
                    {
                        {
                            let mut _fut0 = :: futures :: future :: maybe_done
                            (ready (7)) ; let mut _fut1 = :: futures :: future
                            :: maybe_done (ready (8)) ; let mut _fut2 = ::
                            futures :: future :: maybe_done (ready (9)) ; ::
                            futures :: future :: poll_fn
                            (move | __cx : & mut :: futures :: task :: Context
                             < '_ > |
                             {
                                 let mut __all_done = true ; __all_done &= ::
                                 futures :: core_reexport :: future :: Future
                                 :: poll
                                 (unsafe
                                  {
                                      :: futures :: core_reexport :: pin ::
                                      Pin :: new_unchecked (& mut _fut0)
                                  }, __cx) . is_ready () ; __all_done &= ::
                                 futures :: core_reexport :: future :: Future
                                 :: poll
                                 (unsafe
                                  {
                                      :: futures :: core_reexport :: pin ::
                                      Pin :: new_unchecked (& mut _fut1)
                                  }, __cx) . is_ready () ; __all_done &= ::
                                 futures :: core_reexport :: future :: Future
                                 :: poll
                                 (unsafe
                                  {
                                      :: futures :: core_reexport :: pin ::
                                      Pin :: new_unchecked (& mut _fut2)
                                  }, __cx) . is_ready () ; if __all_done
                                 {
                                     :: futures :: core_reexport :: task ::
                                     Poll :: Ready
                                     ((unsafe
                                       {
                                           :: futures :: core_reexport :: pin
                                           :: Pin :: new_unchecked
                                           (& mut _fut0)
                                       } . take_output () . unwrap (), unsafe
                                       {
                                           :: futures :: core_reexport :: pin
                                           :: Pin :: new_unchecked
                                           (& mut _fut1)
                                       } . take_output () . unwrap (), unsafe
                                       {
                                           :: futures :: core_reexport :: pin
                                           :: Pin :: new_unchecked
                                           (& mut _fut2)
                                       } . take_output () . unwrap (),))
                                 } else
                                 {
                                     :: futures :: core_reexport :: task ::
                                     Poll :: Pending
                                 }
                             }) . await
                        }
                    }
                }
                {
                    let mut _fut0 = ::futures::future::maybe_done(ready(7));
                    let mut _fut1 = ::futures::future::maybe_done(ready(8));
                    let mut _fut2 = ::futures::future::maybe_done(ready(9));
                    ::futures::future::poll_fn(move
                                                   |__cx:
                                                        &mut ::futures::task::Context<'_>|
                                                   {
                                                       let mut __all_done =
                                                           true;
                                                       __all_done &=
                                                           ::futures::core_reexport::future::Future::poll(unsafe
                                                                                                          {
                                                                                                              ::futures::core_reexport::pin::Pin::new_unchecked(&mut _fut0)
                                                                                                          },
                                                                                                          __cx).is_ready();
                                                       __all_done &=
                                                           ::futures::core_reexport::future::Future::poll(unsafe
                                                                                                          {
                                                                                                              ::futures::core_reexport::pin::Pin::new_unchecked(&mut _fut1)
                                                                                                          },
                                                                                                          __cx).is_ready();
                                                       __all_done &=
                                                           ::futures::core_reexport::future::Future::poll(unsafe
                                                                                                          {
                                                                                                              ::futures::core_reexport::pin::Pin::new_unchecked(&mut _fut2)
                                                                                                          },
                                                                                                          __cx).is_ready();
                                                       if __all_done {
                                                           ::futures::core_reexport::task::Poll::Ready((unsafe
                                                                                                        {
                                                                                                            ::futures::core_reexport::pin::Pin::new_unchecked(&mut _fut0)
                                                                                                        }.take_output().unwrap(),
                                                                                                        unsafe
                                                                                                        {
                                                                                                            ::futures::core_reexport::pin::Pin::new_unchecked(&mut _fut1)
                                                                                                        }.take_output().unwrap(),
                                                                                                        unsafe
                                                                                                        {
                                                                                                            ::futures::core_reexport::pin::Pin::new_unchecked(&mut _fut2)
                                                                                                        }.take_output().unwrap()))
                                                       } else {
                                                           ::futures::core_reexport::task::Poll::Pending
                                                       }
                                                   }).await
                }
            }
        };
    results.0 + results.1 + results.2
}
#[allow(dead_code)]
async fn add_joined_results() -> usize {
    let results =
        {
            enum ProcMacroHack {
                Nested =
                    ("futures_crate_path (:: futures) ready (7), ready (8), ready (9)",
                     0).1,
            }
            macro_rules! proc_macro_call_0 {
                () =>
                {
                    {
                        let mut _fut0 = :: futures :: future :: maybe_done
                        (ready (7)) ; let mut _fut1 = :: futures :: future ::
                        maybe_done (ready (8)) ; let mut _fut2 = :: futures ::
                        future :: maybe_done (ready (9)) ; :: futures ::
                        future :: poll_fn
                        (move | __cx : & mut :: futures :: task :: Context <
                         '_ > |
                         {
                             let mut __all_done = true ; __all_done &= ::
                             futures :: core_reexport :: future :: Future ::
                             poll
                             (unsafe
                              {
                                  :: futures :: core_reexport :: pin :: Pin ::
                                  new_unchecked (& mut _fut0)
                              }, __cx) . is_ready () ; __all_done &= ::
                             futures :: core_reexport :: future :: Future ::
                             poll
                             (unsafe
                              {
                                  :: futures :: core_reexport :: pin :: Pin ::
                                  new_unchecked (& mut _fut1)
                              }, __cx) . is_ready () ; __all_done &= ::
                             futures :: core_reexport :: future :: Future ::
                             poll
                             (unsafe
                              {
                                  :: futures :: core_reexport :: pin :: Pin ::
                                  new_unchecked (& mut _fut2)
                              }, __cx) . is_ready () ; if __all_done
                             {
                                 :: futures :: core_reexport :: task :: Poll
                                 :: Ready
                                 ((unsafe
                                   {
                                       :: futures :: core_reexport :: pin ::
                                       Pin :: new_unchecked (& mut _fut0)
                                   } . take_output () . unwrap (), unsafe
                                   {
                                       :: futures :: core_reexport :: pin ::
                                       Pin :: new_unchecked (& mut _fut1)
                                   } . take_output () . unwrap (), unsafe
                                   {
                                       :: futures :: core_reexport :: pin ::
                                       Pin :: new_unchecked (& mut _fut2)
                                   } . take_output () . unwrap (),))
                             } else
                             {
                                 :: futures :: core_reexport :: task :: Poll
                                 :: Pending
                             }
                         }) . await
                    }
                }
            }
            {
                let mut _fut0 = ::futures::future::maybe_done(ready(7));
                let mut _fut1 = ::futures::future::maybe_done(ready(8));
                let mut _fut2 = ::futures::future::maybe_done(ready(9));
                ::futures::future::poll_fn(move
                                               |__cx:
                                                    &mut ::futures::task::Context<'_>|
                                               {
                                                   let mut __all_done = true;
                                                   __all_done &=
                                                       ::futures::core_reexport::future::Future::poll(unsafe
                                                                                                      {
                                                                                                          ::futures::core_reexport::pin::Pin::new_unchecked(&mut _fut0)
                                                                                                      },
                                                                                                      __cx).is_ready();
                                                   __all_done &=
                                                       ::futures::core_reexport::future::Future::poll(unsafe
                                                                                                      {
                                                                                                          ::futures::core_reexport::pin::Pin::new_unchecked(&mut _fut1)
                                                                                                      },
                                                                                                      __cx).is_ready();
                                                   __all_done &=
                                                       ::futures::core_reexport::future::Future::poll(unsafe
                                                                                                      {
                                                                                                          ::futures::core_reexport::pin::Pin::new_unchecked(&mut _fut2)
                                                                                                      },
                                                                                                      __cx).is_ready();
                                                   if __all_done {
                                                       ::futures::core_reexport::task::Poll::Ready((unsafe
                                                                                                    {
                                                                                                        ::futures::core_reexport::pin::Pin::new_unchecked(&mut _fut0)
                                                                                                    }.take_output().unwrap(),
                                                                                                    unsafe
                                                                                                    {
                                                                                                        ::futures::core_reexport::pin::Pin::new_unchecked(&mut _fut1)
                                                                                                    }.take_output().unwrap(),
                                                                                                    unsafe
                                                                                                    {
                                                                                                        ::futures::core_reexport::pin::Pin::new_unchecked(&mut _fut2)
                                                                                                    }.take_output().unwrap()))
                                                   } else {
                                                       ::futures::core_reexport::task::Poll::Pending
                                                   }
                                               }).await
            }
        };
    results.0 + results.1 + results.2
}
error: aborting due to previous error

For more information about this error, try `rustc --explain E0659`.
error: could not compile `proc_macro_hack_bug`.

To learn more, run the command again with --verbose.
``` 

