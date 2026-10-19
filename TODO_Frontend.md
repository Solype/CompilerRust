TODO front-end (langage coréen, voir LANGUAGE_DESIGN.md)

Principe : faire traverser toute la chaîne à un programme minuscule, puis élargir.
Un commit par étape ; on ne passe à la suivante que quand les tests passent.

==================================================
ORGANISATION
==================================================

src/
  lexer/      tokens + positions
  hangeul/    décomposition des syllabes, 받침, conjugaison
  parser/     preparser/ (découpage des particules, mots classés),
              cfg/ (grammaire, table LR(1), arbre de dérivation),
              postparser/ (arbre -> AST, types de l'AST dans structs.rs)
  converter/  analyse sémantique de l'AST, puis IR typée (étape 2.6)
  codegen/    IR typée -> Vec<Instruction> (à faire)
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
    (retirée dans 5aeb848, remplacée par --tree)
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
ÉTAPE 2.5 : PRÉ-PARSER (src/parser/preparser/)
==================================================

But : Vec<LexerToken> -> Vec<ParserToken> où chaque mot hangul est découpé
(나를 -> 나 + 를) et classé : Keyword, Type, Name, Verb + terminaison,
Particle, Bool. Le parser ne voit plus que des catégories fixes.

[X] Mots-clés, types, particules seules après une valeur (42를, 0이에요)
[X] Mot-clé / type + particule (정수를, 값이에요, 10개예요)
[X] Verbes : première passe, tout mot en 다 est un verbe ; table
    « forme conjuguée -> (infinitif, terminaison) », appel avant la définition
[X] Verbes intégrés : 넣다, 바꾸다, 하다 (printf해요)
[X] Warning : verbe en 다 sans ( ni { derrière (바다 est sans doute un nom)
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
[X] Option --pre-tokens (retirée dans 5aeb848)
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
ÉTAPE 2.6 : CONVERTER, ANALYSE SÉMANTIQUE (src/converter/)
==================================================

But : vérifier l'AST avant le codegen. Erreurs ConvertError { kind, span }
(convert_error.rs), affichées fichier:ligne:colonne comme les autres.
Ordre : noms et portées, puis type de chaque expression (règle appels,
retours, opérateurs), puis chemins.

--------------------------------------------------
2.6.1 Fonctions
--------------------------------------------------

[X] Table des fonctions avant les corps (get_all_functions) : appel d'une
    fonction définie plus bas permis
[X] Nom complet : nom$type$type, $... si variadique
[X] 외부 et main sans mangling (le linker et la libc cherchent le nom seul)
[X] Erreurs SameSignature (même nom, mêmes types) et SameSymbol (deux 외부 ou
    main de même nom)
[ ] Message trompeur : printf() défini sans paramètre et 외부 printf donnent
    SameSignature au lieu d'un conflit avec le 외부
[X] main existe et renvoie un 정수 (NoMain, MainNotInteger)
[ ] main sans paramètre (ou argc / argv) ; choisir si seul 정수 (long) passe
    ou 짧은 정수 (int) : aujourd'hui tout 정수 est accepté

--------------------------------------------------
2.6.2 Noms et portées
--------------------------------------------------

[X] Pile de portées : une table des variables par bloc (scope.rs)
[X] Variable déjà déclarée dans le même bloc (AlreadyDeclared) ; masquage
    permis dans un bloc intérieur
[X] Variable utilisée avant sa déclaration ou inconnue (ExprKind::Name,
    cible d'Assign, variable de 세면서) ; les fonctions, elles, peuvent être
    définies après
[ ] Variable lue après Declare sans valeur (erreur ou warning)
[X] Deux paramètres de même nom
[ ] Variable locale du nom d'un paramètre : aujourd'hui permise (le corps est
    un bloc à l'intérieur des paramètres, donc masquage)
[X] For : ty Some = déclare la variable, visible dans le corps seulement ;
    ty None = la variable doit exister et ne pas être 고정된
[ ] Type::Custom inconnu (pas de déclaration de type : toujours une erreur
    pour l'instant)

--------------------------------------------------
2.6.3 Appels
--------------------------------------------------

[X] Verbe sans aucune fonction de ce nom (UnknownFunction)
[X] Choix de la surcharge : nom complet depuis les types des arguments,
    puis recherche dans functions ; sinon les fonctions dont les paramètres
    prennent les littéraux, plusieurs = AmbiguousCall ; variadique : types
    des premiers arguments puis $...
[X] Résultat d'une fonction sans retour (return_type None) utilisé dans une
    expression ou un Init (NoValue)
[ ] 외부 (extern) : trouvés par leur nom et leur nombre d'arguments seuls ;
    vérifier les paramètres fixes avec fits (1.5를 abs해요 passe aujourd'hui)
[ ] Verbes intégrés : 바꾸다 (cast) rend le type de son Argument::ToType sans
    regarder la valeur ; règle des casts permis à fixer (nombre -> nombre,
    주소 -> 주소...) ; particules selon le rôle attendu

--------------------------------------------------
2.6.4 Retour
--------------------------------------------------

[X] Return dans une fonction sans type de retour (ReturnWithoutType)
[X] Type de la valeur différent de return_type (WrongReturnType)
[X] Fonction avec retour dont un chemin n'atteint aucun Return (If sans
    otherwise...) : MissingReturn
[ ] 참인 동안 { … 반환해요 } sans rien après la boucle donne MissingReturn :
    compter un 동안 sur le littéral 참 sans 그만해요
[ ] Code après Return / Break / Continue (warning)

--------------------------------------------------
2.6.5 Types
--------------------------------------------------

[X] Init / Assign : type de la valeur = type de la variable (WrongAssignType)
    ; conversions implicites : un littéral prend le type attendu de sa
    famille (0 -> 짧은 정수, 1.5 -> 짧은 소수), aucune autre (pas de
    littéral entier -> 소수)
[X] Binaires : arithmétique sur nombres ; Rem, bit à bit, décalages sur
    entiers ; And / Or sur 논리 ; comparaisons entre deux valeurs du même type
    (WrongOperands)
[X] Unaires : Neg sur nombre, BitNot sur entier, Not sur 논리
[X] Condition d'If / While : 논리 (NotBool)
[X] For : from, until et variable entiers, du même type (NotInteger)
[X] Littéral qui ne tient pas dans son type (짧은 = 32 bits, négatif dans
    부호 없는) : LiteralOutOfRange
[X] Null seulement vers une Address
[ ] Array de taille 0 ; affectation d'un tableau entier

--------------------------------------------------
2.6.6 Constantes et contrôle
--------------------------------------------------

[X] Assign vers un 고정된 (AssignToConstant), 세면서 compris
[X] Assign vers une fonction au lieu d'une variable (UnknownVariable)
[X] Break / Continue hors boucle (BreakOutOfLoop, ContinueOutOfLoop)
[ ] convert_return : le cas None de scope.function() ne peut pas arriver
    (unreachable!() à la place de ReturnWithoutType(""))
[ ] Tests unitaires du converter (aujourd'hui : samples de sample/erreurs/)

--------------------------------------------------
2.6.7 Sortie pour le codegen : IR typée (src/converter/ir/)
--------------------------------------------------

convert() rend un ir::Program : code à trois adresses, une ir::Function par
fonction (symbole, vars: Vec<Type>, labels, body: Vec<Instr>). L'IR est
affichée après l'AST, chaque instruction source en // commentaire.

Fait :
[X] Structures de l'IR (Function, VarId, Label, Operand, Instr, Program)
[X] Symbole résolu de chaque fonction : 더하다$int$int ; 외부 et main gardent
    leur nom (mangled_name == name)
[X] resolve_call rend la fonction appelée ou le cast 바꾸다 (Callee)
[X] Chaque variable a un VarId unique dans sa fonction (new_var) : params,
    Declare, Init, variable du 세면서
[X] Labels de boucle : generate_loop_label (block_count), label_id (nom ->
    Label, ajouté la première fois)
[X] Init / Assign d'un littéral ou d'une variable : Copy (operand)
[X] Affichage de l'IR (ir/display.rs, Instr::Comment)

Instructions IR manquantes :
[ ] convert_expr (expressions.rs) : descendre une expression en
    instructions, rendre (Operand, Type) ; temporaires par new_var
    [ ] Unary, Binary
    [ ] Call : dest temporaire, arguments convertis
    [ ] Cast (바꾸다) : Instr::Cast
    [ ] Str : chaîne ajoutée à Program.strings, Operand::Str(index)
    [ ] SizeOf : Operand::Int(taille du type)
    [ ] 그리고 / 또는 en court-circuit : JumpIfNot, jamais Binary And / Or
[ ] convert_init / convert_assign : valeur composée (calcul, appel, chaîne,
    sizeof) -> aucun Copy aujourd'hui, à brancher sur convert_expr
[ ] convert_condition : rendre l'Operand de la condition, pas seulement
    vérifier le type
[ ] convert_if : JumpIfNot vers le else, instructions du then et du else
    (rendues par convert_block puis jetées), Jump vers la fin, labels
[ ] convert_while_loop : Label début, condition, JumpIfNot fin, corps (rendu
    puis jeté), Jump début, Label fin
[ ] convert_for_loop : Copy de from dans la variable, condition variable <=
    until + JumpIfNot fin, incrément (Binary Add 1) ; Label début, Jump et
    Label fin déjà émis
[ ] convert_break : Jump(label_id(label_end)) ; il faut passer ir_func
[ ] convert_continue : Jump vers le début (동안) ou vers l'incrément
    (세면서, 3e label)
[ ] convert_return : Instr::Return(operand)
[ ] Fonction sans 반환하는 : Return implicite en fin de corps
    (Return(Option<Operand>))
[ ] convert_call : arguments de l'appel (args: vec![] aujourd'hui) ; bras
    Callee::Cast vide
[ ] convert_func : remplir ir_func.params
[ ] convert() : remplir Program.strings et Program.floats

Choix de l'IR à trancher :
[ ] Type des opérandes dans Binary / Unary (signé ou non, 32 / 64 bits,
    소수) : comparaisons, Div, Rem, Shr en dépendent, pas dest
[ ] Operand::Float(f64) ou index dans Program.floats, pas les deux
[ ] Pointeurs et tableaux : Load / Store / AddrOf
[ ] Labels : String (generate_loop_label) et Label(usize) (label_id), un
    seul des deux à terme

==================================================
ÉTAPE 3 : PREMIÈRE TRANCHE, MAIN QUI REND 42
==================================================

tests/kr/42.kr :
    정수를 반환하는 main() {
        42를 반환해요.
    }

[ ] Parser à table (LR(1)) : automate à états + pile, dans src/parser/cfg/
    [X] Grammaire sur les ParserToken (terminaux = catégories du pré-parser) :
        RULES dans rules.rs, macros rule! et term!
    [X] token_matches_node : Name, Int, Str, Bool comparés sur le genre,
        Verb sur la terminaison, le reste avec ==
    [X] Annulables et FIRST (first.rs), FIRST d'une suite (of_sequence)
    [X] Items LR(1) : (règle, position du point, lookahead) ; un état = un
        ensemble d'items sous forme canonique
    [X] Clôture : [A -> α . B β, a] ajoute [B -> . γ, b] pour b dans FIRST(β a)
    [X] goto(I, X) : avancer le point sur X, puis clôture
    [X] Collection des états : état 0 = clôture de
        [StartSymbol -> . Items Eof, Eof], file de travail, transitions notées
    [X] Tables ACTION (shift état / reduce règle / accept / erreur) et GOTO
        (état x non-terminal -> état) ; accept sur Eof dans
        [StartSymbol -> Items . Eof]
    [ ] Conflits : signalés avec l'état, ses items, le terminal et les deux
        actions ; test « zéro conflit » sur RULES
        (aujourd'hui : seulement le nombre de conflits, sur stderr)
    [ ] (optionnel) LALR(1) : fusion des états de même cœur si la table est
        trop grosse
    [X] Boucle : pile d'états ; shift empile, reduce dépile |règle| éléments,
        construit le nœud de l'arbre de dérivation, puis GOTO sur le non-terminal
    [ ] Erreur : état sans action pour le token -> message avec les tokens
        attendus (ceux qui ont une action dans cet état)
        (aujourd'hui : « syntax error: unexpected <token> »)
    [X] Tests : 42.kr puis Proto.kr acceptés, programmes faux refusés
    [X] AST (postparser/structs.rs) : Program, Function, Statement, Expr, Type,
        avec un Span ; option --tree pour l'arbre de dérivation
    [X] Particules, cas simple : mot-clé connu en tête (정수를 -> 정수 + 를),
        ou particule seule (를 après 42) -> pré-parser
[ ] Codegen : mov rax, 42 ; ret
[ ] Écrire un .o avec main global (réutiliser add_section / add_function)
[X] Retirer les samples de main.rs : seulement avec --demo
[ ] Lien : gcc fichier.o -o prog (la libc fournit _start)
[ ] Script de test : compile chaque tests/kr/*.kr, lance le binaire, compare
    code de sortie et stdout avec un .expected

Vérification : ./prog; echo $? -> 42

==================================================
ÉTAPE 4 : EXPRESSIONS ARITHMÉTIQUES
==================================================

[X] Parser : précédence du C dans les tables, un non-terminal par niveau
    (ou règles de précédence / associativité pour trancher les conflits
    shift/reduce)
[X] - unaire, et "- suivi d'un nombre" = terme simple
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
[X] Parsing d'une chaîne : arguments + particules, verbe conjugué ;
    résultat précédent = premier argument implicite
[X] Destination NOM에 en tête -> argument du dernier verbe
[X] Déclaration en phrase : 두 정수 가와 나를 받아 비교하고 정수를 반환 { … },
    sans paramètre 인사하고 정수를 반환 { … } ou 인사하다 { … }
[ ] Déclaration en phrase avec paramètres sans retour (정수 가를 받아 보여준다)
[ ] Codegen :
    [ ] SysV : rdi, rsi, rdx, rcx, r8, r9 ; retour dans rax
    [ ] Prologue push rbp ; mov rbp, rsp ; sub rsp, N (N multiple de 16)
    [ ] Paramètres recopiés sur la pile ([rbp-8], [rbp-16], ...)

Vérification : 3과 4를 더해서 제곱해서 반환해요 -> 49

==================================================
ÉTAPE 6 : VARIABLES ET AFFECTATION
==================================================

[X] Déclarations : 개수는 0이에요 / 개수는 정수예요 (zéro du type) / 고정된 ...
    (parser et AST)
[ ] Une case de pile par variable
[X] Affectation : 넣어요, destination en tête ou en fin de chaîne (parser et AST)
[ ] Erreurs :
    [X] affecter un 고정된 (converter)
    [X] 고정된 avec un type seul (ConstantWithoutValue)
    [X] nom non déclaré (UnknownIdentifier, sans portées)
    [X] double déclaration (converter, AlreadyDeclared)
[ ] Messages : fichier:ligne:colonne, ligne fautive, ^ sous l'erreur
    (une syllabe hangul occupe 2 colonnes dans le terminal)

==================================================
ÉTAPE 7 : CONTRÔLE DE FLUX
==================================================

[X] Parser et AST : 만약 / 아니면 만약 / 아니면, 동안, 세면서, 그리고 / 또는,
    négations, 그만해요 / 넘어가요 ; reste le codegen ci-dessous
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

[X] Parser et AST : 외부 (variadique …), appels NOM_LATIN해요, chaînes ;
    reste le codegen ci-dessous
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

[X] Passe sémantique séparée (vérification des types) : src/converter/
[ ] 소수 (SSE, déjà prêt dans le backend) ; littéraux déjà dans le lexer et l'AST
[ ] Pointeurs (TYPE 주소, ou TYPE& : & après un mot de type = 주소) et tableaux (TYPE N개) ;
    types déjà dans le parser et l'AST, reste le codegen
[ ] Structures (구조) : à concevoir d'abord dans LANGUAGE_DESIGN.md (ligne vide),
    après le codegen (tailles, alignement, offsets)
[ ] Verbes irréguliers (table + annotation)
[ ] Allocation de registres à la place du codegen en pile
