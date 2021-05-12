# Release Process

This release process is heavily inspired by
[Envoy's](https://github.com/envoyproxy/envoy/blob/main/RELEASES.md) release
process. We have made a few changes to adapt to the size of our community, and
the current state of the project.

## Active development

Active development is happening on the `main` branch, and a new version is released from it
at the end of each quarter.

## Stable releases

Stable releases of Curiefense include:

* Security fixes backported from the `main` branch (including those deemed not worthy
  of creating a CVE).
* Stability fixes backported from the `main` branch (anything that can result in a crash,
  including crashes triggered by a trusted control plane).
* Bugfixes, deemed worthwhile by the maintainers of stable releases.

### Hand-off

Hand-off to the maintainers of stable releases happens after Curiefense maintainers release a new
version from the `main` branch by creating a `vX.Y.0` tag and a corresponding `release/vX.Y`
branch, with merge permissions given to the release manager of stable releases, and CI configured
to execute tests on it.

### Security releases

Critical security fixes are owned by the Curiefense security team, which provides fixes for the
`main` branch, and the latest release branch. Once those fixes are ready, the maintainers
of stable releases backport them to the remaining supported stable releases.

### Backports

All other security and reliability fixes can be nominated for backporting to stable releases by
Curiefense maintainers, Curiefense security team, the change author, or members of the Curiefense
community by adding the `backport/review` or `backport/approved` label. Changes nominated by the
change author and/or members of the Curiefense community are evaluated for backporting on a
case-by-case basis, and require approval from either the release manager of stable release,
Curiefense maintainers, or Curiefense security team. Once approved, those fixes are backported from
the `main` branch to all supported stable branches by the maintainers of stable releases. New stable
versions from non-critical security fixes are released on a regular schedule, initially aiming for
the bi-weekly releases.

### Release management

Release managers of stable releases are responsible for approving and merging backports, tagging
stable releases and sending announcements about them.

| Quarter |       Release manager                                          |
|:-------:|:--------------------------------------------------------------:|
| 2021 Q1 | Justin Dorfman  ([jdorfman](https://github.com/jdorfman))      |
| 2021 Q2 | Justin Dorfman  ([jdorfman](https://github.com/jdorfman))      |

## Release schedule

In order to accommodate downstream projects, new Curiefense releases are produced on a fixed release
schedule (at the end of each quarter), with an acceptable delay of up to 2 weeks, with a hard
deadline of 3 weeks.

| Version |  Expected  |   Actual   | Difference | End of Life |
|:-------:|:----------:|:----------:|:----------:|:-----------:|
| [1.3.0](https://github.com/curiefense/curiefense/releases/tag/v1.3.0)   | 2021/03/02 | 2021/03/02 |  +0 days   |     TBD     |
| [1.4.0](https://github.com/curiefense/curiefense/milestone/3)   | 2021/06/30 |            |            |             |
