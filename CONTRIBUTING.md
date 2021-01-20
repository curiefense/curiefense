We welcome contributions from the community. Please read the following guidelines carefully to
maximize the chances of your PR being merged.

# Communication

* **Before starting work on a major feature**, please reach out to us via [GitHub Discussions](https://github.com/curiefense/curiefense/discussions/categories/q-a)], [Twitter DM](https://twitter.com/curiefense),
  or [email](community@curiefense.io. We will make sure no one else is already working on it and ask you to open a
  GitHub issue.

* Small patches and bug fixes don't need prior communication.


# Submitting a Pull Request (PR)

* Fork the repo.

* Create your PR. 

* Tests will automatically run for you.

* We will **NOT** merge any PR that is not passing tests.

* PRs are expected to have 100% test coverage for added code. This can be verified with a coverage
  build. If your PR cannot have 100% coverage for some reason please clearly explain why when you
  open it.

* Your PR title should be descriptive, and generally start with a subsystem name followed by a
  colon. Examples:
  * "docs: fix grammar error"
  * "http conn man: add new feature"

* Your PR commit message will be used as the commit message when your PR is merged. You should
  update this field if your PR diverges during review.

* When all of the tests are passing and all other conditions described herein are satisfied, a
  maintainer will be assigned to review and merge the PR.



--

This was adapted from Envoy