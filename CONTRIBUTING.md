Hey there fellow open source contributor! Please read the following guidelines below carefully to maximize the chances of your PR being merged. ⤵️

# Communication

* **Before starting work on a major feature**, please reach out to us via [GitHub Issues](https://github.com/curiefense/curiefense/issues/new?assignees=&labels=&template=feature_request.md&title=) or [ Discussions](https://github.com/curiefense/curiefense/discussions/categories/q-a). We will make sure no one else is already working on it.

* Small patches and bug fixes don't need prior communication.

# Contributing With Maintenance

One important fact of curiefense is that you don't have to be "a maintainer" to
be able to contribute with reviews, triaging, or any other maintenance task.
The curiefense community welcomes everyone willing to help with reviews,
testing open PRs, and providing feedback of any sort.

Please, do join the rest of the team in triaging issues, reviewing PRs, and
providing any feedback that would help making the community stronger, and
processes easier for everyone.

## Triaging Issues

Here you can find some guidelines about Curiefense's triaging process. We will present them in the
form of bullet points for the sake of simplicity. All the points presented here are equally
important:

* All issues should either belong to a milestone or be labeled as backlog. Issues without one of
these attributes will be considered untriaged. The `backlog` label and the milestone attribute are
mutually exclusive.

* Issues labeled as `backlog` will be reviewed at  the beginning of the development cycle (after
every release) or during a backlog review meeting.

* Every issue that is actively being worked on should have, at most, one assignee. Conversely, there
should not be anyone assigned to an issue that is not being worked on.


# Contributing With Code

Curiefense follows a very simple process that is referred to as the [GitHub
Flow](https://guides.github.com/introduction/flow/). The goal for this flow is
to provide the least amount of steps and friction to contribute to curiefense.
Here is the flow described in bullet points:

1. Fork the repo
1. Create a branch
1. Create your PR


At this point, you will see the following happening on your PR:

* Tests will automatically run for you.

* Reviewers will be assigned to the PR either automatically or by one of the
  curiefense maintainers.

In order to make interaction simpler, save some time, and don't waste precious
CPU cycles, it is important that the following things are checked before
reviewers start looking at the PRs:

* PRs are expected to have 100% test coverage for added code. This can be verified with a coverage
  build. If your PR cannot have 100% coverage for some reason please clearly explain why when you
  open it.

* Your PR title should be descriptive, Examples:
  * `Bump axios from 0.19.2 to 0.21.1 in /curiefense/ui`
  * `Add run-curieconf-client-tests.yml`
  * `Update getting started documentation`

> ProTip: check out "[How to Write a Git Commit Message](https://chris.beams.io/posts/git-commit/)"

* Your PR commit message will be used as the commit message when your PR is merged. You should
  update this field if your PR diverges during review.

* When all of the tests are passing and all other conditions described herein are satisfied, [a
  maintainer](https://github.com/curiefense/curiefense/graphs/contributors) will be assigned to review and merge the PR.

<br>

---

Adapted from [Envoy's CONTRIBUTING.md](https://github.com/envoyproxy/envoy/blob/main/CONTRIBUTING.md)
