# Rapport de tests de l'encodeur x86-64

2 octobre 2026, mis à jour après la correction de tous les bugs trouvés · version en ligne : https://claude.ai/code/artifact/5f97e6cd-52b8-4205-bc58-d1c65b70267f

## Résumé

1 011 tests unitaires couvrent maintenant toutes les familles d'instructions de l'encodeur, y compris avec les registres `r8`–`r15` et les adresses étendues. Ils ont trouvé **199 encodages faux**, dus à **12 causes**, et **tous sont corrigés**. Les corrections ont aussi révélé deux bugs de relocation, corrigés eux aussi.

`cargo test` passe entièrement : 1 011 tests OK, aucun échec et aucun test ignoré. Un nouveau bug se marque `#[ignore = "BUG: produit ..."]` en attendant sa correction, et `cargo test -- --ignored` en donne la liste.

Le code d'adressage commun (`utils.rs` / `modrm.rs`) portait 5 des 12 causes et 42 des 76 cas de la première exécution ; les 123 cas trouvés ensuite venaient d'encodeurs qui ne transmettaient pas leur opérande à `emit_rex` (cause 5).

## Méthode

Les octets attendus viennent d'une référence indépendante, jamais de l'encodeur lui-même.

1. Chaque cas associe une instruction Rust à son équivalent en assembleur Intel. GNU `as` assemble ce dernier et fournit les octets attendus. Pour les NOP multi-octets, la référence est la table du manuel Intel (SDM vol. 2B).
2. Un script Python génère les fichiers de test Rust à partir de cette liste, puis on lance `cargo test`.
3. Chaque échec est désassemblé avec `objdump`. S'il décode vers la même instruction que la référence, l'encodage est valide et ajouté comme alternative acceptée (`[...] | [...]`). Sinon, c'est un bug, marqué `#[ignore]` avec le désassemblage de ce qui est produit.
4. Trois familles de faux positifs ont été reclassées à la main comme valides : le `REX.W` redondant sur `push`/`pop`, `int 3` encodé `cd 03` au lieu de `cc`, et le préfixe `ds:` explicite que `as` supprime.

Les relocations (offset, type, addend) sont vérifiées à part, contre la sortie de `readelf -r` sur le même code assemblé par `as`.

## Couverture

Sur 994 cas d'encodage, 906 produisent exactement les octets de GNU `as`, 88 un encodage différent mais valide, et aucun un encodage faux. Les 17 tests de relocations passent tous.

- **Familles :** 18 fichiers (mov, alu, sse, complex, unary, no_operand, sys, shift, bit, bitscan, cmov, setcc, lea, stack, string, prefix, ctrl, conversion), plus `relocations.rs`.
- **Tailles :** 8, 16, 32 et 64 bits, avec les registres étendus `r8`–`r15` et `xmm8`–`xmm15`.
- **Adressage :** 19 modes, et chaque famille testée aussi avec `r8`–`r15` en base ou en index (`[r9]`, `[rbx+r8*2]`, `[r10+rax]`), dont `[rsp]`, `[rbp]`, `[r12]`, `[r13]`, `[rbp+rcx]`, `[rbx+r8*2]`, `[rcx*4+16]` sans base, `[rip+sym]` et l'adresse absolue avec segment `fs:`.
- **Relocations :** offset, type et addend des sauts, de `loop`, et des accès `[rip+sym]` suivis d'une immédiate ou précédés de `lock`.

Résultats par famille, triés par nombre de cas faux :

| Famille | Identiques à `as` | Équivalents valides | Faux |
| --- | ---: | ---: | ---: |
| alu | 166 | 49 | 0 |
| shift | 125 | 0 | 0 |
| mov | 94 | 19 | 0 |
| sse | 96 | 0 | 0 |
| complex | 86 | 0 | 0 |
| unary | 60 | 0 | 0 |
| bit | 56 | 0 | 0 |
| stack | 12 | 18 | 0 |
| no_operand | 29 | 0 | 0 |
| string | 29 | 0 | 0 |
| cmov | 28 | 0 | 0 |
| ctrl | 23 | 0 | 0 |
| setcc | 23 | 0 | 0 |
| lea | 23 | 0 | 0 |
| bitscan | 20 | 0 | 0 |
| conversion | 20 | 0 | 0 |
| prefix | 13 | 1 | 0 |
| sys | 3 | 1 | 0 |

Les 88 encodages équivalents se concentrent dans `alu` (49), `mov` (19) et `stack` (18) : ce sont des choix d'encodage valides, détaillés dans les optimisations possibles.

## Bugs trouvés

La première exécution a trouvé 76 cas faux ; les tests ajoutés ensuite pour `r8`–`r15` en ont trouvé 123 de plus, tous dans la cause 5. **Les 12 causes et les 199 cas sont corrigés.**

En corrigeant la cause 7, deux bugs de relocation sont apparus, absents des tests jusque-là : avec un octet SIB (`[rbx+rcx*4+sym]`, `[rsp+sym]`, `[r12+sym]`), la relocation du symbole pointait un octet trop tôt, sur le SIB ; et `.absolute()` avec un symbole avait un addend de -4 au lieu de 0. Les deux sont corrigés et couverts par 5 tests de relocation, vérifiés contre `readelf -r` sur la sortie de GNU `as`.

| # | Cause | Cas | Exemple (attendu → produit) | Fichier |
| --- | --- | --- | --- | --- |
| 1 | **Corrigé.** Opcodes de `jg`, `jl` et `jle` permutés | 3 | `jg` → `jl` | `encode_ctrl.rs` |
| 2 | **Corrigé.** `REX.B`/`REX.X` jamais émis pour la base ou l'index d'une adresse mémoire | 19 | `[r9]` → `[rcx]`, `[rbx+r8*2]` → `[rbx+rax*2]` | `utils.rs` (`emit_rex`) |
| 3 | **Corrigé.** Immédiate 64 bits émise là où x86 n'accepte que 32 bits : 4 octets parasites décalent la suite du code | 11 | `add qword [rbx+8], 0x1000`, `test r9, 1` | `utils.rs` (`emit_imm_sx32`), `encode_alu`, `encode_complexbin.rs` |
| 4 | **Corrigé.** `r12`/`r13` comme base : SIB et disp8 oubliés (test sur `index` au lieu de `low3()`) | 9 | `[r12]`, `[r13]` → octets invalides | `modrm.rs` |
| 5 | **Corrigé.** Opérande r/m ou registre `reg` non transmis à `emit_rex` : `REX.R`/`REX.X`/`REX.B` manquants | 134 | `add qword [r9], 1` → `[rcx]` ; `shl r10, 7` → `shl rdx, 7` ; `bsf r8, rbx` → `bsf rax, rbx` ; `imul r10, r11` → `imul rdx, r11` | `encode_alu/encode.rs`, `encode_shift_rotate.rs`, `encode_bitscan.rs`, `encode_stack.rs`, `encode_complexbin.rs` |
| 6 | **Corrigé.** `sil`, `dil`, `spl`, `bpl` sans le REX `0x40` obligatoire | 10 | `mov sil, al` → `mov dh, al` | `utils.rs` (`emit_rex`) |
| 7 | **Corrigé.** `.absolute()` encode un adressage relatif à `rip` | 3 | `mov eax, [0x1000]` → `[rip+0x1000]` ; `fs:[0x28]` faux aussi | `modrm.rs` |
| 8 | **Corrigé.** `[rbp+index]` sans déplacement : `mod=00` avec base `101` signifie « pas de base » | 2 | `[rbp+rcx]` → invalide | `modrm.rs` |
| 9 | **Corrigé.** `[index*scale+disp]` sans base encodé avec un disp8 | 2 | `[rcx*4+16]` → `[rbp+rcx*4+16]` | `modrm.rs` |
| 10 | **Corrigé.** `movzx`/`movsx` avec une source 16 bits : la destination devient 16 bits | 4 | `movzx eax, bx` → `movzx ax, bx` | `encode_alu` |
| 11 | **Corrigé.** `cwd` sans le préfixe `66` | 1 | `cwd` → `cdq` | `encode_unary.rs` |
| 12 | **Corrigé.** `lock` refusé sur toute la famille `Unary` (panic) | 1 | `lock inc qword [rbx]` | `encode_prefix.rs` |

Par ailleurs, il n'existe aucun moyen d'exprimer `movsxd` (extension 32 vers 64 bits).

## Optimisations possibles

Les 88 encodages « équivalents » sont corrects ; certains coûtent un octet de plus que nécessaire.

- **ALU entre registres :** l'encodeur utilise toujours la forme `reg, r/m` (`03 d8` au lieu de `01 c3` pour `add ebx, eax`). Aucun impact sur la taille.
- **Immédiate avec `eax` :** `add eax, 0x1000` utilise `81 c0` au lieu du raccourci `05`, et `test eax, imm` n'utilise pas `a9`. 1 octet perdu par instruction.
- **`push`/`pop` :** un `REX.W` inutile est émis quand `size` vaut `None`, alors que 64 bits est déjà la taille implicite. 1 octet perdu par instruction.
- **`int 3` :** encodé `cd 03` (2 octets) au lieu de la forme courte `cc`.

## Organisation des tests

Tous les tests sont dans `src/elf/instructions/encode_tests/`, un fichier par famille ; aucun fichier de code n'en contient. Les tests de `cvtsi2sd`, écrits d'abord dans `encode_conversion.rs`, ont été déplacés dans `encode_tests/conversion.rs`.

- **`mod.rs`** définit `check` et la macro `cases!` : `nom: instruction => [octets] | [autre encodage valide];` produit un `#[test]`, avec l'instruction assembleur en commentaire `///`.
- **Fichiers générés :** 17 familles, de `alu.rs` à `unary.rs`.
- **Fichiers écrits à la main :** `conversion.rs` (20 tests) et `relocations.rs` (17 tests).
- **Dossier `encode_tests` et non `tests` :** le motif `test*` du `.gitignore` aurait exclu le dossier du dépôt.

Modifications ailleurs : `src/elf/instructions/mod.rs` déclare `#[cfg(test)] mod encode_tests;`, et `src/samples/mod.rs` passe `helpers` en `pub(crate)` pour que les tests réutilisent `bin`, `reg`, `mem`, `at`, etc.

Pour lancer :

```bash
cargo test                      # 1 011 OK, aucun ignoré
cargo test -- --ignored         # bugs connus non corrigés (aucun aujourd'hui)
cargo test encode_tests::mov    # une seule famille
```

Quand un bug est corrigé, il suffit de retirer le `#[ignore]` du cas correspondant.

## Prochaines étapes

Tous les bugs trouvés sont corrigés. Ce qui reste relève de l'évolution plutôt que de la correction : exprimer une destination 64 bits pour `movzx`/`movsx` et ajouter `movsxd`, ce qui demande une deuxième taille dans l'instruction ; supprimer le `REX.W` inutile de `push`/`pop` et le REX `0x40` superflu de `movzx esi, al` ; tester le `panic!` des immédiates hors de i32 ; et garder le générateur de tests dans le dépôt.

- [x] Remettre les opcodes de `jg` (`0x8F`), `jl` (`0x8C`) et `jle` (`0x8E`) dans `encode_ctrl.rs`
- [x] Faire calculer à `emit_rex` les bits `REX.B`/`REX.X` depuis la base et l'index des opérandes mémoire (cause 2)
- [x] Rétablir dans `encode_conversion.rs` le contrôle « source = registre général ou mémoire » retiré avec l'ancien `rm` (2 tests en échec)
- [x] Émettre dans `emit_rex` un REX nu pour `spl`/`bpl`/`sil`/`dil` en 8 bits, sur les registres généraux seulement (cause 6)
- [x] Dans `modrm.rs`, tester `low3()` au lieu de `index` pour `rsp`/`r12` et `rbp`/`r13`, et corriger `[rbp+index]` (causes 4 et 8)
- [x] Dans `modrm.rs`, encoder `.absolute()` avec un SIB `0x25` (`mod=00`, `rm=100`, ni base ni index) et `[index*scale+disp]` toujours en `mod=00` + base `101` + disp32 (causes 7 et 9)
- [x] Limiter les immédiates à 32 bits signés pour l'ALU et `mov` vers la mémoire en 64 bits (cause 3)
- [x] Passer le bon opérande à `emit_rex` dans l'ALU mémoire/immédiate, les shifts, `push`/`pop`, `imul` et `encode_bitscan.rs` (cause 5)
- [x] Corriger `movzx`/`movsx` 16 bits, `cwd` et `lock` sur les unaires (causes 10 à 12)
- [ ] Ajouter le générateur de tests au dépôt (par exemple `tools/gen_encode_tests.py`) : il est aujourd'hui dans un dossier temporaire de session
