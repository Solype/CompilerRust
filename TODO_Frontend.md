TODO front-end (langage coréen, voir LANGUAGE_DESIGN.md)

Principe : faire traverser toute la chaîne à un programme minuscule, puis élargir.
Un commit par étape ; on ne passe à la suivante que quand les tests passent.

==================================================
ORGANISATION
==================================================

src/
  lexer/      tokens + positions
  hangeul/    décomposition des syllabes, 받침, conjugaison
  parser/     pré-parser (découpage des particules, mots classés) puis
              tokens -> AST
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

But : texte -> Vec<LexerToken>, chaque token avec un Span en octets.
Le lexer ne connaît AUCUN mot-clé : 정수를, 만약, 줘요 sont tous des mots
hangul. Comme les particules sont collées (정수를 = 정수 + 를), c'est le parser
qui reconnaît les mots-clés en découpant les mots.

--------------------------------------------------
1.1 Fichiers
--------------------------------------------------

[X] src/lexer/mod.rs     pub fn tokenize(src: &str) -> Result<Vec<LexerToken>, LexError>
[X] src/lexer/span.rs    Span
[X] src/lexer/lexer_token.rs   LexerToken, LexerTokenKind
[X] src/lexer/lexer_error.rs   LexError, LexErrorKind
[X] main.rs : mod lexer; appeler tokenize sur la source lue

--------------------------------------------------
1.2 Types
--------------------------------------------------

[X] Span { start: usize, end: usize }
    octets, end exclu : &src[span.start..span.end] redonne le texte du token

[X] LexerToken { kind: LexerTokenKind, span: Span }

[X] LexerTokenKind :
    Mots et littéraux
      HangulWord(String)     나를, 정수를, 해요
      LatinWord(String)      main, printf
      Int(i64)               42 (sans signe)
      Str(String)            contenu décodé : "%ld\n" -> %ld + vrai saut de ligne
    Ponctuation              Punctuation(Punctuation), enum à part :
      LParen RParen LBrace RBrace Comma
      Dot                    .
      Ellipsis               … ou ...
    Opérateurs               Operator(Operator), enum à part :
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
      7:17  Punctuation LParen
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

[X] Décomposer / recomposer une syllabe :
    code = c - 0xAC00 ; initiale = code / 588 ;
    voyelle = (code % 588) / 28 ; finale = code % 28
[X] a_batchim(syllabe) : finale != 0 (을/를, 이에요/예요, 과/와, (으)로)
[X] Conjugaison réguliers + 하다 : -아/어서, -아/어요, -(으)ㄴ
    (tableaux de LANGUAGE_DESIGN.md)

Vérification : tests unitaires
    더하다 -> 더해서 / 더해요 / 더한     빼다 -> 빼서 / 빼요 / 뺀
    보다 -> 봐요    마시다 -> 마셔요    만들다 -> 만든
    쓰다 -> 써요    세다 -> 세요

==================================================
ÉTAPE 2.5 : PRÉ-PARSER (src/parser/pre_parser.rs)
==================================================

But : Vec<LexerToken> -> Vec<ParserToken> où chaque mot hangul est découpé
(나를 -> 나 + 를) et classé : Keyword, Type, Name, Verb + terminaison,
Particle, Bool. Le parser ne voit plus que des catégories fixes.

[X] Mots-clés, types, particules seules après une valeur (42를, 0이에요)
[X] Mot-clé / type + particule (정수를, 값이에요, 10개예요)
[X] Verbes : première passe, tout mot en 다 est un verbe ; table
    « forme conjuguée -> (infinitif, terminaison) », appel avant la définition
[X] Verbes intégrés : 넣다, 바꾸다, 하다 (printf해요)
[X] Warning : verbe en 다 sans ( derrière (바다 est sans doute un nom)
[X] Noms déclarés : paramètres (정수 가,), variables (결과는 en début
    d'instruction), variable de boucle (정수 칸을)
[X] Noms utilisés découpés : plus long nom déclaré en tête, reste = particule
    qu'un nom peut porter (pas 로 ni 의)
[X] Erreur UnknownIdentifier : un mot qui n'est rien de tout ça
[X] Particule après un nom latin (printf를, main을)
[X] & juste après un type = Type(Address), répétable (정수&&) ; déclare le
    paramètre qui suit (정수& 가)
[X] 부호 없는 -> Keyword(Unsigned), un seul token sur les deux mots
[X] 참 / 거짓 -> Bool, seuls ou avec particule (참이면)
[X] Option --pre-tokens
[ ] Portées : aujourd'hui les noms sont ceux de tout le fichier, sans portée
    (nom utilisé hors de sa portée accepté) ; à faire avec le parser

--------------------------------------------------
2.5.1 Vérification des particules selon le 받침 (étape suivante)
--------------------------------------------------

Seules les particules à deux formes : 을/를, 이/가, 은/는, 과/와, (으)로,
이면/면, 이에요/예요 (에, 의, 부터, 까지, 인 ne changent pas). Le 받침 est
celui de la dernière syllabe PRONONCÉE, et pas seulement sur les noms :

[ ] Nom hangul (결과를, 가를) : dernière syllabe du nom
    -> split_name (declared_names, tokenize_names)
[ ] Type / mot-clé / booléen (정수를, 값이에요, 참이면) : dernière syllabe
    -> hangul_word, étape 3
[ ] Nombre (3을, 4를, 0이에요) : lecture sino-coréenne, exacte
    -> hangul_word, étape 2 (prev = Int)
      n % 10 == 0 : 받침 (영, 십, 백, 천, 만, 억 en ont tous un)
      sinon dernier chiffre : 일 삼 육 칠 팔 -> 받침 ; 이 사 오 구 -> non
      le signe ne compte pas (-1을 = 마이너스 일)
[ ] ) (가 + 나)를, ((결과 << 1) ^ 3)을 : 받침 du dernier token avant ),
    récursivement si c'est un autre ) -> hangul_word, étape 2 (prev = RParen)
[ ] Nom latin (printf를, main을) : prononciation anglaise, pas fiable ;
    heuristique sur le nom coréen de la dernière lettre (l 엘, m 엠, n 엔,
    r 알 -> 받침) ; accepter les deux formes, au plus un warning
[ ] Chaîne ("%ld\n"과) : pas de règle, accepter les deux formes
[ ] (으)로 : 로 après un 받침 ㄹ (결말로), pas 으로 ; aujourd'hui (으)로 ne
    suit qu'un type et aucun ne finit par ㄹ

==================================================
ÉTAPE 3 : PREMIÈRE TRANCHE, MAIN QUI REND 42
==================================================

tests/kr/42.kr :
    정수를 주는 main() {
        42를 줘요.
    }

[ ] Parser à table (LR) : automate à états + pile
    [ ] Grammaire sur les ParserToken (terminaux = catégories du pré-parser)
    [ ] Tables ACTION (shift état / reduce règle / accept / erreur) et GOTO
        (état x non-terminal -> état)
    [ ] Boucle : pile d'états (et de valeurs AST) ; shift empile, reduce dépile
        |règle| éléments, construit le nœud, puis GOTO sur le non-terminal
    [ ] Erreur : état sans action pour le token -> message avec les tokens
        attendus (ceux qui ont une action dans cet état)
    [ ] AST : Program { functions }, Function { nom, params, type_retour, corps },
        Stmt::Return(Expr), Expr::Int
    [X] Particules, cas simple : mot-clé connu en tête (정수를 -> 정수 + 를),
        ou particule seule (를 après 42) -> pré-parser
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

[ ] Parser : précédence du C dans les tables, un non-terminal par niveau
    (ou règles de précédence / associativité pour trancher les conflits
    shift/reduce)
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

[X] Première passe sur les déclarations : table des verbes
    (forme conjuguée -> fonction + usage), pour appeler une fonction définie plus bas
    -> pré-parser
[X] Découpage complet des particules : plus long nom déclaré en tête,
    reste = particule valide -> pré-parser
[ ] Pile de portées dans le parser
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
[ ] Appel NOM_LATIN해요 / 해서 (tokens déjà prêts : Name + Verb 하다)
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
[ ] Pointeurs (TYPE 주소, ou TYPE& : & après un mot de type = 주소) et tableaux (TYPE N개)
[ ] Verbes irréguliers (table + annotation)
[ ] Allocation de registres à la place du codegen en pile
