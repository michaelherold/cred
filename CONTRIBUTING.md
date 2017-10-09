# Contributing

In the spirit of [free software](http://www.fsf.org/licensing/essays/free-sw.html), **everyone** is encouraged to help improve this project. Here are some ways *you* can contribute:

* Use alpha, beta, and pre-release versions.
* Report bugs.
* Suggest new features.
* Write or edit documentation.
* Write specifications.
* Write code (**no patch is too small**: fix typos, add comments, clean up inconsistent whitespace).
* Refactor code.
* Fix [issues].
* Review patches.

[issues]: https://github.com/michaelherold/cred/issues

## Submitting an Issue

We use the [GitHub issue tracker][issues] to track bugs and features. Before submitting a bug report or feature request, check to make sure it hasn't already been submitted.

When submitting a bug report, please include a [Gist](https://gist.github.com) that includes a stack trace and any details that may be necessary to reproduce the bug, including your Cargo version, Rust version, and operating system.

Ideally, a bug report should include a pull request with failing specs.

## Submitting a Pull Request

1. [Fork the repository](http://learn.github.com/p/branching.html).
2. [Create a topic branch](https://help.github.com/articles/creating-and-deleting-branches-within-your-repository/).
3. Add tests for your unimplemented feature or bug fix.
4. Run `cargo test`. If your tests pass, return to step 3.
5. Implement your feature or bug fix.
6. Run `[INSERT LINTER HERE]`. If your specs or any of the rules fail, return to step 5.
7. Open `[INSERT COVERAGE REPORT HERE]`. If your changes are not completely covered by your tests, return to step 3.
8. Add documentation for your feature or bug fix.
9. Run `cargo doc`. If your changes are undocumented in the outputted
   documentation, go back to step 8.
10. Commit and push your changes.
11. [Submit a pull request](https://help.github.com/articles/creating-a-pull-request/).

## Tools to Help You Succeed

After checking out the repository, run `bin/setup` to install dependencies. Then, run `cargo test` to run the tests.

When writing code, you can use Mocha’s watcher mode to automatically run tests whenever you modify and save a file. This helps to eliminate the tedium of running tests manually and reduces the chance that you will accidentally forget to run the tests. To use this mode, run `[INSERT AUTO RUNNER HERE]`.

Before committing code, run `[INSERT LINTER HERE]` to check that the code conforms to the style guidelines of the project, that all of the tests are green (if you're writing a feature; if you're only submitting a failing test, then it does not have to pass!), and that the changes are sufficiently documented.
