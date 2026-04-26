// MIT License - Copyright (c) 2026 Destroyer
// CLI integration tests using assert_cmd + predicates

use assert_cmd::Command;
use predicates::prelude::*;
use std::fs;
use tempfile::TempDir;

/// Devuelve un Command listo para invocar el binario `disc` con --no-color.
fn disc() -> Command {
    let mut cmd = Command::cargo_bin("disc").unwrap();
    cmd.arg("--no-color");
    cmd
}

// ─────────────────────────────────────────────────────────────────────────────
// Help / version
// ─────────────────────────────────────────────────────────────────────────────

#[test]
fn help_shows_binary_name() {
    disc()
        .arg("--help")
        .assert()
        .success()
        .stdout(predicate::str::contains("disc"));
}

#[test]
fn version_succeeds() {
    disc().arg("--version").assert().success();
}

// ─────────────────────────────────────────────────────────────────────────────
// Completions
// ─────────────────────────────────────────────────────────────────────────────

#[test]
fn completions_bash() {
    disc()
        .args(["completions", "bash"])
        .assert()
        .success()
        .stdout(predicate::str::contains("_disc"));
}

#[test]
fn completions_zsh() {
    disc()
        .args(["completions", "zsh"])
        .assert()
        .success()
        .stdout(predicate::str::contains("#compdef disc"));
}

#[test]
fn completions_fish() {
    disc()
        .args(["completions", "fish"])
        .assert()
        .success()
        .stdout(predicate::str::contains("disc"));
}

// ─────────────────────────────────────────────────────────────────────────────
// Create
// ─────────────────────────────────────────────────────────────────────────────

#[test]
fn create_new_dsk() {
    let dir = TempDir::new().unwrap();
    let dsk = dir.path().join("test.dsk");

    disc()
        .args(["create", dsk.to_str().unwrap()])
        .assert()
        .success();

    assert!(dsk.exists(), "DSK file should exist after create");
}

#[test]
fn create_existing_without_force_fails() {
    let dir = TempDir::new().unwrap();
    let dsk = dir.path().join("test.dsk");

    disc()
        .args(["create", dsk.to_str().unwrap()])
        .assert()
        .success();

    disc()
        .args(["create", dsk.to_str().unwrap()])
        .assert()
        .failure();
}

#[test]
fn create_with_force_overwrites() {
    let dir = TempDir::new().unwrap();
    let dsk = dir.path().join("test.dsk");

    disc()
        .args(["create", dsk.to_str().unwrap()])
        .assert()
        .success();

    disc()
        .args(["create", "--force", dsk.to_str().unwrap()])
        .assert()
        .success();
}

#[test]
fn create_with_custom_tracks() {
    let dir = TempDir::new().unwrap();
    let dsk = dir.path().join("80t.dsk");

    disc()
        .args(["create", "--tracks", "80", dsk.to_str().unwrap()])
        .assert()
        .success();

    assert!(dsk.exists());
}

// ─────────────────────────────────────────────────────────────────────────────
// List
// ─────────────────────────────────────────────────────────────────────────────

#[test]
fn list_empty_dsk_shows_zero_files() {
    let dir = TempDir::new().unwrap();
    let dsk = dir.path().join("empty.dsk");
    disc()
        .args(["create", dsk.to_str().unwrap()])
        .assert()
        .success();

    disc()
        .args(["list", dsk.to_str().unwrap()])
        .assert()
        .success()
        .stdout(predicate::str::contains("0 files"));
}

#[test]
fn list_nonexistent_dsk_fails() {
    disc()
        .args(["list", "/tmp/nonexistent_disc_test_99999.dsk"])
        .assert()
        .failure();
}

#[test]
fn list_json_format_empty_dsk() {
    let dir = TempDir::new().unwrap();
    let dsk = dir.path().join("test.dsk");
    disc()
        .args(["create", dsk.to_str().unwrap()])
        .assert()
        .success();

    disc()
        .args(["list", "--format", "json", dsk.to_str().unwrap()])
        .assert()
        .success()
        .stdout(predicate::str::contains("total_size"))
        .stdout(predicate::str::contains("files"));
}

#[test]
fn list_csv_format_has_header_row() {
    let dir = TempDir::new().unwrap();
    let dsk = dir.path().join("test.dsk");
    disc()
        .args(["create", dsk.to_str().unwrap()])
        .assert()
        .success();

    disc()
        .args(["list", "--format", "csv", dsk.to_str().unwrap()])
        .assert()
        .success()
        .stdout(predicate::str::contains("name,size,type"));
}

#[test]
fn list_simple_format_empty_dsk_has_no_files() {
    let dir = TempDir::new().unwrap();
    let dsk = dir.path().join("test.dsk");
    disc()
        .args(["create", dsk.to_str().unwrap()])
        .assert()
        .success();

    // Simple format: solo imprime nombres, uno por línea. DSK vacío → stdout vacío.
    disc()
        .args(["list", "--format", "simple", dsk.to_str().unwrap()])
        .assert()
        .success()
        .stdout(predicate::str::is_empty());
}

// ─────────────────────────────────────────────────────────────────────────────
// Import
// ─────────────────────────────────────────────────────────────────────────────

#[test]
fn import_ascii_file_appears_in_list() {
    let dir = TempDir::new().unwrap();
    let dsk = dir.path().join("test.dsk");
    let src = dir.path().join("HELLO.BAS");
    fs::write(&src, b"10 PRINT \"HELLO CPC\"\n20 END\n").unwrap();

    disc()
        .args(["create", dsk.to_str().unwrap()])
        .assert()
        .success();
    disc()
        .args([
            "import",
            dsk.to_str().unwrap(),
            src.to_str().unwrap(),
            "--file-type",
            "ascii",
        ])
        .assert()
        .success();

    disc()
        .args(["list", dsk.to_str().unwrap()])
        .assert()
        .success()
        .stdout(predicate::str::contains("HELLO.BAS"));
}

#[test]
fn import_binary_file_appears_in_list() {
    let dir = TempDir::new().unwrap();
    let dsk = dir.path().join("test.dsk");
    let src = dir.path().join("LOADER.BIN");
    fs::write(&src, &[0u8; 256]).unwrap();

    disc()
        .args(["create", dsk.to_str().unwrap()])
        .assert()
        .success();
    disc()
        .args([
            "import",
            dsk.to_str().unwrap(),
            src.to_str().unwrap(),
            "--file-type",
            "binary",
        ])
        .assert()
        .success();

    disc()
        .args(["list", dsk.to_str().unwrap()])
        .assert()
        .success()
        .stdout(predicate::str::contains("LOADER.BIN"));
}

#[test]
fn import_duplicate_without_force_prints_error_to_stderr() {
    let dir = TempDir::new().unwrap();
    let dsk = dir.path().join("test.dsk");
    let src = dir.path().join("FILE.TXT");
    fs::write(&src, b"hello").unwrap();

    disc()
        .args(["create", dsk.to_str().unwrap()])
        .assert()
        .success();
    disc()
        .args(["import", dsk.to_str().unwrap(), src.to_str().unwrap()])
        .assert()
        .success();

    // Segunda importación sin --force: stderr contiene el mensaje de error
    disc()
        .args(["import", dsk.to_str().unwrap(), src.to_str().unwrap()])
        .assert()
        .success()
        .stderr(predicate::str::contains("already exists"));
}

#[test]
fn import_with_force_overwrites_existing() {
    let dir = TempDir::new().unwrap();
    let dsk = dir.path().join("test.dsk");
    let src = dir.path().join("FILE.TXT");
    fs::write(&src, b"version 1").unwrap();

    disc()
        .args(["create", dsk.to_str().unwrap()])
        .assert()
        .success();
    disc()
        .args(["import", dsk.to_str().unwrap(), src.to_str().unwrap()])
        .assert()
        .success();

    fs::write(&src, b"version 2").unwrap();
    disc()
        .args([
            "import",
            "--force",
            dsk.to_str().unwrap(),
            src.to_str().unwrap(),
        ])
        .assert()
        .success()
        .stderr(predicate::str::is_empty());
}

#[test]
fn import_nonexistent_source_prints_error() {
    let dir = TempDir::new().unwrap();
    let dsk = dir.path().join("test.dsk");
    disc()
        .args(["create", dsk.to_str().unwrap()])
        .assert()
        .success();

    // El fichero fuente no existe: error en stderr, exit 0 (no es fatal)
    disc()
        .args([
            "import",
            dsk.to_str().unwrap(),
            "/tmp/no_such_file_disc_test.bas",
        ])
        .assert()
        .success()
        .stderr(predicate::str::contains("not found").or(predicate::str::contains("No such file")));
}

// ─────────────────────────────────────────────────────────────────────────────
// Export
// ─────────────────────────────────────────────────────────────────────────────

#[test]
fn export_creates_file_in_output_dir() {
    let dir = TempDir::new().unwrap();
    let dsk = dir.path().join("test.dsk");
    let src = dir.path().join("DATA.TXT");
    let out = dir.path().join("out");
    fs::write(&src, b"Hello Amstrad CPC\n").unwrap();

    disc()
        .args(["create", dsk.to_str().unwrap()])
        .assert()
        .success();
    disc()
        .args([
            "import",
            dsk.to_str().unwrap(),
            src.to_str().unwrap(),
            "--file-type",
            "ascii",
        ])
        .assert()
        .success();

    disc()
        .args([
            "export",
            dsk.to_str().unwrap(),
            "DATA.TXT",
            "--output",
            out.to_str().unwrap(),
        ])
        .assert()
        .success();

    assert!(out.join("DATA.TXT").exists(), "Exported file should exist");
}

#[test]
fn export_nonexistent_file_prints_error() {
    let dir = TempDir::new().unwrap();
    let dsk = dir.path().join("test.dsk");
    let out = dir.path().join("out");
    disc()
        .args(["create", dsk.to_str().unwrap()])
        .assert()
        .success();

    disc()
        .args([
            "export",
            dsk.to_str().unwrap(),
            "GHOST.BAS",
            "--output",
            out.to_str().unwrap(),
        ])
        .assert()
        .success()
        .stderr(
            predicate::str::contains("not found").or(predicate::str::contains("File not found")),
        );
}

#[test]
fn export_auto_creates_output_directory() {
    let dir = TempDir::new().unwrap();
    let dsk = dir.path().join("test.dsk");
    let src = dir.path().join("TEST.BIN");
    let out = dir.path().join("subdir").join("deep");
    fs::write(&src, &[0xAB_u8; 128]).unwrap();

    disc()
        .args(["create", dsk.to_str().unwrap()])
        .assert()
        .success();
    disc()
        .args([
            "import",
            dsk.to_str().unwrap(),
            src.to_str().unwrap(),
            "--file-type",
            "binary",
        ])
        .assert()
        .success();

    disc()
        .args([
            "export",
            dsk.to_str().unwrap(),
            "TEST.BIN",
            "--output",
            out.to_str().unwrap(),
        ])
        .assert()
        .success();

    assert!(out.join("TEST.BIN").exists());
}

// ─────────────────────────────────────────────────────────────────────────────
// Remove
// ─────────────────────────────────────────────────────────────────────────────

#[test]
fn remove_file_disappears_from_list() {
    let dir = TempDir::new().unwrap();
    let dsk = dir.path().join("test.dsk");
    let src = dir.path().join("PROG.BAS");
    fs::write(&src, b"10 PRINT\n").unwrap();

    disc()
        .args(["create", dsk.to_str().unwrap()])
        .assert()
        .success();
    disc()
        .args(["import", dsk.to_str().unwrap(), src.to_str().unwrap()])
        .assert()
        .success();

    // Verificar que está en el disco
    disc()
        .args(["list", dsk.to_str().unwrap()])
        .assert()
        .success()
        .stdout(predicate::str::contains("PROG.BAS"));

    // Eliminar
    disc()
        .args(["remove", "--force", dsk.to_str().unwrap(), "PROG.BAS"])
        .assert()
        .success();

    // Verificar que ya no está
    disc()
        .args(["list", "--format", "simple", dsk.to_str().unwrap()])
        .assert()
        .success()
        .stdout(predicate::str::is_empty());
}

#[test]
fn remove_nonexistent_file_still_exits_ok() {
    let dir = TempDir::new().unwrap();
    let dsk = dir.path().join("test.dsk");
    disc()
        .args(["create", dsk.to_str().unwrap()])
        .assert()
        .success();

    disc()
        .args(["remove", "--force", dsk.to_str().unwrap(), "GHOST.BAS"])
        .assert()
        .success();
}

// ─────────────────────────────────────────────────────────────────────────────
// Check
// ─────────────────────────────────────────────────────────────────────────────

#[test]
fn check_valid_empty_dsk_succeeds() {
    let dir = TempDir::new().unwrap();
    let dsk = dir.path().join("empty.dsk");

    disc()
        .args(["create", dsk.to_str().unwrap()])
        .assert()
        .success();

    disc()
        .args(["check", dsk.to_str().unwrap()])
        .assert()
        .success()
        .stdout(predicate::str::contains("OK"));
}

#[test]
fn check_valid_dsk_with_file_succeeds() {
    let dir = TempDir::new().unwrap();
    let dsk = dir.path().join("test.dsk");
    let src = dir.path().join("PROG.BAS");
    fs::write(&src, b"10 PRINT \"CPC\"\n20 END\n").unwrap();

    disc()
        .args(["create", dsk.to_str().unwrap()])
        .assert()
        .success();
    disc()
        .args([
            "import",
            dsk.to_str().unwrap(),
            src.to_str().unwrap(),
            "--file-type",
            "ascii",
        ])
        .assert()
        .success();

    disc()
        .args(["check", dsk.to_str().unwrap()])
        .assert()
        .success()
        .stdout(predicate::str::contains("OK"))
        .stdout(predicate::str::contains("PROG.BAS"));
}

#[test]
fn check_nonexistent_dsk_fails() {
    disc()
        .args(["check", "/tmp/nonexistent_disc_check_99999.dsk"])
        .assert()
        .failure();
}

#[test]
fn check_non_dsk_file_fails() {
    let dir = TempDir::new().unwrap();
    let garbage = dir.path().join("garbage.dsk");
    // Write garbage bytes — not a valid DSK
    fs::write(&garbage, b"this is not a dsk file at all!!!!").unwrap();

    disc()
        .args(["check", garbage.to_str().unwrap()])
        .assert()
        .failure();
}

// ─────────────────────────────────────────────────────────────────────────────
// Info
// ─────────────────────────────────────────────────────────────────────────────

#[test]
fn info_empty_dsk_shows_geometry() {
    let dir = TempDir::new().unwrap();
    let dsk = dir.path().join("empty.dsk");

    disc()
        .args(["create", dsk.to_str().unwrap()])
        .assert()
        .success();

    disc()
        .args(["info", dsk.to_str().unwrap()])
        .assert()
        .success()
        // Geometry
        .stdout(predicate::str::contains("40"))
        .stdout(predicate::str::contains("9"))
        // Directory
        .stdout(predicate::str::contains("0 / 64"));
}

#[test]
fn info_dsk_with_file_shows_filename() {
    let dir = TempDir::new().unwrap();
    let dsk = dir.path().join("test.dsk");
    let src = dir.path().join("HELLO.BAS");
    fs::write(&src, b"10 PRINT \"HELLO\"\n").unwrap();

    disc()
        .args(["create", dsk.to_str().unwrap()])
        .assert()
        .success();
    disc()
        .args([
            "import",
            dsk.to_str().unwrap(),
            src.to_str().unwrap(),
            "--file-type",
            "ascii",
        ])
        .assert()
        .success();

    disc()
        .args(["info", dsk.to_str().unwrap()])
        .assert()
        .success()
        .stdout(predicate::str::contains("HELLO.BAS"))
        .stdout(predicate::str::contains("1 / 64"));
}

#[test]
fn info_nonexistent_dsk_fails() {
    disc()
        .args(["info", "/tmp/nonexistent_disc_info_99999.dsk"])
        .assert()
        .failure();
}

// ─────────────────────────────────────────────────────────────────────────────
// Diff
// ─────────────────────────────────────────────────────────────────────────────

#[test]
fn diff_empty_dsk_with_itself_shows_zero_differences() {
    let dir = TempDir::new().unwrap();
    let dsk = dir.path().join("empty.dsk");

    disc()
        .args(["create", dsk.to_str().unwrap()])
        .assert()
        .success();

    disc()
        .args(["diff", dsk.to_str().unwrap(), dsk.to_str().unwrap()])
        .assert()
        .success()
        .stdout(predicate::str::contains("0 added"))
        .stdout(predicate::str::contains("0 removed"))
        .stdout(predicate::str::contains("0 modified"));
}

#[test]
fn diff_dsk_with_itself_shows_identical_files() {
    let dir = TempDir::new().unwrap();
    let dsk = dir.path().join("test.dsk");
    let src = dir.path().join("DATA.TXT");
    fs::write(&src, b"Amstrad CPC 6128\n").unwrap();

    disc()
        .args(["create", dsk.to_str().unwrap()])
        .assert()
        .success();
    disc()
        .args(["import", dsk.to_str().unwrap(), src.to_str().unwrap()])
        .assert()
        .success();

    disc()
        .args(["diff", dsk.to_str().unwrap(), dsk.to_str().unwrap()])
        .assert()
        .success()
        .stdout(predicate::str::contains("Identical"))
        .stdout(predicate::str::contains("DATA.TXT"))
        .stdout(predicate::str::contains("0 added"))
        .stdout(predicate::str::contains("0 removed"));
}

#[test]
fn diff_two_different_dsks_shows_differences() {
    let dir = TempDir::new().unwrap();
    let dsk1 = dir.path().join("disk1.dsk");
    let dsk2 = dir.path().join("disk2.dsk");
    let file_a = dir.path().join("GAME.BAS");
    let file_b = dir.path().join("MENU.BAS");
    fs::write(&file_a, b"10 PRINT \"GAME\"\n").unwrap();
    fs::write(&file_b, b"10 PRINT \"MENU\"\n").unwrap();

    // dsk1 has GAME.BAS, dsk2 has MENU.BAS
    disc()
        .args(["create", dsk1.to_str().unwrap()])
        .assert()
        .success();
    disc()
        .args([
            "import",
            dsk1.to_str().unwrap(),
            file_a.to_str().unwrap(),
            "--file-type",
            "ascii",
        ])
        .assert()
        .success();

    disc()
        .args(["create", dsk2.to_str().unwrap()])
        .assert()
        .success();
    disc()
        .args([
            "import",
            dsk2.to_str().unwrap(),
            file_b.to_str().unwrap(),
            "--file-type",
            "ascii",
        ])
        .assert()
        .success();

    disc()
        .args(["diff", dsk1.to_str().unwrap(), dsk2.to_str().unwrap()])
        .assert()
        .success()
        .stdout(predicate::str::contains("GAME.BAS"))
        .stdout(predicate::str::contains("MENU.BAS"))
        .stdout(predicate::str::contains("1 added"))
        .stdout(predicate::str::contains("1 removed"));
}

#[test]
fn diff_nonexistent_dsk_fails() {
    let dir = TempDir::new().unwrap();
    let dsk = dir.path().join("real.dsk");
    disc()
        .args(["create", dsk.to_str().unwrap()])
        .assert()
        .success();

    disc()
        .args([
            "diff",
            dsk.to_str().unwrap(),
            "/tmp/nonexistent_disc_diff_99999.dsk",
        ])
        .assert()
        .failure();
}

#[test]
fn full_pipeline_create_import_list_export_remove() {
    let dir = TempDir::new().unwrap();
    let dsk = dir.path().join("pipeline.dsk");
    let prog = dir.path().join("GAME.BAS");
    let data = dir.path().join("SPRITES.BIN");
    let out = dir.path().join("out");

    fs::write(&prog, b"10 PRINT \"AMSTRAD\"\n20 END\n").unwrap();
    fs::write(&data, &[0xE5_u8; 512]).unwrap();

    // 1. Crear disco
    disc()
        .args(["create", dsk.to_str().unwrap()])
        .assert()
        .success();

    // 2. Importar dos ficheros
    disc()
        .args([
            "import",
            dsk.to_str().unwrap(),
            prog.to_str().unwrap(),
            "--file-type",
            "ascii",
        ])
        .assert()
        .success();
    disc()
        .args([
            "import",
            dsk.to_str().unwrap(),
            data.to_str().unwrap(),
            "--file-type",
            "binary",
        ])
        .assert()
        .success();

    // 3. Listar: ambos presentes
    disc()
        .args(["list", dsk.to_str().unwrap()])
        .assert()
        .success()
        .stdout(predicate::str::contains("GAME.BAS"))
        .stdout(predicate::str::contains("SPRITES.BIN"));

    // 4. Exportar GAME.BAS
    disc()
        .args([
            "export",
            dsk.to_str().unwrap(),
            "GAME.BAS",
            "--output",
            out.to_str().unwrap(),
        ])
        .assert()
        .success();
    assert!(out.join("GAME.BAS").exists());

    // 5. Eliminar SPRITES.BIN
    disc()
        .args(["remove", "--force", dsk.to_str().unwrap(), "SPRITES.BIN"])
        .assert()
        .success();

    // 6. Listar: solo queda GAME.BAS
    disc()
        .args(["list", "--format", "simple", dsk.to_str().unwrap()])
        .assert()
        .success()
        .stdout(predicate::str::contains("GAME.BAS"))
        .stdout(predicate::str::contains("SPRITES.BIN").not());
}

// ─────────────────────────────────────────────────────────────────────────────
// Copy
// ─────────────────────────────────────────────────────────────────────────────

/// Helper: crea un DSK vacío en `path`.
fn create_dsk(path: &std::path::Path) {
    disc()
        .args(["create", path.to_str().unwrap()])
        .assert()
        .success();
}

/// Helper: importa un archivo ASCII en un DSK.
fn import_ascii(dsk: &std::path::Path, file: &std::path::Path) {
    disc()
        .args([
            "import",
            dsk.to_str().unwrap(),
            file.to_str().unwrap(),
            "--file-type",
            "ascii",
        ])
        .assert()
        .success();
}

/// Helper: importa un archivo binario en un DSK.
fn import_binary(dsk: &std::path::Path, file: &std::path::Path) {
    disc()
        .args([
            "import",
            dsk.to_str().unwrap(),
            file.to_str().unwrap(),
            "--file-type",
            "binary",
            "--load",
            "0x4000",
        ])
        .assert()
        .success();
}

#[test]
fn copy_all_files_to_empty_dsk() {
    let dir = TempDir::new().unwrap();
    let src = dir.path().join("src.dsk");
    let dst = dir.path().join("dst.dsk");
    let prog = dir.path().join("HELLO.BAS");
    let data = dir.path().join("SPRITES.BIN");

    fs::write(&prog, b"10 PRINT \"HELLO\"\n20 END\n").unwrap();
    fs::write(&data, &[0xAB_u8; 256]).unwrap();

    create_dsk(&src);
    import_ascii(&src, &prog);
    import_binary(&src, &data);
    create_dsk(&dst);

    // Copy all files (no pattern → all)
    disc()
        .args(["copy", src.to_str().unwrap(), dst.to_str().unwrap()])
        .assert()
        .success()
        .stdout(predicate::str::contains("HELLO.BAS"))
        .stdout(predicate::str::contains("SPRITES.BIN"));

    // Verify both files appear in destination
    disc()
        .args(["list", "--format", "simple", dst.to_str().unwrap()])
        .assert()
        .success()
        .stdout(predicate::str::contains("HELLO.BAS"))
        .stdout(predicate::str::contains("SPRITES.BIN"));
}

#[test]
fn copy_specific_file() {
    let dir = TempDir::new().unwrap();
    let src = dir.path().join("src.dsk");
    let dst = dir.path().join("dst.dsk");
    let prog = dir.path().join("LOADER.BAS");
    let data = dir.path().join("DATA.BIN");

    fs::write(&prog, b"10 PRINT \"LOADER\"\n").unwrap();
    fs::write(&data, &[0x00_u8; 128]).unwrap();

    create_dsk(&src);
    import_ascii(&src, &prog);
    import_binary(&src, &data);
    create_dsk(&dst);

    // Copy only the BASIC file
    disc()
        .args([
            "copy",
            src.to_str().unwrap(),
            dst.to_str().unwrap(),
            "LOADER.BAS",
        ])
        .assert()
        .success()
        .stdout(predicate::str::contains("LOADER.BAS"));

    disc()
        .args(["list", "--format", "simple", dst.to_str().unwrap()])
        .assert()
        .success()
        .stdout(predicate::str::contains("LOADER.BAS"))
        .stdout(predicate::str::contains("DATA.BIN").not());
}

#[test]
fn copy_with_wildcard() {
    let dir = TempDir::new().unwrap();
    let src = dir.path().join("src.dsk");
    let dst = dir.path().join("dst.dsk");
    let bas1 = dir.path().join("LEVEL1.BAS");
    let bas2 = dir.path().join("LEVEL2.BAS");
    let bin = dir.path().join("GFX.BIN");

    fs::write(&bas1, b"10 REM LEVEL1\n").unwrap();
    fs::write(&bas2, b"10 REM LEVEL2\n").unwrap();
    fs::write(&bin, &[0xFF_u8; 64]).unwrap();

    create_dsk(&src);
    import_ascii(&src, &bas1);
    import_ascii(&src, &bas2);
    import_binary(&src, &bin);
    create_dsk(&dst);

    // Copy only *.BAS files
    disc()
        .args([
            "copy",
            src.to_str().unwrap(),
            dst.to_str().unwrap(),
            "*.BAS",
        ])
        .assert()
        .success();

    disc()
        .args(["list", "--format", "simple", dst.to_str().unwrap()])
        .assert()
        .success()
        .stdout(predicate::str::contains("LEVEL1.BAS"))
        .stdout(predicate::str::contains("LEVEL2.BAS"))
        .stdout(predicate::str::contains("GFX.BIN").not());
}

#[test]
fn copy_without_force_fails_when_file_exists() {
    let dir = TempDir::new().unwrap();
    let src = dir.path().join("src.dsk");
    let dst = dir.path().join("dst.dsk");
    let prog = dir.path().join("MENU.BAS");

    fs::write(&prog, b"10 PRINT \"MENU\"\n").unwrap();

    create_dsk(&src);
    import_ascii(&src, &prog);
    create_dsk(&dst);
    import_ascii(&dst, &prog);

    // Copy without --force should print an error to stderr but exit 0
    disc()
        .args([
            "copy",
            src.to_str().unwrap(),
            dst.to_str().unwrap(),
            "MENU.BAS",
        ])
        .assert()
        .success()
        .stderr(predicate::str::contains("already exists"));
}

#[test]
fn copy_with_force_overwrites_existing() {
    let dir = TempDir::new().unwrap();
    let src = dir.path().join("src.dsk");
    let dst = dir.path().join("dst.dsk");
    let prog = dir.path().join("MENU.BAS");
    let prog2 = dir.path().join("MENU2.BAS");

    fs::write(&prog, b"10 PRINT \"NEW\"\n").unwrap();
    fs::write(&prog2, b"10 PRINT \"OLD\"\n").unwrap();

    create_dsk(&src);
    import_ascii(&src, &prog);

    // Import prog2 but rename it as MENU.BAS in destination (use same filename)
    create_dsk(&dst);
    import_ascii(&dst, &prog); // same file, same name

    // --force should succeed
    disc()
        .args([
            "copy",
            "--force",
            src.to_str().unwrap(),
            dst.to_str().unwrap(),
            "MENU.BAS",
        ])
        .assert()
        .success()
        .stdout(predicate::str::contains("MENU.BAS"));
}

#[test]
fn copy_nonexistent_src_fails() {
    let dir = TempDir::new().unwrap();
    let dst = dir.path().join("dst.dsk");
    create_dsk(&dst);

    disc()
        .args([
            "copy",
            "/tmp/nonexistent_copy_src_99999.dsk",
            dst.to_str().unwrap(),
        ])
        .assert()
        .failure();
}

#[test]
fn copy_nonexistent_dst_fails() {
    let dir = TempDir::new().unwrap();
    let src = dir.path().join("src.dsk");
    create_dsk(&src);

    disc()
        .args([
            "copy",
            src.to_str().unwrap(),
            "/tmp/nonexistent_copy_dst_99999.dsk",
        ])
        .assert()
        .failure();
}

#[test]
fn copy_alias_cp_works() {
    let dir = TempDir::new().unwrap();
    let src = dir.path().join("src.dsk");
    let dst = dir.path().join("dst.dsk");
    let prog = dir.path().join("TEST.BAS");

    fs::write(&prog, b"10 PRINT \"TEST\"\n").unwrap();
    create_dsk(&src);
    import_ascii(&src, &prog);
    create_dsk(&dst);

    disc()
        .args(["cp", src.to_str().unwrap(), dst.to_str().unwrap()])
        .assert()
        .success()
        .stdout(predicate::str::contains("TEST.BAS"));
}

#[test]
fn copy_no_matching_files_reports_warning() {
    let dir = TempDir::new().unwrap();
    let src = dir.path().join("src.dsk");
    let dst = dir.path().join("dst.dsk");
    let prog = dir.path().join("HELLO.BAS");

    fs::write(&prog, b"10 PRINT \"HI\"\n").unwrap();
    create_dsk(&src);
    import_ascii(&src, &prog);
    create_dsk(&dst);

    // Pattern that matches nothing
    disc()
        .args([
            "copy",
            src.to_str().unwrap(),
            dst.to_str().unwrap(),
            "NOEXIST.DAT",
        ])
        .assert()
        .success()
        .stderr(predicate::str::contains("No files match"));
}
