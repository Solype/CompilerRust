# Rapport de tests de l'encodeur x86-64

1er octobre 2026, mis à jour après la correction des sauts · version en ligne : https://claude.ai/code/artifact/5f97e6cd-52b8-4205-bc58-d1c65b70267f

## Résumé

779 tests unitaires couvrent maintenant toutes les familles d'instructions de l'encodeur : ils ont trouvé **76 encodages faux**, dus à **12 causes**. Les sauts `jg`/`jl`/`jle`, qui étaient permutés, sont corrigés depuis : **il reste 73 cas faux, dus à 11 causes**. Le plus grave restant : les registres `r8`–`r15` utilisés comme base ou index d'une adresse mémoire sont silencieusement remplacés par `rax`–`rdi`.

`cargo test` passe (706 tests OK, 0 échec) parce que chaque cas faux restant est marqué `#[ignore = "BUG: produit ..."]`. `cargo test -- --ignored` fait échouer les 73, ce qui donne la liste de travail.

Cinq causes sont dans le code d'adressage commun (`utils.rs` / `modrm.rs`) et représentent à elles seules 42 des 76 cas.

## Méthode

Les octets attendus viennent d'une référence indépendante, jamais de l'encodeur lui-même.

1. Chaque cas associe une instruction Rust à son équivalent en assembleur Intel. GNU `as` assemble ce dernier et fournit les octets attendus. Pour les NOP multi-octets, la référence est la table du manuel Intel (SDM vol. 2B).
2. Un script Python génère les fichiers de test Rust à partir de cette liste, puis on lance `cargo test`.
3. Chaque échec est désassemblé avec `objdump`. S'il décode vers la même instruction que la référence, l'encodage est valide et ajouté comme alternative acceptée (`[...] | [...]`). Sinon, c'est un bug, marqué `#[ignore]` avec le désassemblage de ce qui est produit.
4. Trois familles de faux positifs ont été reclassées à la main comme valides : le `REX.W` redondant sur `push`/`pop`, `int 3` encodé `cd 03` au lieu de `cc`, et le préfixe `ds:` explicite que `as` supprime.

Les relocations (offset, type, addend) sont vérifiées à part, contre la sortie de `readelf -r` sur le même code assemblé par `as`.

## Couverture

Sur 767 cas d'encodage, 612 produisent exactement les octets de GNU `as`, 79 un encodage différent mais valide, et 76 un encodage faux. Les 12 tests de relocations passent tous.

- **Familles :** 18 fichiers (mov, alu, sse, complex, unary, no_operand, sys, shift, bit, bitscan, cmov, setcc, lea, stack, string, prefix, ctrl, conversion), plus `relocations.rs`.
- **Tailles :** 8, 16, 32 et 64 bits, avec les registres étendus `r8`–`r15` et `xmm8`–`xmm15`.
- **Adressage :** 19 modes, dont `[rsp]`, `[rbp]`, `[r12]`, `[r13]`, `[rbp+rcx]`, `[rbx+r8*2]`, `[rcx*4+16]` sans base, `[rip+sym]` et l'adresse absolue avec segment `fs:`.
- **Relocations :** offset, type et addend des sauts, de `loop`, et des accès `[rip+sym]` suivis d'une immédiate ou précédés de `lock`.

Résultats par famille à la première exécution, triés par nombre de cas faux (les 3 cas faux de `ctrl` sont corrigés depuis) :

| Famille | Identiques à `as` | Équivalents valides | Faux |
| --- | ---: | ---: | ---: |
| mov | 59 | 17 | 22 |
| sse | 74 | 0 | 14 |
| alu | 88 | 49 | 9 |
| shift | 70 | 0 | 5 |
| lea | 12 | 0 | 5 |
| unary | 40 | 0 | 4 |
| bitscan | 10 | 0 | 4 |
| ctrl | 20 | 0 | 3 |
| prefix | 8 | 1 | 3 |
| complex | 60 | 0 | 2 |
| setcc | 18 | 0 | 2 |
| no_operand | 28 | 0 | 1 |
| stack | 12 | 11 | 1 |
| conversion | 19 | 0 | 1 |
| bit | 40 | 0 | 0 |
| string | 29 | 0 | 0 |
| cmov | 22 | 0 | 0 |
| sys | 3 | 1 | 0 |

En proportion, `lea` (5 sur 17) et `bitscan` (4 sur 14) sont les familles les plus touchées : elles dépendent entièrement du code d'adressage et des bits REX.

## Bugs trouvés

Les 76 cas faux se répartissaient en 12 causes, classées de la plus grave à la moins grave. La cause 1 est corrigée. Les causes 2, 4, 6, 8 et 9 sont dans le code d'adressage commun et touchent donc toutes les familles.

| # | Cause | Cas | Exemple (attendu → produit) | Fichier |
| --- | --- | --- | --- | --- |
| 1 | **Corrigé.** Opcodes de `jg`, `jl` et `jle` permutés | 3 | `jg` → `jl` | `encode_ctrl.rs` |
| 2 | `REX.B`/`REX.X` jamais émis pour la base ou l'index d'une adresse mémoire | 19 | `[r9]` → `[rcx]`, `[rbx+r8*2]` → `[rbx+rax*2]` | `utils.rs` (`emit_rex`) |
| 3 | Immédiate 64 bits émise là où x86 n'accepte que 32 bits : 4 octets parasites décalent la suite du code | 11 | `add qword [rbx+8], 0x1000`, `test r9, 1` | `encode_alu` |
| 4 | `r12`/`r13` comme base : SIB et disp8 oubliés (test sur `index` au lieu de `low3()`) | 9 | `[r12]`, `[r13]` → octets invalides | `modrm.rs` |
| 5 | `REX.R`/`REX.B` manquants dans certains encodeurs | 11 | `bsf r8, rbx` → `bsf rax, rbx` ; `shl r10, 7` → `shl rdx, 7` ; `imul r10, r11` → `imul rdx, r11` | `encode_bitscan.rs`, `encode_shift_rotate.rs`, `encode_complexbin.rs` |
| 6 | `sil`, `dil`, `spl`, `bpl` sans le REX `0x40` obligatoire | 10 | `mov sil, al` → `mov dh, al` | `utils.rs` (`emit_rex`) |
| 7 | `.absolute()` encode un adressage relatif à `rip` | 3 | `mov eax, [0x1000]` → `[rip+0x1000]` ; `fs:[0x28]` faux aussi | `modrm.rs` |
| 8 | `[rbp+index]` sans déplacement : `mod=00` avec base `101` signifie « pas de base » | 2 | `[rbp+rcx]` → invalide | `modrm.rs` |
| 9 | `[index*scale+disp]` sans base encodé avec un disp8 | 2 | `[rcx*4+16]` → `[rbp+rcx*4+16]` | `modrm.rs` |
| 10 | `movzx`/`movsx` avec une source 16 bits : la destination devient 16 bits | 4 | `movzx eax, bx` → `movzx ax, bx` | `encode_alu` |
| 11 | `cwd` sans le préfixe `66` | 1 | `cwd` → `cdq` | `encode_unary.rs` |
| 12 | `lock` refusé sur toute la famille `Unary` (panic) | 1 | `lock inc qword [rbx]` | `encode_prefix.rs` |

Par ailleurs, il n'existe aucun moyen d'exprimer `movsxd` (extension 32 vers 64 bits).

## Optimisations possibles

Les 79 encodages « équivalents » sont corrects ; certains coûtent un octet de plus que nécessaire.

- **ALU entre registres :** l'encodeur utilise toujours la forme `reg, r/m` (`03 d8` au lieu de `01 c3` pour `add ebx, eax`). Aucun impact sur la taille.
- **Immédiate avec `eax` :** `add eax, 0x1000` utilise `81 c0` au lieu du raccourci `05`, et `test eax, imm` n'utilise pas `a9`. 1 octet perdu par instruction.
- **`push`/`pop` :** un `REX.W` inutile est émis quand `size` vaut `None`, alors que 64 bits est déjà la taille implicite. 1 octet perdu par instruction.
- **`int 3` :** encodé `cd 03` (2 octets) au lieu de la forme courte `cc`.

## Organisation des tests

Tous les tests sont dans `src/elf/instructions/encode_tests/`, un fichier par famille ; aucun fichier de code n'en contient. Les tests de `cvtsi2sd`, écrits d'abord dans `encode_conversion.rs`, ont été déplacés dans `encode_tests/conversion.rs`.

- **`mod.rs`** définit `check` et la macro `cases!` : `nom: instruction => [octets] | [autre encodage valide];` produit un `#[test]`, avec l'instruction assembleur en commentaire `///`.
- **Fichiers générés :** 17 familles, de `alu.rs` à `unary.rs`.
- **Fichiers écrits à la main :** `conversion.rs` (20 tests) et `relocations.rs` (12 tests).
- **Dossier `encode_tests` et non `tests` :** le motif `test*` du `.gitignore` aurait exclu le dossier du dépôt.

Modifications ailleurs : `src/elf/instructions/mod.rs` déclare `#[cfg(test)] mod encode_tests;`, et `src/samples/mod.rs` passe `helpers` en `pub(crate)` pour que les tests réutilisent `bin`, `reg`, `mem`, `at`, etc.

Pour lancer :

```bash
cargo test                      # 706 OK, 73 ignorés
cargo test -- --ignored         # les 73 bugs restants, tous en échec
cargo test encode_tests::mov    # une seule famille
```

Quand un bug est corrigé, il suffit de retirer le `#[ignore]` du cas correspondant.

## Prochaines étapes

Les sauts sont corrigés ; la suite est le code d'adressage commun, qui débloque 42 cas d'un coup.

- [x] Remettre les opcodes de `jg` (`0x8F`), `jl` (`0x8C`) et `jle` (`0x8E`) dans `encode_ctrl.rs`
- [ ] Faire calculer à `emit_rex` les bits `REX.B`/`REX.X` depuis la base et l'index des opérandes mémoire, et émettre un REX nu pour `spl`/`bpl`/`sil`/`dil` (causes 2 et 6)
- [ ] Dans `modrm.rs`, tester `low3()` au lieu de `index` pour `rsp`/`r12` et `rbp`/`r13`, et corriger `[rbp+index]`, `[index*scale+disp]` et `.absolute()` (causes 4, 7, 8, 9)
- [ ] Limiter les immédiates à 32 bits signés pour l'ALU et `mov` vers la mémoire en 64 bits (cause 3)
- [ ] Utiliser `emit_rex` dans `encode_bitscan.rs`, les shifts par immédiate et `imul` à 2 et 3 opérandes (cause 5)
- [ ] Corriger `movzx`/`movsx` 16 bits, `cwd` et `lock` sur les unaires (causes 10 à 12)
- [ ] Ajouter le générateur de tests au dépôt (par exemple `tools/gen_encode_tests.py`) : il est aujourd'hui dans un dossier temporaire de session
