TODO front-end (langage coréen, voir LANGUAGE_DESIGN.md)

Principe : faire traverser toute la chaîne à un programme minuscule, puis élargir.
Un commit par étape ; on ne passe à la suivante que quand les tests passent.

==================================================
ORGANISATION
==================================================

src/
  lexer/      tokens + positions
  hangul/     décomposition des syllabes, 받침, conjugaison
  parser/     tokens -> AST (avec découpage des particules)
  ast.rs      types de l'AST
  codegen/    AST -> Vec<Instruction>
  elf/        (existant)

Le lexer est écrit à la main (l'ancien automate lexical_analisys/ est supprimé).

==================================================
FAIT
==================================================

[X] CLI : ./compiler fichier.kr lit le fichier (UTF-8), erreur + code 1 si
    argument manquant ou fichier illisible
[X] Proto.kr : programme de référence, valeurs attendues en commentaire

==================================================
ÉTAPE 1 : LEXER
==================================================

But : texte -> Vec<Token>, chaque token avec un Span en octets.
Le lexer ne connaît AUCUN mot-clé : 정수를, 만약, 줘요 sont tous des mots
hangul. Comme les particules sont collées (정수를 = 정수 + 를), c'est le parser
qui reconnaît les mots-clés en découpant les mots.

--------------------------------------------------
1.1 Fichiers
--------------------------------------------------

[X] src/lexer/mod.rs     pub fn tokenize(src: &str) -> Result<Vec<Token>, LexError>
[X] src/lexer/span.rs    Span
[X] src/lexer/token.rs   Token, TokenKind
[X] src/lexer/lexer_error.rs   LexError, LexErrorKind
[X] main.rs : mod lexer; appeler tokenize sur la source lue

--------------------------------------------------
1.2 Types
--------------------------------------------------

[X] Span { start: usize, end: usize }
    octets, end exclu : &src[span.start..span.end] redonne le texte du token

[X] Token { kind: TokenKind, span: Span }

[X] TokenKind :
    Mots et littéraux
      HangulWord(String)     나를, 정수를, 해요
      LatinWord(String)      main, printf
      Int(i64)               42 (sans signe)
      Str(String)            contenu décodé : "%ld\n" -> %ld + vrai saut de ligne
    Ponctuation
      LParen RParen LBrace RBrace Comma
      Dot                    .
      Ellipsis               … ou ...
    Opérateurs
      Plus Minus Star Slash Percent      + - * / %
      Amp Pipe Caret Tilde               & | ^ ~
      Shl Shr                            << >>
      Lt Gt Le Ge EqEq NotEq             < > <= >= == !=
    Fin
      Eof                    span = len..len, simplifie le parser

[X] LexError { kind: LexErrorKind, span: Span }

[X] LexErrorKind :
      UnexpectedChar(char)   # @ $ ; etc.
      UnterminatedComment    /* sans */ ; span = le /* d'ouverture
      UnterminatedString     " sans " avant la fin de ligne ; span = le " d'ouverture
      InvalidEscape(char)    \q ; span = les 2 caractères
      IntegerOverflow        > i64::MAX ; span = le nombre
      LoneEquals             = seul ; message : « l'affectation s'écrit 넣어요 »
      LoneBang               ! seul ; message : « la négation s'écrit 아니면 »
      LooseJamo(char)        ㄱ, ㅏ... (jamo isolés, U+3131 à U+318E) ;
                             message : « syllabe incomplète »

--------------------------------------------------
1.3 Expressions régulières (une par type de token)
--------------------------------------------------

    Ignorés
      espaces             [ \t\r\n]+
      commentaire ligne   //[^\n]*
      commentaire bloc    /\*[\s\S]*?\*/          (non imbriqué, le plus court)

    Mots
      mot hangul          [가-힣]+                (syllabes U+AC00 à U+D7A3)
      mot latin           [A-Za-z_]+              (pas de chiffres, cf. décisions)

    Littéraux
      entier              [0-9]+                  (pas de signe : - est un token)
      flottant (plus tard) [0-9]+\.[0-9]+         (un chiffre après le point,
                                                   sinon 줘요. serait ambigu)
      chaîne              "([^"\\\n]|\\[nt0\\"])*"

    Ponctuation
      parenthèses, etc.   [(){},]
      variadique          …|\.\.\.                (avant le point simple)
      point final         \.                      (accepté seulement après 요 :
                                                   vérifié par le parser)

    Opérateurs (le plus long d'abord)
      sur 2 caractères    <<|>>|<=|>=|==|!=
      sur 1 caractère     [-+*/%&|^~<>]

    Erreur
      tout le reste       .                       (= et ! seuls n'existent pas)

Règles de priorité :
    - plus longue correspondance (maximal munch) : <= avant <, ... avant .,
      // et /* avant l'opérateur /
    - le changement d'écriture coupe tout seul : [가-힣]+ ne mange pas de latin,
      [A-Za-z_]+ pas de hangul, [0-9]+ ni l'un ni l'autre (42를, printf해요)

--------------------------------------------------
1.4 Algorithme (version écrite à la main)
--------------------------------------------------

[X] État : struct Lexer<'a> { src: &'a str, pos: usize }   (pos en octets)
[X] Outils :
      peek()    -> Option<char>   src[pos..].chars().next()
      peek_at(n)-> Option<char>   n-ième caractère après pos
      bump()    -> Option<char>   avance de c.len_utf8() octets
      eat_while(f)                avance tant que f(c)
[X] Boucle : sauter espaces et commentaires, noter start = pos, puis match
    sur le premier caractère :

      ' ' '\t' '\r' '\n'   espace : sauter
      '/'                  '/' ensuite : commentaire ligne jusqu'au \n
                           '*' ensuite : chercher "*/" (src[pos..].find),
                                         absent -> UnterminatedComment
                           sinon : Slash
      '가'..='힣'          eat_while(hangul) -> HangulWord(src[start..pos])
      'a'..='z' 'A'..='Z' '_'
                           eat_while(latin ou _) -> LatinWord
      '0'..='9'            eat_while(chiffre), parse::<i64>() ;
                           échec -> IntegerOverflow
      '"'                  boucle :
                             fin du texte ou '\n' -> UnterminatedString
                             '\\' -> n t 0 \ " sinon InvalidEscape
                             '"'  -> fin, Str(contenu décodé)
                             autre -> ajouter au contenu
      '(' ')' '{' '}' ','  token simple
      '.'                  ".." ensuite -> Ellipsis (3 octets), sinon Dot
      '…'                  Ellipsis (3 octets en UTF-8)
      '<'                  '<' -> Shl, '=' -> Le, sinon Lt
      '>'                  '>' -> Shr, '=' -> Ge, sinon Gt
      '='                  '=' -> EqEq, sinon LoneEquals
      '!'                  '=' -> NotEq, sinon LoneBang
      '+' '-' '*' '%' '&' '|' '^' '~'
                           token simple
      'ㄱ'..='ㆎ'          LooseJamo
      autre                UnexpectedChar

[X] À la fin : pousser Eof
[X] Erreurs : s'arrêter à la première (plus simple pour commencer)

Pièges :
    - jamais d'index de caractère : pos avance de c.len_utf8()
    - un match sur des plages de char ('가'..='힣') marche en Rust : les char
      sont ordonnés par point de code
    - is_alphabetic() accepterait aussi le chinois, le cyrillique... :
      utiliser les plages exactes

--------------------------------------------------
1.5 Position ligne:colonne
--------------------------------------------------

[X] Calculée seulement à l'affichage, depuis l'offset :
      ligne   = nombre de '\n' dans src[..offset] + 1
      colonne = nombre de char depuis le dernier '\n' + 1
[ ] Plus tard (messages avec ^) : largeur d'affichage, 2 colonnes par syllabe
    hangul

--------------------------------------------------
1.6 Option --tokens
--------------------------------------------------

[X] ./compiler --tokens fichier.kr : un token par ligne, puis quitter
[X] Adapter read_source() dans main.rs : il refuse aujourd'hui tout argument
    en plus du fichier
    format libre, par exemple pour la ligne 7 de Proto.kr :
      7:1   HangulWord  외부
      7:4   HangulWord  정수를
      7:8   HangulWord  주는
      7:11  LatinWord   printf
      7:17  LParen
[X] Erreur : fichier:ligne:colonne: message, code de sortie 1

--------------------------------------------------
1.7 Tests (src/lexer/lexer_tests.rs)
--------------------------------------------------

Cas valides (entrée -> tokens, sans Eof) :
[X] 42를 줘요.             Int(42) HangulWord(를) HangulWord(줘요) Dot
[X] printf해요             LatinWord(printf) HangulWord(해요)
[X] 가-5                   HangulWord(가) Minus Int(5)
[X] -5를                   Minus Int(5) HangulWord(를)
[X] (가 + 나)를            LParen HangulWord(가) Plus HangulWord(나) RParen HangulWord(를)
[X] 가 <= 나               HangulWord(가) Le HangulWord(나)
[X] 1 << 2 >> 3            Int(1) Shl Int(2) Shr Int(3)
[X] 가 == 나 != 다         ... EqEq ... NotEq ...
[X] "%ld\n"과              Str("%ld" + saut de ligne) HangulWord(과)
[X] (형식, …) et (형식, ...)  ... Comma Ellipsis RParen (les deux)
[X] /* a */ 1 // b\n2      Int(1) Int(2)
[X] 정수를                 HangulWord, span 0..9 (3 syllabes x 3 octets)
[X] chaîne vide            [] (seulement Eof)
[X] Proto.kr entier        aucune erreur

Cas d'erreur (entrée -> erreur, span) :
[X] /* abc                 UnterminatedComment, 0..2
[X] "abc                   UnterminatedString, 0..1
[X] "a\nb" (vrai saut)     UnterminatedString
[X] "\q"                   InvalidEscape('q')
[X] 가 = 1                 LoneEquals
[X] !가                    LoneBang
[X] 99999999999999999999   IntegerOverflow
[X] #                      UnexpectedChar('#')
[X] ㄱ                     LooseJamo('ㄱ')

--------------------------------------------------
1.8 Si tu choisis logos plutôt que la version à la main
--------------------------------------------------

[ ] Cargo.toml : logos
[ ] Un attribut par variante : #[regex("[가-힣]+")], #[token("<<")],
    #[regex(r"[ \t\r\n]+", logos::skip)]
[ ] Commentaire bloc sans quantificateur paresseux :
    /\*([^*]|\*+[^*/])*\*+/
[ ] Erreurs précises (non fermé, = seul...) : variantes dédiées ou callbacks,
    sinon logos ne dit que « erreur à cette position »
[ ] Valeurs (i64, contenu de chaîne décodé) : callbacks sur les variantes

==================================================
ÉTAPE 2 : MODULE HANGUL (indépendant, faisable en parallèle)
==================================================

[ ] Décomposer / recomposer une syllabe :
    code = c - 0xAC00 ; initiale = code / 588 ;
    voyelle = (code % 588) / 28 ; finale = code % 28
[ ] a_batchim(syllabe) : finale != 0 (을/를, 이에요/예요, 과/와, (으)로)
[ ] Conjugaison réguliers + 하다 : -아/어서, -아/어요, -(으)ㄴ
    (tableaux de LANGUAGE_DESIGN.md)

Vérification : tests unitaires
    더하다 -> 더해서 / 더해요 / 더한     빼다 -> 빼서 / 빼요 / 뺀
    보다 -> 봐요    마시다 -> 마셔요    만들다 -> 만든
    쓰다 -> 써요    세다 -> 세요

==================================================
ÉTAPE 3 : PREMIÈRE TRANCHE, MAIN QUI REND 42
==================================================

tests/kr/42.kr :
    정수를 주는 main() {
        42를 줘요.
    }

[ ] Parser (descente récursive) :
    [ ] AST : Program { functions }, Function { nom, params, type_retour, corps },
        Stmt::Return(Expr), Expr::Int
    [ ] Particules, cas simple : mot-clé connu en tête (정수를 -> 정수 + 를),
        ou particule seule (를 après 42)
[ ] Codegen : mov rax, 42 ; ret
[ ] Écrire un .o avec main global (réutiliser add_section / add_function)
[ ] Retirer les samples de main.rs (les garder pour les tests de l'encodeur)
[ ] Lien : gcc fichier.o -o prog (la libc fournit _start)
[ ] Script de test : compile chaque tests/kr/*.kr, lance le binaire, compare
    code de sortie et stdout avec un .expected

Vérification : ./prog; echo $? -> 42

==================================================
ÉTAPE 4 : EXPRESSIONS ARITHMÉTIQUES
==================================================

[ ] Parser : precedence climbing (Pratt), table de précédence du C
[ ] - unaire, et "- suivi d'un nombre" = terme simple
[ ] Codegen en pile, résultat dans rax :
    a op b -> calculer a ; push rax ; calculer b ; mov rcx, rax ; pop rax ; op rax, rcx
[ ] / et % : cqo ; idiv rcx (quotient rax, reste rdx)
[ ] Comparaisons : cmp ; setcc al ; movzx rax, al
[ ] Opérateurs bit à bit, décalages (sar si signé, shr si 부호 없는)

Vérification : un .kr par opérateur ; ((3 + 4) * 5)를 줘요 -> 35
(code de sortie limité à 0-255)

==================================================
ÉTAPE 5 : FONCTIONS, PARAMÈTRES, CHAÎNES D'APPELS
==================================================

[ ] Première passe sur les déclarations : table des verbes
    (forme conjuguée -> fonction + usage), pour appeler une fonction définie plus bas
[ ] Découpage complet des particules : plus long nom déclaré en tête,
    reste = particule valide ; pile de portées dans le parser
[ ] Conflit de noms (사 et 사과 dans la même portée) = erreur
[ ] Parsing d'une chaîne : arguments + particules, verbe conjugué ;
    résultat précédent = premier argument implicite
[ ] Destination NOM에 en tête -> argument du dernier verbe
[ ] Codegen :
    [ ] SysV : rdi, rsi, rdx, rcx, r8, r9 ; retour dans rax
    [ ] Prologue push rbp ; mov rbp, rsp ; sub rsp, N (N multiple de 16)
    [ ] Paramètres recopiés sur la pile ([rbp-8], [rbp-16], ...)

Vérification : 3과 4를 더해서 제곱해서 줘요 -> 49

==================================================
ÉTAPE 6 : VARIABLES ET AFFECTATION
==================================================

[ ] Déclarations : 개수는 0이에요 / 개수는 정수예요 (zéro du type) / 고정된 ...
[ ] Une case de pile par variable
[ ] Affectation : 넣어요, destination en tête ou en fin de chaîne
[ ] Erreurs :
    [ ] affecter un 고정된
    [ ] 고정된 avec un type seul
    [ ] nom non déclaré
    [ ] double déclaration
[ ] Messages : fichier:ligne:colonne, ligne fautive, ^ sous l'erreur
    (une syllabe hangul occupe 2 colonnes dans le terminal)

==================================================
ÉTAPE 7 : CONTRÔLE DE FLUX
==================================================

[ ] 만약 ... 이면 / 아니면 만약 / 아니면 (LabelId + jcc)
[ ] COND인 동안
[ ] NOM을 A부터 B까지 세면서 : borne de fin exclue (< et non <=)
[ ] 그리고 / 또는 en court-circuit
[ ] Négation : COND이/가 아니면, COND이/가 아닌 동안
[ ] 그만해요 / 넘어가요 : pile des boucles (label de fin, label de suite) ;
    hors boucle = erreur

==================================================
ÉTAPE 8 : PRINTF, PUIS TOUT PROTO.KR
==================================================

[ ] 외부 : symbole non défini, lié par PLT32
[ ] Appel NOM_LATIN해요 / 해서
[ ] Chaînes littérales en .rodata avec un label ; lea rdi, [rip+label]
[ ] Variadique : xor eax, eax avant le call (al = nb de registres XMM)
[ ] Alignement : rsp % 16 == 0 avant chaque call ; compter les push en
    attente du codegen en pile et corriger

Vérification : sortie de Proto.kr = 35, 69, 1, 285, 4, 128, puis 1, 2, 4, 5

==================================================
APRÈS
==================================================

[ ] Passe sémantique séparée (vérification des types)
[ ] 실수 (SSE, déjà prêt dans le backend)
[ ] Pointeurs (TYPE 주소) et tableaux (TYPE N개)
[ ] Verbes irréguliers (table + annotation)
[ ] Vérification des particules selon le 받침
[ ] Allocation de registres à la place du codegen en pile
