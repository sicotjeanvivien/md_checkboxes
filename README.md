# md_checkboxes

Un petit outil en ligne de commande écrit en Rust pour transformer les checkboxes Markdown en emojis.

| Avant | Après |
|-------|-------|
| `- [x] Tâche accomplie` | `- ✅ Tâche accomplie` |
| `- [X] Tâche accomplie` | `- ✅ Tâche accomplie` |
| `- [ ] Tâche non faite` | `- ❌ Tâche non faite` |

---

## Prérequis

- [Rust](https://www.rust-lang.org/tools/install) (édition 2021 ou supérieure)

```bash
curl --proto '=https' --tlsv1.2 -sSf https://sh.rustup.rs | sh
```

---

## Installation

```bash
# Cloner ou copier le projet
cd md_checkboxes

# Compiler en mode release
cargo build --release
```

Le binaire compilé se trouve dans `./target/release/md_checkboxes`.

Optionnellement, pour l'utiliser depuis n'importe où :

```bash
cp ./target/release/md_checkboxes /usr/local/bin/
```

---

## Utilisation

```bash
# Transformer un fichier (modifie le fichier en place)
md_checkboxes <fichier.md>

# Aperçu sans modifier le fichier
md_checkboxes <fichier.md> --dry-run
```

### Exemple

Fichier `janvier.md` avant :

```markdown
# Janvier 2025

## Objectifs
- [x] Mettre en place le projet Rust
- [x] Écrire les tests unitaires
- [ ] Rédiger la documentation
- [ ] Déployer en production
```

Après `md_checkboxes janvier.md` :

```markdown
# Janvier 2025

## Objectifs
- ✅ Mettre en place le projet Rust
- ✅ Écrire les tests unitaires
- ❌ Rédiger la documentation
- ❌ Déployer en production
```

Sortie dans le terminal :

```
✅ Fichier mis à jour : janvier.md
   2 [x] → ✅
   2 [ ] → ❌
```

---

## Options

| Option | Description |
|--------|-------------|
| `--dry-run` | Affiche le résultat transformé sans modifier le fichier |

---

## Tests

```bash
cargo test
```

Les tests couvrent :
- Conversion `[x]` minuscule
- Conversion `[X]` majuscule
- Conversion `[ ]`
- Fichiers mixtes (tâches faites et non faites)
- Fichiers sans aucune checkbox (contenu préservé intact)

---

## Structure du projet

```
md_checkboxes/
├── Cargo.toml
└── src/
    └── main.rs
```

---

## Licence

MIT

---

## Notice

Ce projet a été entièrement généré par intelligence artificielle ([Claude](https://claude.ai) par Anthropic).
