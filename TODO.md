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

Organisation :
[X] main.rs réduit à la construction de l'ELF
[X] Programmes de test déplacés dans src/samples/
[X] Une fonction de test par famille d'instructions (même nom dans le binaire)

==================================================
À FAIRE : PRIORITAIRE (nécessaire au compilateur)
==================================================

Backend :
[ ] Conversions int <-> float :
    [ ] Cvtsi2sd / Cvtsi2ss
    [ ] Cvttsd2si / Cvttss2si
    [ ] Cvtsd2ss / Cvtss2sd
[ ] Ajouter Movsxd
[ ] Séparer taille source/destination
[ ] Gérer tailles mémoire distinctes
[ ] Ajouter ImmediateFloat(f64) (constantes flottantes en .data/.rodata)
[ ] Séparer Label et Sym
[ ] Revoir l'ordre dst/src de StoreF (le registre est dans dst)
[ ] Ajouter tests automatiques :
    [ ] encodage (comparer les octets à une référence)
    [ ] objdump
    [ ] ndisasm
    [ ] décodage

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

[ ] Ajouter SSE2 minimum :
    [ ] Movaps
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
