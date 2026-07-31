# Agreed Gentle Workflow

This file records the agreed result of [Prototype Narrator Recording Workflow](https://github.com/jonbaldie/ebook-narrator/issues/4).

## Gentle mantra

Show one clear task at a time. Use calm and inviting language. Hide technical detail. Show secondary actions only when the Narrator needs them.

## Main workflow

1. The start screen gives **Continue [book title]** as its main action when a saved Project exists. It also gives **New Project** and access to earlier Projects.
2. A new Project starts with **Choose your EPUB file**. The Narration Application uses the book title as the Project name. The Narrator can change it later.
3. The Narration Application imports the EPUB file and creates Narration Segments automatically. It shows **Your book is ready**. It shows technical information only if import fails.
4. Microphone setup uses the usual system microphone. The main action is **Test microphone**. **Choose another microphone** is a secondary action.
5. The microphone test records and plays a short sample. The screen asks **Can you hear yourself clearly?** and gives **Yes, continue** and **Try again**.
6. The Gentle guide shows the current text and one clear recording task. **Record** starts a quiet three-second count. During recording, **Stop** is the main action and **Pause** is secondary. **Resume** replaces Pause when needed.
7. Stop saves the new Take without opening review. A small **Saved** message appears and then disappears. The default path continues to the next Narration Segment.
8. **My recordings** appears after the first Take exists. It opens review on a separate screen. Review is optional while recording. Final review is required before export.
9. **My recordings** starts with **Review next recording**. **Browse all recordings** is secondary.
10. A new Take becomes the selected Take automatically. Earlier Takes remain safe under **Other recordings**.
11. **Record again** returns to the related text and uses the same three-second count.
12. Trim stays hidden until the Narrator selects **Adjust the start or end**. The Trim screen shows one audio bar with two movable ends, **Play**, and **Done**. It shows no other audio editing tools.
13. When recording is complete, the Gentle guide shows **Your audiobook is ready to check**. It presents final checks one at a time.
14. After final review, the export screen gives one main action: **Export audiobook**. The Narration Application applies the agreed audio settings automatically. Success shows **Export complete** and **Open folder**.

## Required screens

- Project start
- EPUB selection
- EPUB import result
- Microphone test
- Microphone test result
- Gentle recording guide
- My recordings
- Recording review
- Adjust start or end
- Final review
- Export
- Export result

## Important states

- No Project
- Saved Project ready to continue
- EPUB import in progress, complete, or failed
- Microphone not tested, testing, accepted, or failed
- Ready to record, counting down, recording, paused, or saved
- Narration Segment not recorded, recorded, or selected for review
- Take selected, earlier Take available, or Trim set
- Recording incomplete, ready for final review, ready to export, exporting, or exported

## Main controls

- Continue [book title]
- New Project
- Choose your EPUB file
- Continue
- Test microphone
- Choose another microphone
- Yes, continue
- Try again
- Record
- Pause
- Resume
- Stop
- Previous text
- Next text
- My recordings
- Review next recording
- Browse all recordings
- Play
- Record again
- Other recordings
- Adjust the start or end
- Done
- Export audiobook
- Open folder
