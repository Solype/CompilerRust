TODO x86/x86_64 IR

==================================================
AVANCEMENT (estimation)
==================================================

Backend (encodeur x86-64 + ELF) : ~75-80 %
Compilateur complet              : ~30 %

    Encodeur x86-64 + ELF          ~75-80 %
    Lexer                          ~5 %  (automate, pas encore branché)
    Parser + AST                   0 %
    Analyse sémantique             0 %
    Génération de code AST -> IR   0 %
    CLI                            0 %

==================================================
FAIT
==================================================

ELF :
[X] Headers ELF (ehdr, shdr, phdr), en 32 et 64 bits
[X] Strtab / shstrtab
[X] Sections avec padding d'alignement
[X] Symboles globaux (fonctions et données)
[X] Symboles non définis (externes, résolus au link)
[X] Symboles locaux (labels)
[X] Relocations (rela), y compris 8 bits et sous préfixe lock
[X] Fichier objet linkable avec ld

Encodage :
[X] ModRM / SIB / displacements
[X] Adressage [base + index * scale + disp]
[X] Adressage RIP-relative et absolu
[X] Préfixe REX, registres étendus R8 à R15
[X] Tailles 8 / 16 / 32 / 64 bits
[X] Immédiats en i64
[X] Registres XMM0 à XMM15
[X] EncodeInformation pour factoriser l'encodage

Transfert de données :
[X] Mov (reg, imm, mem, sym, imm64)
[X] Movzx / Movsx
[X] Xchg
[X] Lea
[X] CMovCC (toutes les conditions)

Arithmétique / logique :
[X] Add / Sub / Adc / Sbb
[X] And / Or / Xor
[X] Cmp / Test
[X] Inc / Dec / Neg / Not
[X] Mul / Div / Idiv
[X] Imul à 1, 2 et 3 opérandes
[X] Shl / Shr / Sar / Rol / Ror (imm et cl)
[X] Bt / Bts / Btr / Btc
[X] Bsf / Bsr
[X] SetCC (toutes les conditions)

Flottants SSE scalaires (ss si U32, sd sinon) :
[X] LoadF / StoreF (movss / movsd)
[X] AddF / SubF / MulF / DivF
[X] ComiF / UcomiF
[X] Cvtsi2sd (entier -> double)

Contrôle :
[X] Jmp / Call / Ret / IRet
[X] JmpCC (toutes les conditions)
[X] Loop / Loope / Loopne
[X] Syscall / Sysenter / Int

Pile :
[X] Push / Pop
[X] Pushf / Popf
[X] Enter / Leave

Chaînes :
[X] Movs / Cmps / Scas / Lods / Stos

Préfixes :
[X] Lock / Rep / Repe / Repne (avec validation)
[X] Segment override (Cs, Ds, Es, Ss, Fs, Gs)

Instructions sans opérande :
[X] Nop (1 à 9 octets)
[X] Cbw / Cwde / Cdqe
[X] Cwd / Cdq / Cqo
[X] Clc / Stc / Cmc
[X] Cld / Std
[X] Cli / Sti
[X] Lahf / Sahf
[X] Ud2 / Hlt / Pause / Fwait

Atomiques :
[X] Xadd
[X] Cmpxchg

Tests :
[X] 1011 tests unitaires (src/elf/instructions/encode_tests/), octets de
    référence produits par GNU as, une famille par fichier
[X] Tests de relocations (offset, type, addend) comparés à readelf
[X] Correction des 199 encodages faux trouvés (voir RAPPORT_TESTS.md)

Organisation :
[X] main.rs réduit à la construction de l'ELF
[X] Programmes de test déplacés dans src/samples/
[X] Une fonction de test par famille d'instructions (même nom dans le binaire)

==================================================
À FAIRE : PRIORITAIRE (nécessaire au compilateur)
==================================================

Backend :
[ ] Call / Jmp indirects (pointeurs de fonction, GOT, tables de switch) :
    [X] call reg / jmp reg          (FF /2, FF /4)
    [X] call [mem] / jmp [mem]      (FF /2, FF /4 + ModRM mémoire)
    [X] call [rip+sym]              (relocation PC32)
    [ ] call [rip+sym@GOTPCREL]     (nouveau RelocKind, pour la libc dynamique / PIE)
[X] Conversions int <-> float (une ligne par opcode dans ConvOp) :
    [X] Cvtsi2sd
    [X] Cvtsi2ss
    [X] Cvttsd2si / Cvttss2si / Cvtsd2si / Cvtss2si
    [X] Cvtsd2ss / Cvtss2sd
[X] Ajouter Movsxd
[X] Movzx / Movsx vers une destination 64 bits (REX.W)
[X] Séparer taille source/destination (famille Extend : src_size + size)
[ ] Gérer tailles mémoire distinctes
[ ] Ajouter ImmediateFloat(f64) (constantes flottantes en .data/.rodata)
[ ] Séparer Label et Sym
[ ] Revoir l'ordre dst/src de StoreF (le registre est dans dst)
[ ] SSE scalaire courant :
    [ ] Xorps / Xorpd (mise à zéro, changement de signe)
    [ ] Andpd / Andps (valeur absolue)
    [ ] Movq / Movd (GPR <-> XMM)
    [ ] Movaps / Movapd (copie XMM -> XMM)
    [ ] Sqrtsd / Sqrtss
    [ ] Minsd / Maxsd / Minss / Maxss
[ ] Ajouter tests automatiques :
    [X] encodage (comparer les octets à une référence GNU as)
    [X] objdump (tri des encodages équivalents)
    [ ] ndisasm
    [ ] décodage
    [ ] panic! des immédiates hors de i32 (emit_imm_sx32)

Front-end :
[ ] Lexer :
    [ ] Brancher lexical_analisys dans main.rs
    [ ] Définir les tokens du langage
    [ ] Positions (ligne/colonne) pour les erreurs
[ ] Parser :
    [ ] Grammaire du langage source
    [ ] Construction de l'AST
    [ ] Messages d'erreur de syntaxe
[ ] Analyse sémantique :
    [ ] Table des symboles / portées
    [ ] Vérification des types
[ ] Génération de code :
    [ ] AST -> Vec<Instruction>
    [ ] Allocation de registres
    [ ] Gestion de la pile (variables locales, prologue/épilogue)
    [ ] Convention d'appel x86_64 SysV
    [ ] Variables globales et constantes dans .data/.rodata
[ ] CLI :
    [ ] Fichier d'entrée
    [ ] Fichier de sortie
    [ ] Options

==================================================
À FAIRE : PLUS TARD (non nécessaire au compilateur)
==================================================

[ ] Ajouter registres SIMD YMM, ZMM (XMM fait)
[ ] Ajouter registres segment
[ ] Ajouter registres contrôle/debug

[ ] Sauts courts (jmp / jcc rel8) quand la cible est proche
[ ] Ret imm16
[ ] Int3 (forme courte CC au lieu de CD 03)
[ ] Endbr64 (si la cible active CET / IBT)

[ ] Optimisations d'encodage (valides mais un octet de trop) :
    [ ] Supprimer le REX.W inutile de push / pop
    [X] Supprimer le REX 0x40 superflu de movzx esi, al
    [ ] Forme courte de l'accumulateur (05 / A9 au lieu de 81 / F7)

[ ] Garder le générateur de tests (gen_encode_tests.py) dans le dépôt

[ ] Ajouter SSE2 minimum :
    [ ] Movups
    [ ] Movdqa
    [ ] Movdqu
    [ ] Addps
    [ ] Subps
    [ ] Mulps
    [ ] Divps

[ ] Ajouter SIMD integer :
    [ ] Pxor
    [ ] Pand
    [ ] Paddd

[ ] Ajouter shuffle SIMD :
    [ ] Shufps
    [ ] Pshufd

[ ] Ajouter AVX de base
[ ] Ajouter BMI1/BMI2

[ ] Ajouter Popcnt
[ ] Ajouter Lzcnt
[ ] Ajouter Tzcnt
[ ] Ajouter Bswap

[ ] Ajouter memory fences :
    [ ] Mfence
    [ ] Lfence
    [ ] Sfence

[ ] Ajouter :
    [ ] Cmpxchg8b
    [ ] Cmpxchg16b

[ ] Ajouter :
    [ ] Cpuid
    [ ] Rdtsc
    [ ] Rdmsr
    [ ] Wrmsr

[ ] Ajouter :
    [ ] Jecxz
    [ ] Jrcxz

[ ] Ajouter string ops :
    [ ] Ins
    [ ] Outs

[ ] Ajouter support x87 FPU :
    [ ] Fld
    [ ] Fstp
    [ ] Fadd
    [ ] Fmul
    [ ] Fcom

[ ] Ajouter :
    [ ] Prefetch
    [ ] Clflush

[ ] Réfléchir à un modèle générique :
    [ ] Opcode
    [ ] Nullary
    [ ] Unary
    [ ] Binary
    [ ] Ternary

[ ] Vérifier couverture complète x86_64 SysV ABI
[ ] Vérifier encodage REX/VEX/EVEX
