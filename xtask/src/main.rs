fn main() {
    let mut args = std::env::args().skip(1); // skip binary name
    let cmd = args.next();

    match cmd.as_deref() {
        Some("codegen") => {
            let filter = args.next(); // optional second argument
            codegen(filter.as_ref().map(|x| x.as_str()))
        }
        _ => {
            eprintln!("Usage: cargo xtask <command> [filter]");
            eprintln!("Commands:");
            eprintln!("  codegen [filter]   Generate Rust source from grammar files");
            std::process::exit(1);
        }
    }
}

fn codegen(filter: Option<&str>) {
    let manifest_dir = std::env::var("CARGO_MANIFEST_DIR").unwrap_or_else(|_| {
        std::env::current_dir()
            .unwrap()
            .to_string_lossy()
            .into_owned()
    });

    // xtask lives at <root>/xtask, grammars and src are at <root>
    let root = std::path::Path::new(&manifest_dir)
        .parent()
        .expect("xtask must be inside the project root");

    let src_dir = root.join("src");
    let grammars_dir = root.join("grammars");

    let entries = std::fs::read_dir(&grammars_dir)
        .unwrap_or_else(|e| panic!("could not read {}: {}", grammars_dir.display(), e))
        .filter_map(|e| e.ok())
        .filter(|e| {
            let path = e.path();

            // Only .txt files
            if !path.extension().map_or(false, |ext| ext == "txt") {
                return false;
            }

            // Apply optional filter
            if let Some(ref filter) = filter {
                let name = path.file_stem().and_then(|s| s.to_str()).unwrap_or("");
                return name.to_lowercase().contains(filter);
            }

            true
        });

    for entry in entries {
        let path = entry.path();
        let path_str = path.to_str().unwrap();

        let contents = std::fs::read_to_string(&path)
            .unwrap_or_else(|e| panic!("failed to read {}: {}", path_str, e));

        let name = path.file_stem().unwrap().to_str().unwrap();
        let code = xtask::generate(path_str, &contents);

        let module_dir = src_dir.join(name);
        std::fs::create_dir_all(&module_dir)
            .unwrap_or_else(|e| panic!("failed to create {}: {}", module_dir.display(), e));

        let out_path = module_dir.join("parser.rs");
        let existing = std::fs::read_to_string(&out_path).ok();

        // Format the generated code via rustfmt before comparing so the
        // idempotency check isn't defeated by formatting differences.
        let formatted_code = {
            let mut child = std::process::Command::new("rustfmt")
                .stdin(std::process::Stdio::piped())
                .stdout(std::process::Stdio::piped())
                .spawn()
                .unwrap_or_else(|e| panic!("failed to run rustfmt: {}", e));
            {
                use std::io::Write;
                child
                    .stdin
                    .take()
                    .unwrap()
                    .write_all(code.as_bytes())
                    .unwrap();
            }
            let output = child.wait_with_output().unwrap();
            assert!(output.status.success(), "rustfmt failed");
            String::from_utf8(output.stdout).unwrap()
        };

        if existing.as_deref() != Some(&formatted_code) {
            std::fs::write(&out_path, &formatted_code)
                .unwrap_or_else(|e| panic!("failed to write {}: {}", out_path.display(), e));
            println!("wrote {}", out_path.display());
        } else {
            println!("up to date {}", out_path.display());
        }
    }
}
