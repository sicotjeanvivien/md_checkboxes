use std::env;
use std::fs;
use std::path::Path;
use std::process;

fn convert_checkboxes(content: &str) -> (String, usize, usize) {
    let mut checked_count = 0;
    let mut unchecked_count = 0;

    let result = content
        .lines()
        .map(|line| {
            // [x] ou [X] → ✅
            if line.contains("- [x]") || line.contains("- [X]") {
                checked_count += 1;
                line.replace("- [x]", "- ✅").replace("- [X]", "- ✅")
            }
            // [ ] → ❌
            else if line.contains("- [ ]") {
                unchecked_count += 1;
                line.replace("- [ ]", "- ❌")
            } else {
                line.to_string()
            }
        })
        .collect::<Vec<_>>()
        .join("\n");

    // Préserver le saut de ligne final si présent
    let result = if content.ends_with('\n') {
        result + "\n"
    } else {
        result
    };

    (result, checked_count, unchecked_count)
}

fn main() {
    let args: Vec<String> = env::args().collect();

    if args.len() < 2 {
        eprintln!("Usage: {} <fichier.md> [--dry-run]", args[0]);
        eprintln!();
        eprintln!("  <fichier.md>   Fichier Markdown à transformer");
        eprintln!("  --dry-run      Affiche le résultat sans modifier le fichier");
        process::exit(1);
    }

    let filepath = &args[1];
    let dry_run = args.contains(&"--dry-run".to_string());

    // Vérifier que le fichier existe
    if !Path::new(filepath).exists() {
        eprintln!("❌ Erreur : fichier introuvable → {}", filepath);
        process::exit(1);
    }

    // Lire le contenu
    let content = match fs::read_to_string(filepath) {
        Ok(c) => c,
        Err(e) => {
            eprintln!("❌ Erreur de lecture : {}", e);
            process::exit(1);
        }
    };

    let (converted, checked, unchecked) = convert_checkboxes(&content);

    if dry_run {
        println!("=== Aperçu (--dry-run, fichier non modifié) ===\n");
        println!("{}", converted);
        println!("=== Résumé ===");
        println!("✅ Tâches accomplies  : {}", checked);
        println!("❌ Tâches non faites  : {}", unchecked);
    } else {
        match fs::write(filepath, &converted) {
            Ok(_) => {
                println!("✅ Fichier mis à jour : {}", filepath);
                println!("   {} [x] → ✅", checked);
                println!("   {} [ ] → ❌", unchecked);
            }
            Err(e) => {
                eprintln!("❌ Erreur d'écriture : {}", e);
                process::exit(1);
            }
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_checked_lowercase() {
        let input = "- [x] Faire les courses\n";
        let (result, checked, unchecked) = convert_checkboxes(input);
        assert_eq!(result, "- ✅ Faire les courses\n");
        assert_eq!(checked, 1);
        assert_eq!(unchecked, 0);
    }

    #[test]
    fn test_checked_uppercase() {
        let input = "- [X] Appel client\n";
        let (result, checked, _) = convert_checkboxes(input);
        assert_eq!(result, "- ✅ Appel client\n");
        assert_eq!(checked, 1);
    }

    #[test]
    fn test_unchecked() {
        let input = "- [ ] Rapport mensuel\n";
        let (result, checked, unchecked) = convert_checkboxes(input);
        assert_eq!(result, "- ❌ Rapport mensuel\n");
        assert_eq!(checked, 0);
        assert_eq!(unchecked, 1);
    }

    #[test]
    fn test_mixed() {
        let input = "- [x] Tâche A\n- [ ] Tâche B\n- [X] Tâche C\n";
        let (result, checked, unchecked) = convert_checkboxes(input);
        assert_eq!(result, "- ✅ Tâche A\n- ❌ Tâche B\n- ✅ Tâche C\n");
        assert_eq!(checked, 2);
        assert_eq!(unchecked, 1);
    }

    #[test]
    fn test_no_checkboxes() {
        let input = "# Titre\nDu texte normal\n";
        let (result, checked, unchecked) = convert_checkboxes(input);
        assert_eq!(result, input);
        assert_eq!(checked, 0);
        assert_eq!(unchecked, 0);
    }
}
