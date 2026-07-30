# Issue tracker: GitHub

GitHub Issues store issues and product requirements for this repository. Use the `gh` command for all issue work.

## Commands

- Create an issue: `gh issue create --title "..." --body "..."`.
- Read an issue: `gh issue view <number> --comments`.
- List issues: `gh issue list --state open`.
- Add a comment: `gh issue comment <number> --body "..."`.
- Add or remove a label: `gh issue edit <number> --add-label "..."` or `gh issue edit <number> --remove-label "..."`.
- Close an issue: `gh issue close <number> --comment "..."`.

Run these commands in this clone. The `gh` command gets the repository from `git remote`.

## Pull requests

**Pull requests as a triage source: no.**

Do not add pull requests to the triage queue. Change this value to `yes` if this policy changes.

## Skill actions

When a skill says "publish to the issue tracker", create a GitHub Issue.

When a skill says "get the ticket", run `gh issue view <number> --comments`.

## Wayfinder work

Use one GitHub Issue as a map. Give it the `wayfinder:map` label.

Use child GitHub Issues as tickets. Put `Part of #<map>` at the start of each ticket. Use one type label: `wayfinder:research`, `wayfinder:prototype`, `wayfinder:grilling`, or `wayfinder:task`.

Use a GitHub issue dependency to show a blocker. If dependencies are not available, put `Blocked by: #<number>` at the start of the ticket.

Before work starts, assign the ticket to the current user. When work is complete, add the answer as a comment, close the ticket, and add a context link to the map.
