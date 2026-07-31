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

**Teleprompter**:
A view of EPUB text that the Narrator moves with buttons or keyboard keys while recording.
_Avoid_: Scrolling text, automatic prompt

**Teleprompter Display Settings**:
Local controls for the text size, line spacing, and light or dark color theme of the Teleprompter.
_Avoid_: Fixed display, default text

**Narration Segment**:
A small, ordered part of a chapter that has its own recorded audio.
_Avoid_: Clip, text section

**Take**:
One recorded audio version of a Narration Segment. A Narrator can make another Take without loss of an earlier Take.
_Avoid_: Replacement, overwrite

**Trim**:
A change to the start or end time of a selected Take. Trim does not change the source recording.
_Avoid_: Full audio edit, waveform edit
