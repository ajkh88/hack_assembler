use assert_cmd::prelude::*;
use std::process::Command;
use std::fs::{self, File};
use file_diff::{diff_files};

#[test]
fn test_add() -> Result<(), Box<dyn std::error::Error>> {
    test_correct_machine_language("Add")
}

#[test]
fn test_max() -> Result<(), Box<dyn std::error::Error>> {
    test_correct_machine_language("Max")
}

#[test]
fn test_max_l() -> Result<(), Box<dyn std::error::Error>> {
    test_correct_machine_language("MaxL")
}

#[test]
fn test_pong() -> Result<(), Box<dyn std::error::Error>> {
    test_correct_machine_language("Pong")
}

#[test]
fn test_pong_l() -> Result<(), Box<dyn std::error::Error>> {
    test_correct_machine_language("PongL")
}

#[test]
fn test_rect() -> Result<(), Box<dyn std::error::Error>> {
    test_correct_machine_language("Rect")
}

#[test]
fn test_rect_l() -> Result<(), Box<dyn std::error::Error>> {
    test_correct_machine_language("RectL")
}

fn test_correct_machine_language(filename: &str) -> Result<(), Box<dyn std::error::Error>> {
    delete_output_file_if_exists(&format!("tests/files/{}.hack", filename));

    let mut cmd = Command::cargo_bin("hack_assembler")?;

    cmd.arg(format!("tests/files/{}.asm", filename)).assert().success();

    let mut output = File::open(format!("tests/files/{}.hack", filename))?;
    let mut expected = File::open(format!("tests/files/{}_example.hack", filename))?;

    diff_files(&mut expected, &mut output);

    Ok(())
}

fn delete_output_file_if_exists(filename: &str) {
    fs::remove_file(filename).ok();
}