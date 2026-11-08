# Changelog

## 0.1.1

This is a prototype snapshot, not a production Narration Application.

- A new Project starts with no Takes from the previous Project.
- The workflow prototype has regression tests for Project isolation.
- Cancelling New Project setup can interrupt the previous Project timer. See [issue 26](https://github.com/jonbaldie/ebook-narrator/issues/26).
- The locked Playwright version has a browser download certificate verification vulnerability. See [GHSA-7mvr-c777-76hp](https://github.com/advisories/GHSA-7mvr-c777-76hp).

The limits of the 0.1.0 snapshot still apply. Windows and production release tests remain incomplete.

## 0.1.0

This is a prototype snapshot, not a production Narration Application.

- The workflow prototype shows the agreed Narrator flow with simulated state.
- The native audio prototype tests EPUB fixtures, WAV Takes, Trims, and LAME export.
- The exploratory report records the prototype test results and their limits.

The workflow prototype does not import EPUB files, record audio, save Projects, or export audiobooks. The native audio prototype is a test harness, not a secure EPUB importer or a finished application. Windows and production release tests remain incomplete.
