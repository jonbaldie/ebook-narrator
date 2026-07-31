# Narrator Recording Workflow Prototype

This is throwaway UI code for the question in [Prototype Narrator Recording Workflow](https://github.com/jonbaldie/ebook-narrator/issues/4).

`index.html` shows the agreed Gentle workflow from Project start to export. It does not save a Project, read an EPUB file, use a microphone, record audio, or create an export.

Run this command from the repository root:

```sh
python3 -m http.server 8080 --directory prototypes/narrator-recording-workflow
```

Then open <http://localhost:8080>.

The supporting files are:

- `AGREED-WORKFLOW.md` — the agreed workflow, screens, states, and controls
- `simple-options.html` — the three simple options used to select the Gentle guide
- `previous-busy-options.html` — the rejected, busier options

The buttons change in-memory test state only. The prototype label shows the current test state.
