use std::error::Error;
use std::fs::File;
use std::io::{self, Write};
use csv::WriterBuilder;
use rusqlite::Connection;

fn main() -> Result<(), Box<dyn Error>> {
    let mut nums = String::new();
    println!("How many table entries do you want?");

    match io::stdin().read_line(&mut nums) {
        Ok(_) => {
            let num_entries: usize = match nums.trim().parse() {
                Ok(n) => n,
                Err(_) => {
                    eprintln!("Invalid number entered");
                    return Ok(());
                }
            };

            // Vector to store entries
            let mut entries: Vec<(String, String, String, String)> = Vec::new();

            for _ in 0..num_entries {
                let mut input = String::new();
                println!("Enter your first name, last name, age, and job");

                match io::stdin().read_line(&mut input) {
                    Ok(_) => {
                        let input_components: Vec<&str> = input.trim().split(',').map(|s| s.trim()).collect();

                        if input_components.len() >= 4 {
                            let first_name = input_components[0].to_string();
                            let last_name = input_components[1].to_string();
                            let age = input_components[2].to_string();
                            let job = input_components[3].to_string();

                            // Save the entry to the vector
                            entries.push((first_name.clone(), last_name.clone(), age.clone(), job.clone()));
                        } else {
                            eprintln!("Please enter all four necessary components: first name, last name, age, job");
                        }
                    }
                    Err(error) => {
                        eprintln!("Error reading input: {}", error);
                    }
                }
            }

            // Write entries to CSV file
            let csv_filename = "output.csv";
            let mut csv_writer = WriterBuilder::new().from_path(csv_filename)?;

            for (first_name, last_name, age, job) in &entries {
                csv_writer.write_record(&[first_name, last_name, age, job])?;
            }

            csv_writer.flush()?;

            // Create SQLite database and table
            let conn = Connection::open("my_database.db")?;
            conn.execute(
                "CREATE TABLE IF NOT EXISTS entries (
                    id INTEGER PRIMARY KEY,
                    first_name TEXT,
                    last_name TEXT,
                    age TEXT,
                    job TEXT
                )",
                [],
            )?;

            // Insert entries into the database
            for (first_name, last_name, age, job) in &entries {
                conn.execute(
                    "INSERT INTO entries (first_name, last_name, age, job) VALUES (?1, ?2, ?3, ?4)",
                    &[&first_name, &last_name, &age, &job],
                )?;
            }
        }
        Err(error) => {
            eprintln!("Error reading input: {}", error);
        }
    }

    Ok(())
}
