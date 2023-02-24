use std::fs;

use std::io::Write;

fn main() {





    // assign path "C:\\tmp\\UserFileSizes.csv" to a variable
    let result_path = "C:\\temp\\UserFileSizes.csv";

    if std::path::Path::new(result_path).exists() {
        std::fs::remove_file(result_path).unwrap();
    }


    let mut file = std::fs::OpenOptions::new()
        .write(true)
        .append(true)
        .create(true)
        .open(result_path)
        .unwrap();

        writeln!(
            file,
            "Full Path, File Name, Size, Raw Size"
        )
        .unwrap();

    let path = "C:\\Users";
    deeper(path, &mut file );

}

// function to recursively scan a path
fn deeper(path : &str, file : &mut std::fs::File) {
    let paths = fs::read_dir(path);
    if paths.is_err() {
        return;
    }
    let paths = paths.unwrap();

    
    for path in paths {
        if path.is_err() {
            continue;
        }
        let path = path.unwrap().path();
        let metadata = fs::metadata(&path);
        if metadata.is_err() {
            continue;
        }
        let metadata = metadata.unwrap();
        let size = metadata.len();
        let rawsize = metadata.len();

        let size = if size > 1000000000 {
            format!("{:.2} GB", size as f64 / 1000000000.0)
        } else if size > 1000000 {
            format!("{:.2} MB", size as f64 / 1000000.0)
        } else if size > 1000 {
            format!("{:.2} KB", size as f64 / 1000.0)
        } else {
            format!("{} B", size)
        };

        let name = path.file_name().unwrap().to_str().unwrap();

        writeln!(
            file,
            "{}, {}, {}, {}",
            path.display().to_string().replace(",", ""),
            name.replace(",", ""),
            size,
            rawsize
        )
        .unwrap();


        if metadata.is_dir() {
            if name.starts_with(".") || name == "AppData" {
                continue;
            }
            if name.starts_with(".") || name.starts_with("$") {
                continue;
            }
            deeper(&path.to_str().unwrap(), file);
        }
    }
}