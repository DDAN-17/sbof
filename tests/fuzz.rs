use std::{
    fs,
    io::{ErrorKind, Write, stdout},
    panic::{catch_unwind, resume_unwind},
    time::{Duration, SystemTime},
};

use rand::{prelude::*, random, rng};
use sbof::{de::from_bytes, ser::to_bytes, *};
use serde::*;

#[derive(Serialize, Deserialize, Debug, PartialEq)]
enum TestEnum {
    Unit,
    Newtype(u8),
    Tuple(u8, u16),
    Struct { field1: i32, field2: char },
}

#[derive(Serialize, Deserialize, Debug, PartialEq)]
struct TestStruct {
    character: char,
    unsigned8: u8,
    integer8: i8,
    unsigned16: u16,
    integer16: i16,
    unsigned32: u32,
    integer32: i32,
    unsigned64: u64,
    integer64: i64,
    unsigned128: u128,
    integer128: i128,
    float32: f32,
    float64: f64,
    vector: Vec<u8>,
    tuple: (u8, u16, u8),
    enumeration: TestEnum,
    string: String,
}

impl TestStruct {
    fn random() -> Self {
        let mut rng = rng();
        TestStruct {
            character: rng.random(),
            unsigned8: rng.random(),
            integer8: rng.random(),
            unsigned16: rng.random(),
            integer16: rng.random(),
            unsigned32: rng.random(),
            integer32: rng.random(),
            unsigned64: rng.random(),
            integer64: rng.random(),
            unsigned128: rng.random(),
            integer128: rng.random(),
            float32: rng.random(),
            float64: rng.random(),
            vector: (0..random::<u8>() as usize).map(|_| rng.random()).collect(),
            tuple: (rng.random(), rng.random(), rng.random()),
            enumeration: match random::<u8>() % 4 {
                0 => TestEnum::Unit,
                1 => TestEnum::Newtype(rng.random()),
                2 => TestEnum::Tuple(rng.random(), rng.random()),
                _ => TestEnum::Struct {
                    field1: rng.random(),
                    field2: rng.random(),
                },
            },
            string: (0..random::<u8>() as usize)
                .map(|_| rng.random::<char>())
                .collect(),
        }
    }
}

#[test]
fn run_repeat() -> Result<()> {
    let start = SystemTime::now();
    const QUIT_DURATION: Duration = Duration::from_secs(10);

    loop {
        all()?;
        let duration = SystemTime::now()
            .duration_since(start)
            .expect("system time error");
        if duration >= QUIT_DURATION {
            break;
        }
    }

    Ok(())
}

fn all() -> Result<()> {
    let test_struct = if fs::exists("failed_case.json")? {
        println!("loading failed_case.json...");
        let str = fs::read_to_string("failed_case.json")?;
        let test_struct: TestStruct =
            serde_json::from_str(&str).expect("could not parse failed_case.json");
        println!("done\n");
        test_struct
    } else {
        TestStruct::random()
    };

    let bytes = to_bytes(&test_struct)?;
    println!("serialization complete");
    let unwind = catch_unwind(|| from_bytes::<TestStruct>(&bytes));
    let deser = match unwind {
        Err(e) => {
            failed_case(&test_struct, &bytes)?;
            resume_unwind(e)
        }
        Ok(Err(e)) => {
            failed_case(&test_struct, &bytes)?;
            stdout().flush()?;
            Err(e)?
        }
        Ok(Ok(deser)) => deser,
    };

    if test_struct != deser {
        failed_case_eq(&test_struct, &deser, &bytes)?;
        return Err(Error::Io(std::io::Error::new(
            ErrorKind::InvalidData,
            "failed",
        )));
    } else {
        println!("success!");
        let _ = fs::remove_file("failed_case.bin");
        let _ = fs::remove_file("failed_case.json");
        let _ = fs::remove_file("deser.json");
    }

    Ok(())
}

fn failed_case(test_struct: &TestStruct, bytes: &[u8]) -> Result<()> {
    println!("\n\nfound bad input: {test_struct:?}");
    fs::write(
        "failed_case.json",
        serde_json::to_string_pretty(test_struct).expect("failed to serialize to JSON"),
    )?;
    println!("wrote input to failed_case.json");
    fs::write("failed_case.bin", bytes)?;
    println!("wrote serialized input to failed_case.bin");
    Ok(())
}

fn failed_case_eq(test_struct: &TestStruct, deser: &TestStruct, bytes: &[u8]) -> Result<()> {
    println!("\n\nfound bad input: {test_struct:?}");
    fs::write(
        "failed_case.json",
        serde_json::to_string_pretty(test_struct).expect("failed to serialize to JSON"),
    )?;
    println!("wrote input to failed_case.json");
    fs::write("failed_case.bin", bytes)?;
    println!("wrote serialized input to failed_case.bin");
    fs::write(
        "deser.json",
        serde_json::to_string_pretty(deser).expect("failed to serialize to JSON"),
    )?;
    println!("wrote deserialized output to deser.json");
    println!("use `diff failed_case.json deser.json` to find differences");
    Ok(())
}
