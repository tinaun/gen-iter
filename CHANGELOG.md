# version 0.4.1
* tests refactorings, now it's 100% test coverage
  - unit tests in 'src/' just use `core::*`
  - integration tests in 'tests/' can use `std::*`
  - move 'impl_generator' tests to 'tests/'
  - add test for `#[derive(Debug)]` in 'impl_generator' tests

# version 0.4.0
* incompatible refactorings for `GenIterReturn`:
  - rename `GenIterReturn::is_done()` to `GenIterReturn::is_complete()`
  - rename `GenIterReturn::return_or_self()` to `GenIterReturn::try_get_return()`
* add tests for feature `generator_clone`
* add tests those do not use closure but use `impl Generator`

# version 0.3
* made the crate no_std compatible (#5)
* added struct GenIterReturn and macro gen_iter_return! to iterate over a generator and get the return value (#6)

# version 0.2.1
* added `move` varient of gen-iter (#4)

# version 0.2
* updated to latest rust generator syntax

# version 0.1.2
* impl `From<Generator>` for GenIter
* added `gen_iter!` convienence macro

# version 0.1.1
* fixed documentation link

# version 0.1.0
* inital release
