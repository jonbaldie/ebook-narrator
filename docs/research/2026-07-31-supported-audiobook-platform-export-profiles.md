# Research: Supported Audiobook Platform Export Profiles

**Research date:** 2026-07-31  
**Question:** Which audiobook hosting platforms must the first release support with ACX? What source-audio, exported-audio, metadata, and file-package rules apply?

## Result

Support these two export profiles in the first release:

1. **ACX.** ACX is a direct service for rights holders. It accepts completed audiobook audio and sends approved titles to Audible, Amazon, and Apple Books. [ACX: Learn about ACX](https://help.acx.com/s/article/what-is-acx) [ACX: Submit audiobook for quality review](https://help.acx.com/s/article/what-happens-during-audio-review)
2. **Spotify for Authors.** A self-published author can publish an audiobook direct to Spotify. Spotify states that this distribution is non-exclusive. [Spotify for Authors: Get started](https://authors.spotify.com/get-started) [Spotify: Uploading audiobooks](https://support.spotify.com/ke-en/authors/article/uploading-audiobooks/)

Do not add a separate Apple Books profile in the first release. ACX sends a title that passes its quality review to Apple Books. [ACX: Submit audiobook for quality review](https://help.acx.com/s/article/what-happens-during-audio-review)

Do not add a wide-distribution profile in the first release. Spotify has an optional Voices by INaudio route for other retailers. That route needs a later platform review. [Spotify for Authors: Direct audiobook publishing](https://authors.spotify.com/blog/direct-audiobook-publishing)

## Terms and evidence limit

In this report, **source audio** is the saved audio before export from a Project. **Exported audio** is a file that the Narration Application gives to a platform.

The reviewed ACX and Spotify documents state delivery rules. They state no rule for source-audio container, bit depth, or sample rate. Do not make a source-audio format a platform rule without a separate Project decision. [ACX: Audio submission requirements](https://help.acx.com/s/article/what-are-the-acx-audio-submission-requirements) [Spotify for Authors: Metadata and asset guide](https://support.spotifycdn.com/pdf/SFA%20Metadata_Asset%20Guide_2024.pdf)

The Spotify guide is dated November 2024. Spotify links to it from its current upload help page. Check it again before a production release. [Spotify: Uploading audiobooks](https://support.spotify.com/ke-en/authors/article/uploading-audiobooks/) [Spotify for Authors: Metadata and asset guide](https://support.spotifycdn.com/pdf/SFA%20Metadata_Asset%20Guide_2024.pdf)

## First-release audio profile

Create an **ACX-compatible MP3** profile. Use it for ACX and Spotify for Authors.

| Item | First-release profile | Evidence and status |
| --- | --- | --- |
| File format | MP3, 44.1 kHz, at least 192 kbps, CBR | ACX requires this. Spotify accepts it and states 16-bit MP3. **Fixed.** [ACX: Audio submission requirements](https://help.acx.com/s/article/what-are-the-acx-audio-submission-requirements) [Spotify guide](https://support.spotifycdn.com/pdf/SFA%20Metadata_Asset%20Guide_2024.pdf) |
| Mean loudness | -23 dB RMS to -18 dB RMS | ACX requires this range. It is inside Spotify's recommended -24 dB to -14 dB range. **Fixed for this profile.** [ACX: Audio submission requirements](https://help.acx.com/s/article/what-are-the-acx-audio-submission-requirements) [Spotify guide](https://support.spotifycdn.com/pdf/SFA%20Metadata_Asset%20Guide_2024.pdf) |
| Peak level | No higher than -3 dB | ACX requires this. Spotify gives no peak-level rule in its guide. **Fixed by ACX.** [ACX: Audio submission requirements](https://help.acx.com/s/article/what-are-the-acx-audio-submission-requirements) |
| Noise floor | Less than -60 dB RMS | Spotify requires less than -60 dB. ACX allows no higher than -60 dB. The lower value meets both. **Fixed for this profile.** [ACX: Audio submission requirements](https://help.acx.com/s/article/what-are-the-acx-audio-submission-requirements) [Spotify guide](https://support.spotifycdn.com/pdf/SFA%20Metadata_Asset%20Guide_2024.pdf) |
| Channel mode | All mono or all stereo | Both platforms allow mono or stereo. Neither allows a mixture in one audiobook. **The mode is a human choice.** [ACX: Audio submission requirements](https://help.acx.com/s/article/what-are-the-acx-audio-submission-requirements) [Spotify guide](https://support.spotifycdn.com/pdf/SFA%20Metadata_Asset%20Guide_2024.pdf) |
| Start and end room tone | One second at the start; one to five seconds at the end | ACX recommends one to five seconds at both ends and limits room tone to five seconds. Spotify recommends 0.5 to one second at the start and one to five seconds at the end. **This is the shared recommended value.** [ACX: Audio submission requirements](https://help.acx.com/s/article/what-are-the-acx-audio-submission-requirements) [Spotify guide](https://support.spotifycdn.com/pdf/SFA%20Metadata_Asset%20Guide_2024.pdf) |
| File duration and division | One chapter or section in each file. Keep each file at or below 120 minutes. | Both platforms state this rule. Split a longer chapter and add a continuation header. **Fixed.** [ACX: Audio submission requirements](https://help.acx.com/s/article/what-are-the-acx-audio-submission-requirements) [Spotify guide](https://support.spotifycdn.com/pdf/SFA%20Metadata_Asset%20Guide_2024.pdf) |
| Spoken chapter indication | State the chapter or section header in each file. | ACX requires a section header. Spotify requires a verbal indication of a new chapter. **Fixed.** [ACX: Audio submission requirements](https://help.acx.com/s/article/what-are-the-acx-audio-submission-requirements) [Spotify guide](https://support.spotifycdn.com/pdf/SFA%20Metadata_Asset%20Guide_2024.pdf) |
| Narration type | Human narration | ACX prohibits unapproved text-to-speech, AI, and automated narration. **Fixed by ACX.** [ACX: Audio submission requirements](https://help.acx.com/s/article/what-are-the-acx-audio-submission-requirements) |

Spotify also accepts 44.1 kHz, 16-bit WAV and FLAC files. Its guide prefers FLAC level 5 or higher. These are valid Spotify alternatives, but they do not meet the ACX MP3 delivery rule. Do not add them as first-release profiles. [Spotify guide](https://support.spotifycdn.com/pdf/SFA%20Metadata_Asset%20Guide_2024.pdf)

ACX accepts 256 kbps and 320 kbps CBR MP3 files. The 192 kbps value is the first-release minimum. [ACX: Audio submission requirements](https://help.acx.com/s/article/what-are-the-acx-audio-submission-requirements)

## ACX profile

### Source audio

ACX gives no source-audio rule. Its audio rules apply to the final uploaded MP3 files. [ACX: Audio submission requirements](https://help.acx.com/s/article/what-are-the-acx-audio-submission-requirements)

The ACX 15-minute checkpoint must be fully edited and show the final audio quality. It must show voices, accents, pace, tone, and pitch. This is a review asset, not a source-audio format rule. [ACX: Record a 15-minute sample](https://help.acx.com/s/article/record-a-15-minute-sample)

### Exported audio

Use the ACX-compatible MP3 profile in the table above. ACX also requires consistent sound, no outtakes, and no distracting noise such as clicks, pops, plosives, and excessive mouth noise. [ACX: Audio submission requirements](https://help.acx.com/s/article/what-are-the-acx-audio-submission-requirements)

### Metadata and cover art

The Narrator must claim a title with the Amazon Kindle ebook ASIN. ACX then requires Title details, Chapter names, and Distribution details before posting the title. [ACX: Claim my title](https://help.acx.com/s/article/claim-my-title)

Opening credits must state the title, author or authors, and Narrator. This text must match the title metadata and cover art. Closing credits must clearly state that the audiobook has ended. [ACX: Audio submission requirements](https://help.acx.com/s/article/what-are-the-acx-audio-submission-requirements)

ACX chapter-name data can contain at most 250 characters. It does not allow bold text, italic text, or HTML. [ACX: Edit chapter titles](https://help.acx.com/s/article/edit-chapter-titles)

The product description can contain at most 2,000 characters, including spaces. The title, subtitle, author, and series information on the cover must match the related metadata as closely as possible. [ACX: Set up a title description](https://help.acx.com/s/article/acx-audiobook-profile)

ACX cover art must be square, JPG, PNG, or TIF, at least 2,400 by 2,400 pixels, at least 72 dpi, 24-bit or higher, RGB, and no more than 8 MB. It must clearly show the title and author name or names. [ACX: Cover art requirements](https://help.acx.com/s/article/cover-art-requirements)

### File package

Upload each chapter or section as an individual file. Upload opening credits and closing credits as separate files. Use only standard US letters and numbers in file names. Include the chapter or section title. [ACX: Audio submission requirements](https://help.acx.com/s/article/what-are-the-acx-audio-submission-requirements)

For normal audio upload, ACX states individual file upload. It states no ZIP archive package rule. [ACX: Audio submission requirements](https://help.acx.com/s/article/what-are-the-acx-audio-submission-requirements)

Add a retail sample no longer than five minutes. It must come from the audiobook and contain no explicit material. ACX prefers the first five minutes, but allows another suitable part. Apple Books uses the first five minutes. [ACX: Audio submission requirements](https://help.acx.com/s/article/what-are-the-acx-audio-submission-requirements)

## Spotify for Authors profile

### Source audio

Spotify for Authors gives no source-audio rule. Its public guide specifies files that the author uploads. [Spotify: Uploading audiobooks](https://support.spotify.com/ke-en/authors/article/uploading-audiobooks/) [Spotify guide](https://support.spotifycdn.com/pdf/SFA%20Metadata_Asset%20Guide_2024.pdf)

### Exported audio

Use the shared ACX-compatible MP3 profile. Spotify accepts MP3, WAV, and FLAC. Its detailed guide gives the MP3, WAV, FLAC, level, channel, RMS, noise-floor, and room-tone values in the first-release table. [Spotify: Uploading audiobooks](https://support.spotify.com/ke-en/authors/article/uploading-audiobooks/) [Spotify guide](https://support.spotifycdn.com/pdf/SFA%20Metadata_Asset%20Guide_2024.pdf)

### Metadata and cover art

Spotify requires a payment profile before publishing. The publisher field is required and becomes locked after title submission. [Spotify: Uploading audiobooks](https://support.spotify.com/ke-en/authors/article/uploading-audiobooks/) [Spotify guide](https://support.spotifycdn.com/pdf/SFA%20Metadata_Asset%20Guide_2024.pdf)

Title and subtitle together must contain fewer than 256 characters. The title description must contain fewer than 2,000 characters, including formatting HTML. [Spotify guide](https://support.spotifycdn.com/pdf/SFA%20Metadata_Asset%20Guide_2024.pdf)

Set author or authors, Narrator or Narrators, and a translator when applicable. Set the abridgement value, language, distribution territories, fiction or non-fiction value, grade level, BISAC category or categories, and keywords. Spotify permits at most three BISAC categories. [Spotify guide](https://support.spotifycdn.com/pdf/SFA%20Metadata_Asset%20Guide_2024.pdf)

Set a unique 13-digit audiobook ISBN for retail and library distribution. Do not reuse a print or ebook ISBN. The guide states that Apple and Google retail distribution requires the 13-digit ISBN. [Spotify guide](https://support.spotifycdn.com/pdf/SFA%20Metadata_Asset%20Guide_2024.pdf)

Set the original release date, any strict on-sale date, source copyright owner and year, audio copyright owner and year, and the USD price. [Spotify guide](https://support.spotifycdn.com/pdf/SFA%20Metadata_Asset%20Guide_2024.pdf)

Spotify requires square cover art. Its upload page accepts PNG or JPEG and recommends 3,000 by 3,000 pixels. Its detailed guide prefers 3,000 by 3,000 pixel, 300 dpi, 24-bit PNG art. It requires title, author, and applicable series text. [Spotify: Uploading audiobooks](https://support.spotify.com/ke-en/authors/article/uploading-audiobooks/) [Spotify guide](https://support.spotifycdn.com/pdf/SFA%20Metadata_Asset%20Guide_2024.pdf)

### File package

Spotify requires separate files for chapters, credits, and introductions. A chapter file cannot exceed 120 minutes. A book without chapters must use 30- to 120-minute sections. [Spotify guide](https://support.spotifycdn.com/pdf/SFA%20Metadata_Asset%20Guide_2024.pdf)

Opening credits are required. They can contain only the title and author or Narrator names. Put a preface or other introduction in a separate file. [Spotify guide](https://support.spotifycdn.com/pdf/SFA%20Metadata_Asset%20Guide_2024.pdf)

The guide has two different statements for closing credits. One statement says that closing credits are not required when no track follows the last chapter. The later closing-credits section says that one separate closing-credit file is required. Treat closing credits as a human review item. [Spotify guide](https://support.spotifycdn.com/pdf/SFA%20Metadata_Asset%20Guide_2024.pdf)

Add a retail sample no longer than five minutes. It must contain no music, credits, or explicit material. Name it `ISBN_sample.mp3`. Spotify generates a sample from a chapter if no sample is supplied. [Spotify guide](https://support.spotifycdn.com/pdf/SFA%20Metadata_Asset%20Guide_2024.pdf)

Spotify recommends a file name like `ISBN_track-number.mp3` and uses the upload-page order as the published order. ACX restricts file names to letters and numbers. Therefore, create separate names for each platform or use only letters and numbers in the shared name. [Spotify guide](https://support.spotifycdn.com/pdf/SFA%20Metadata_Asset%20Guide_2024.pdf) [ACX: Audio submission requirements](https://help.acx.com/s/article/what-are-the-acx-audio-submission-requirements)

Spotify states upload of audio files through its service. Its public documents state no ZIP archive package rule. [Spotify: Uploading audiobooks](https://support.spotify.com/ke-en/authors/article/uploading-audiobooks/) [Spotify guide](https://support.spotifycdn.com/pdf/SFA%20Metadata_Asset%20Guide_2024.pdf)

## Fixed and varying requirements

**Fixed for the first release:** ACX and Spotify for Authors; an ACX-compatible 44.1 kHz, 192 kbps or higher CBR MP3 export; one consistent channel mode per audiobook; separate audio files; a maximum 120-minute file duration; title, author, and Narrator credit data; and platform-specific metadata and cover-art assets. [ACX: Audio submission requirements](https://help.acx.com/s/article/what-are-the-acx-audio-submission-requirements) [Spotify: Uploading audiobooks](https://support.spotify.com/ke-en/authors/article/uploading-audiobooks/) [Spotify guide](https://support.spotifycdn.com/pdf/SFA%20Metadata_Asset%20Guide_2024.pdf)

**Varying by Project or platform:** mono or stereo; 192, 256, or 320 kbps ACX MP3; optional Spotify WAV or FLAC exports; titles, contributors, description, ISBN, territories, category, keywords, rights, price, and release dates; sample selection; file names; and the split of a chapter longer than 120 minutes. [ACX: Audio submission requirements](https://help.acx.com/s/article/what-are-the-acx-audio-submission-requirements) [Spotify guide](https://support.spotifycdn.com/pdf/SFA%20Metadata_Asset%20Guide_2024.pdf)

## Human review items

- Confirm that the Narrator or rights holder owns the audiobook rights. ACX also requires an Amazon Kindle ebook that is for sale and a valid ebook ASIN. [ACX: Claim my title](https://help.acx.com/s/article/claim-my-title)
- Confirm ACX account eligibility. ACX limits accounts to residents of the United States, United Kingdom, Canada, and Ireland with a local address, tax number, and banking details. [ACX: Create an ACX profile](https://help.acx.com/s/article/create-an-acx-account)
- Select mono or stereo for the Project. Check that all exported files use that one mode. [ACX: Audio submission requirements](https://help.acx.com/s/article/what-are-the-acx-audio-submission-requirements) [Spotify guide](https://support.spotifycdn.com/pdf/SFA%20Metadata_Asset%20Guide_2024.pdf)
- Check RMS, peak, noise floor, duration, sample rate, bit rate, CBR, room tone, and file count. Use ACX Audio Lab for its automated checks. It does not detect the noise floor or editing errors. [ACX: Check your audio quality](https://help.acx.com/s/article/acx-audio-analysis-tool-faq-s)
- Listen for clicks, pops, plosives, mouth noise, outtakes, uneven tone, background noise, wrong pronunciation, wrong file order, and incorrect chapter headers. [ACX: Audio submission requirements](https://help.acx.com/s/article/what-are-the-acx-audio-submission-requirements) [Spotify guide](https://support.spotifycdn.com/pdf/SFA%20Metadata_Asset%20Guide_2024.pdf)
- Check that opening credits, closing credits, title metadata, contributors, chapter names, and cover text agree. Check the Spotify closing-credit rule in the live upload flow because its guide has conflicting statements. [ACX: Audio submission requirements](https://help.acx.com/s/article/what-are-the-acx-audio-submission-requirements) [Spotify guide](https://support.spotifycdn.com/pdf/SFA%20Metadata_Asset%20Guide_2024.pdf)
- Select a retail sample that follows each platform rule. The ACX sample can contain no explicit material. The Spotify sample can contain no music, credits, or explicit material. [ACX: Audio submission requirements](https://help.acx.com/s/article/what-are-the-acx-audio-submission-requirements) [Spotify guide](https://support.spotifycdn.com/pdf/SFA%20Metadata_Asset%20Guide_2024.pdf)
- Confirm human narration for ACX. [ACX: Audio submission requirements](https://help.acx.com/s/article/what-are-the-acx-audio-submission-requirements)
- Check the current platform pages before release. Platforms can change file and metadata rules. [ACX: Audio submission requirements](https://help.acx.com/s/article/what-are-the-acx-audio-submission-requirements) [Spotify: Uploading audiobooks](https://support.spotify.com/ke-en/authors/article/uploading-audiobooks/)
