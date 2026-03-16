fn main() {
    let cmd = std::env::args().nth(1);
    match cmd.as_deref() {
        Some("codegen") => codegen(),
        _ => {
            eprintln!("Usage: cargo xtask <command>");
            eprintln!("Commands:");
            eprintln!("  codegen   Generate Rust source from grammar files");
            std::process::exit(1);
        }
    }
}

fn codegen() {
    let manifest_dir = std::env::var("CARGO_MANIFEST_DIR")
        .unwrap_or_else(|_| std::env::current_dir().unwrap().to_string_lossy().into_owned());

    // xtask lives at <root>/xtask, grammars and src are at <root>
    let root = std::path::Path::new(&manifest_dir)
        .parent()
        .expect("xtask must be inside the project root");

    let src_dir = root.join("src");
    let grammars_dir = root.join("grammars");

    let entries = std::fs::read_dir(&grammars_dir)
        .unwrap_or_else(|e| panic!("could not read {}: {}", grammars_dir.display(), e))
        .filter_map(|e| e.ok())
        .filter(|e| e.path().extension().map_or(false, |ext| ext == "txt"));

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
        if existing.as_deref() != Some(&code) {
            std::fs::write(&out_path, &code)
                .unwrap_or_else(|e| panic!("failed to write {}: {}", out_path.display(), e));
            std::process::Command::new("rustfmt")
                .arg(&out_path)
                .status()
                .unwrap_or_else(|e| panic!("failed to run rustfmt on {}: {}", out_path.display(), e));
            println!("wrote {}", out_path.display());
        } else {
            println!("up to date {}", out_path.display());
        }
    }
}
