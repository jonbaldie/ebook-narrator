use crate::model::{Project, Segment};
use anyhow::{Context, Result, bail};
use roxmltree::Document;
use scraper::{Html, Selector};
use sha2::{Digest, Sha256};
use std::collections::HashMap;
use std::fs::{self, File};
use std::io::{Read, Write};
use std::path::{Path, PathBuf};
use uuid::Uuid;
use zip::write::FileOptions;
use zip::{CompressionMethod, ZipArchive, ZipWriter};

pub fn create_fixture(project_dir: &Path) -> Result<PathBuf> {
    let fixture_dir = project_dir.join("fixture");
    fs::create_dir_all(&fixture_dir)?;
    let path = fixture_dir.join("native-audio-prototype.epub");
    let file = File::create(&path)?;
    let mut zip = ZipWriter::new(file);
    let stored = FileOptions::default().compression_method(CompressionMethod::Stored);
    let deflated = FileOptions::default().compression_method(CompressionMethod::Deflated);

    zip.start_file("mimetype", stored)?;
    zip.write_all(b"application/epub+zip")?;
    zip.start_file("META-INF/container.xml", deflated)?;
    zip.write_all(
        br#"<?xml version="1.0"?>
<container version="1.0" xmlns="urn:oasis:names:tc:opendocument:xmlns:container">
  <rootfiles><rootfile full-path="OEBPS/content.opf" media-type="application/oebps-package+xml"/></rootfiles>
</container>"#,
    )?;
    zip.start_file("OEBPS/content.opf", deflated)?;
    zip.write_all(
        br#"<?xml version="1.0" encoding="UTF-8"?>
<package xmlns="http://www.idpf.org/2007/opf" version="3.0">
  <metadata xmlns:dc="http://purl.org/dc/elements/1.1/"><dc:title>Prototype Fixture</dc:title></metadata>
  <manifest>
    <item id="one" href="chapter-1.xhtml" media-type="application/xhtml+xml"/>
    <item id="two" href="chapter-2.xhtml" media-type="application/xhtml+xml"/>
  </manifest>
  <spine><itemref idref="one"/><itemref idref="two"/></spine>
</package>"#,
    )?;
    zip.start_file("OEBPS/chapter-1.xhtml", deflated)?;
    zip.write_all(br#"<html xmlns="http://www.w3.org/1999/xhtml"><body><h1>First test chapter</h1><p>This text checks the first Teleprompter step.</p></body></html>"#)?;
    zip.start_file("OEBPS/chapter-2.xhtml", deflated)?;
    zip.write_all(br#"<html xmlns="http://www.w3.org/1999/xhtml"><body><h1>Second test chapter</h1><p>This text checks manual movement to the next Narration Segment.</p></body></html>"#)?;
    zip.finish()?;
    Ok(path)
}

pub fn import(path: &Path, project_dir: &Path, project: &mut Project) -> Result<usize> {
    let bytes = fs::read(path).with_context(|| format!("Could not read {}", path.display()))?;
    let hash = format!("{:x}", Sha256::digest(&bytes));
    let file = File::open(path)?;
    let mut archive =
        ZipArchive::new(file).context("The selected file is not a readable EPUB ZIP file")?;

    if archive.by_name("META-INF/encryption.xml").is_ok() {
        bail!("The prototype rejects an EPUB that has an encryption record.");
    }

    let container = read_zip_text(&mut archive, "META-INF/container.xml")?;
    let container_document =
        Document::parse(&container).context("The EPUB container XML is not valid")?;
    let package_path = container_document
        .descendants()
        .find_map(|node| node.attribute("full-path"))
        .context("The EPUB has no package path")?
        .to_string();
    let package = read_zip_text(&mut archive, &package_path)?;
    if package.contains("pre-paginated") {
        bail!("The prototype rejects a fixed-layout EPUB.");
    }
    let package_document =
        Document::parse(&package).context("The EPUB package XML is not valid")?;
    let manifest: HashMap<String, String> = package_document
        .descendants()
        .filter(|node| node.tag_name().name() == "item")
        .filter_map(|node| {
            Some((
                node.attribute("id")?.to_string(),
                node.attribute("href")?.to_string(),
            ))
        })
        .collect();
    let package_parent = Path::new(&package_path)
        .parent()
        .unwrap_or_else(|| Path::new(""));
    let body_selector = Selector::parse("body").expect("The body selector is valid");
    let mut segments = Vec::new();

    for itemref in package_document
        .descendants()
        .filter(|node| node.tag_name().name() == "itemref")
    {
        let idref = itemref
            .attribute("idref")
            .context("A spine item has no idref")?;
        let href = manifest
            .get(idref)
            .with_context(|| format!("The EPUB spine refers to missing item {idref}"))?;
        let source_path = package_parent.join(href);
        let source_name = source_path.to_string_lossy().replace('\\', "/");
        let xhtml = read_zip_text(&mut archive, &source_name)?;
        let html = Html::parse_document(&xhtml);
        let text = html
            .select(&body_selector)
            .flat_map(|body| body.text())
            .map(str::trim)
            .filter(|part| !part.is_empty())
            .collect::<Vec<_>>()
            .join("\n\n");
        if !text.is_empty() {
            segments.push(Segment {
                id: Uuid::new_v4().to_string(),
                source_path: source_name,
                text,
            });
        }
    }
    if segments.is_empty() {
        bail!("The EPUB has no usable reflowable spine text.");
    }

    let source_dir = project_dir.join("source");
    fs::create_dir_all(&source_dir)?;
    fs::write(source_dir.join("book.epub"), bytes)?;
    project.source_epub_name = Some(
        path.file_name()
            .and_then(|name| name.to_str())
            .unwrap_or("book.epub")
            .to_string(),
    );
    project.source_epub_sha256 = Some(hash);
    project.segments = segments;
    project.current_segment_index = 0;
    Ok(project.segments.len())
}

fn read_zip_text(archive: &mut ZipArchive<File>, name: &str) -> Result<String> {
    let mut entry = archive
        .by_name(name)
        .with_context(|| format!("The EPUB is missing {name}"))?;
    if entry.size() > 2 * 1024 * 1024 {
        bail!("The EPUB entry {name} is larger than the prototype limit.");
    }
    let mut text = String::new();
    entry.read_to_string(&mut text)?;
    Ok(text)
}
