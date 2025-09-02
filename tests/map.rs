use std::{
    collections::HashMap,
    fs,
    io::{self, ErrorKind, Write},
    panic::{catch_unwind, resume_unwind},
};

use rand::{random, random_range};
use sbof::{Error, Result, from_bytes, to_bytes};
use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, Debug, PartialEq)]
enum TestEnum {
    Unit,
    Newtype(u8),
    Tuple(u8, u16),
    Struct { field1: i32, field2: char },
}

#[test]
fn main() -> Result<()> {
    all()
}

fn all() -> Result<()> {
    let mut loaded = false;
    let test_struct = if fs::exists("failed_case.json")? {
        println!("loading failed_case.json...");
        let str = fs::read_to_string("failed_case.json")?;
        let test_struct: HashMap<String, TestEnum> =
            serde_json::from_str(&str).expect("could not parse failed_case.json");
        println!("done\n");
        loaded = true;
        test_struct
    } else {
        println!("random");
        let len = 0..random::<u8>() as usize;
        let entries = len.clone().map(|_| {
            (
                {
                    (0..4)
                        .map(|_| random_range::<u8, _>(65..(65 + 26)) as char)
                        .collect::<String>()
                },
                match random::<u8>() % 4 {
                    0 => TestEnum::Unit,
                    1 => TestEnum::Newtype(random()),
                    2 => TestEnum::Tuple(random(), random()),
                    _ => TestEnum::Struct {
                        field1: random(),
                        field2: random(),
                    },
                },
            )
        });

        let mut map = HashMap::new();
        for (k, v) in entries {
            map.insert(k, v);
        }
        map
    };

    let bytes = to_bytes(&test_struct)?;
    println!("serialization complete");
    let unwind = catch_unwind(|| from_bytes::<HashMap<String, TestEnum>>(&bytes));
    let deser = match unwind {
        Err(e) => {
            failed_case(&test_struct, &bytes, loaded)?;
            resume_unwind(e)
        }
        Ok(Err(e)) => {
            failed_case(&test_struct, &bytes, loaded)?;
            io::stdout().flush()?;
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

fn failed_case(test_struct: &HashMap<String, TestEnum>, bytes: &[u8], loaded: bool) -> Result<()> {
    if !loaded {
        fs::write(
            "failed_case.json",
            serde_json::to_string_pretty(test_struct).expect("failed to serialize to JSON"),
        )?;
        println!("wrote input to failed_case.json");
    }
    fs::write("failed_case.bin", bytes)?;
    println!("wrote serialized input to failed_case.bin");
    Ok(())
}

fn failed_case_eq(
    test_struct: &HashMap<String, TestEnum>,
    deser: &HashMap<String, TestEnum>,
    bytes: &[u8],
) -> Result<()> {
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
