# Audiobook Narration

This context defines a desktop application for a person to narrate an audiobook from an EPUB file.

## Language

**Narrator**:
A person who uses the Narration Application to record an audiobook on one computer.
_Avoid_: User, reader

**Narration Application**:
A local desktop application that helps a Narrator read an EPUB file and record spoken audio.
_Avoid_: App, teleprompter

**Project**:
The local saved work for one audiobook, including its EPUB file, Narration Segments, Takes, and export settings.
_Avoid_: Session, job

**Project Copy**:
A complete copy of a Project in a location that the Narrator selects. The original Project stays unchanged.
_Avoid_: Partial copy, moved Project

**Teleprompter**:
A view of EPUB text that the Narrator moves with buttons or keyboard keys while recording.
_Avoid_: Scrolling text, automatic prompt

**Teleprompter Display Settings**:
Text size, line spacing, and light or dark color theme controls for the Teleprompter. The Narration Application saves these settings on one computer and uses them for all Projects; a Project Copy does not contain them.
_Avoid_: Teleprompter settings in a Project, fixed display, default text

**Narration Segment**:
A small, ordered part of a chapter that has its own recorded audio.
_Avoid_: Clip, text section

**Take**:
One recorded audio version of a Narration Segment. A Narrator can make another Take without loss of an earlier Take.
_Avoid_: Replacement, overwrite

**Recovered Take**:
A Take that the Narration Application keeps after a recording stops unexpectedly. The Narration Application marks it as recovered and does not select it automatically.
_Avoid_: Selected Take, complete Take

**Trim**:
A change to the start or end time of a selected Take. Trim does not change the source recording.
_Avoid_: Full audio edit, waveform edit

**Export Preparation**:
Changes that the Narration Application makes to copies of selected Takes before Final Review and export. The Narration Application measures all selected Takes and shows one recommended `Fix audio levels` action when necessary. This action corrects loudness and peak level together with fixed safe target values. It corrects only the Takes that need a change and gives short previews of the largest changes. An individual `Fix audio levels` action is a secondary action when the Narrator reviews one Take. The Narrator can keep or undo the corrections, but cannot set technical audio values. Export Preparation does not change a Take.
_Avoid_: Export profile, technical audio controls

**Final Review**:
A guided review in which the Narrator listens to every prepared, selected Take at least once before export. The Narration Application shows one Take at a time, saves review progress, and lets the Narrator stop and continue later. If the Narrator changes or records a Take, the Narration Application checks that Take again and returns it to Final Review.
_Avoid_: Forced review after each recording, sample-only review

**Platform Export**:
An upload-ready folder of audiobook audio files that the Narration Application creates for one selected platform. Each Platform Export has its own date and does not replace an earlier Platform Export.
_Avoid_: Shared export, export report
