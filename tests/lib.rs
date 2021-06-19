extern crate libarchive;

pub mod util;

use std::error::Error;
use std::io::BufReader;
use std::fs::File;
use libarchive::archive::{self, ReadFilter, ReadFormat};
use libarchive::reader::{self, Reader};
use libarchive::writer;

pub type TestResult = Result<(), Box<dyn Error>>;

#[test]
fn reading_from_file() -> TestResult {
    let tar = util::path::fixture("sample.tar.gz")?;
    let mut builder = reader::Builder::new();
    builder.support_format(ReadFormat::All)?;
    builder.support_filter(ReadFilter::All)?;
    let mut reader = builder.open_file(tar)?;
    reader.next_header();
    // let entry: &archive::Entry = &dyn reader.entry;
    // println!("{:?}", entry.pathname());
    // println!("{:?}", entry.size());
    // for entry in reader.entries() {
    //     let file = entry as &dyn archive::Entry;
    //     println!("{:?}", file.pathname());
    //     println!("{:?}", file.size());
    // }
    Ok(())
}

#[test]
fn read_archive_from_stream() -> TestResult {
    // set up the fs stream
    let tar = util::path::fixture("sample.tar.gz")?;
    let f = File::open(tar)?;
    let reader = BufReader::new(f);
    // set up the reader using `open_stream`
    let mut builder = reader::Builder::new();
    builder.support_format(ReadFormat::All)?;
    builder.support_filter(ReadFilter::All)?;
    let mut reader = builder.open_stream(reader)?;
    assert_eq!(reader.header_position(), 0);
    // extract the archive to a temp folder, consuming the stream
    let writer = writer::Disk::new();
    let tmp = tempfile::tempdir()?;
    let tmp_str = tmp.path().to_str().unwrap();
    let bytes_written = writer.write(&mut reader, Some(tmp_str))?;
    assert_eq!(bytes_written, 14);
    assert_eq!(reader.header_position(), 1024);
    Ok(())
}

#[test]
fn extracting_from_file() -> TestResult {
    let tar = util::path::fixture("sample.tar.gz")?;
    let mut builder = reader::Builder::new();
    builder.support_format(ReadFormat::All)?;
    builder.support_filter(ReadFilter::All)?;
    let mut reader = builder.open_file(tar)?;
    println!("{:?}", reader.header_position());
    let writer = writer::Disk::new();
    let tmp = tempfile::tempdir()?;
    let tmp_str = tmp.path().to_str().unwrap();
    writer.write(&mut reader, Some(tmp_str))?;
    println!("{:?}", reader.header_position());
    Ok(())
}

#[test]
fn extracting_an_archive_with_options() -> TestResult {
    let tar = util::path::fixture("sample.tar.gz")?;
    let mut builder = reader::Builder::new();
    builder.support_format(ReadFormat::All)?;
    builder.support_filter(ReadFilter::All)?;
    let mut reader = builder.open_file(tar)?;
    println!("{:?}", reader.header_position());
    let mut opts = archive::ExtractOptions::new();
    opts.add(archive::ExtractOption::Time);
    let writer = writer::Disk::new();
    writer.set_options(&opts)?;
    let tmp = tempfile::tempdir()?;
    let tmp_str = tmp.path().to_str().unwrap();
    writer.write(&mut reader, Some(tmp_str))?;
    println!("{:?}", reader.header_position());
    Ok(())
}

#[test]
fn extracting_a_reader_twice() -> TestResult {
    let tar = util::path::fixture("sample.tar.gz")?;
    let mut builder = reader::Builder::new();
    builder.support_format(ReadFormat::All)?;
    builder.support_filter(ReadFilter::All)?;
    let mut reader = builder.open_file(tar)?;
    println!("{:?}", reader.header_position());
    let writer = writer::Disk::new();
    {
        let tmp = tempfile::tempdir()?;
        let tmp_str = tmp.path().to_str().unwrap();
        writer.write(&mut reader, Some(tmp_str))?;
    }
    println!("{:?}", reader.header_position());
    // try to extract again
    {
        let tmp = tempfile::tempdir()?;
        let tmp_str = tmp.path().to_str().unwrap();
        let result = writer.write(&mut reader, Some(tmp_str));
        assert!(result.is_err(), "extracted from reader twice");
    }
    Ok(())
}
