# Agreed Automatic Narration Segment Boundaries

This file records the agreed result of [Prototype Automatic Narration Segment Boundaries](https://github.com/jonbaldie/ebook-narrator/issues/11).

## Gentle mantra

Show one clear task at a time. Use calm and inviting language. Hide technical detail. Show secondary actions only when the Narrator needs them.

## Automatic creation rules

The Narration Application creates Narration Segments automatically after the Narrator chooses an EPUB file.

1. A new chapter always starts a new Narration Segment.
2. A major heading inside a chapter always starts a new Narration Segment.
3. Paragraphs pack into the current Narration Segment until the next paragraph would push the part past about 180 words.
4. A paragraph never splits in the middle.
5. A leftover shorter than about 40 words joins the previous Narration Segment in the same section.
6. Opening credits, chapters, and closing credits stay separate sections.

## When the Narrator acts on boundaries

1. **Import stays quiet.** After EPUB import, the Narration Application shows only **Your book is ready**. It does not open a boundary editor on the happy path.
2. **Accept is the default.** Opening the Project accepts the automatic Narration Segments. The Narrator does not confirm each boundary.
3. **Join and split stay secondary.** During recording or review, the current part may offer **Combine with next part** and, when a part holds more than one paragraph, **Split after this paragraph**.
4. **No technical language.** Screens say **part**, not “segment,” “boundary,” “heading level,” or required word counts. Word counts may appear only as quiet help.
5. **Export still follows chapters.** A Platform Export uses the chapter or section structure from the book. Small Narration Segments shape recording work only.

## Join and split behaviour

- **Combine with next part** joins the current Narration Segment with the next Narration Segment only when both belong to the same chapter or section.
- Parts from different chapters stay separate.
- **Split after this paragraph** is available only when the current part holds more than one paragraph. Each side of the split must keep at least one paragraph.

## Interactive prototype

The throwaway demo that supported this decision is `index.html` in this folder.
