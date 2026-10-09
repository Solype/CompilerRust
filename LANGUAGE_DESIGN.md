# Conception du langage

Mots à définir pour le langage source. Compléter la colonne « Mot » ; laisser vide si le concept
n'existe pas dans le langage. ★ = nécessaire pour la première tranche (`main` qui retourne 42,
puis un appel à `printf`).

Décisions déjà prises :

- Les identifiants n'acceptent pas les chiffres (hangul, latin et `_`).
- Registre : 해요체 partout (`예요`, `줘요`, `넣어요`…).
- Fin d'instruction : la terminaison en `요` termine l'instruction, pas de `;`. Un `.` est accepté
  juste après le `요` (`42를 줘요.`). Les retours à la ligne comptent comme des espaces.
- Commentaires : `// …` jusqu'à la fin de la ligne, et `/* … */`.
- Fonctions de la libc : nom latin + `하다`, comme les mots étrangers en coréen (컴파일하다,
  클릭하다) : `printf해요`, `printf해서`. Le lexer sépare le latin du hangul tout seul.
  Exemple : `"%ld\n"과 가를 printf해요`. Les chiffres dans les noms latins (`log10`, `atan2`)
  restent à décider.
- Particules : toujours collées (`나를`, `정수를`, `42를`, `(가 + 나)를`). Voir
  [Particules collées](#particules-collées) pour le découpage.
- Fonctions = verbes, appels en chaîne sans imbrication : voir [Fonctions et conjugaison](#fonctions-et-conjugaison).
- Retour : une expression avec un opérateur va entre parenthèses (`(가 + 나)를 줘요`), sinon on ne
  sait pas si `를` porte sur `나` ou sur `가 + 나`. Un terme simple (nombre, nom) n'en a pas
  besoin : `42를 줘요`, `가를 줘요`. Après `)`, la particule est collée.
- Arithmétique simple toujours possible : partout où une valeur est attendue (affectation,
  argument, retour, déclaration, condition), une expression avec opérateurs est acceptée, sans
  passer par une fonction. Entre parenthèses si une particule suit : `개수에 (개수 + 1)을 넣어요`
  (`a = a + 1`), `결과에 (가 * 나)를 넣어요` (`a = b * c`).
- Nombre négatif : `-5` est un terme simple (`-5를 줘요`). Le `-` reste un token à part dans le
  lexer (sinon `가-5` donnerait `가` puis `-5`) ; c'est le parser qui colle `-` devant un
  nombre. `-가` reste une expression : `(-가)를 줘요`.

Exemple de référence (première tranche) :

```
정수를 주는 main() {
    42를 줘요
}

정수를 주는 더하다(정수 가, 정수 나) {
    (가 + 나)를 줘요
}

정수를 주는 곱하다(정수 가, 정수 나) {
    (가 * 나)를 줘요
}

정수를 주는 계산하다() {
    3과 4를 더해서 5를 곱해서 줘요       // (3 + 4) * 5
}

인사하다() {
    …
}
```

## Fonctions et conjugaison

### Principe

- Une fonction est un **verbe**, déclaré à la forme du dictionnaire (`더하다`, `계산하다`, `먹다`).
  Seule exception au langage tout coréen : `main`, qui n'est jamais appelé dans le code.
- **`다` est la marque des fonctions** : tout mot qui finit par `다` est un verbe, donc une
  fonction. Un nom de variable ou de paramètre ne peut pas finir par `다` (`바다`, « la mer »). C'est
  ce qui permet au compilateur de reconnaître un verbe sans contexte, même appelé avant sa
  déclaration. Un mot en `다` qui n'est pas suivi de `(` donne un avertissement : c'est sans doute
  un nom mal choisi.
- Les appels **ne s'imbriquent pas** : on compose en chaîne, de gauche à droite.
  Le résultat de l'étape précédente devient le **premier argument** de l'étape suivante.
- Arguments : reliés par `와/과`, le dernier porte `을/를`. Un argument est un terme simple ou une
  expression arithmétique entre parenthèses, jamais un appel.
- Une chaîne se termine par un verbe à la forme `-아/어요` : `줘요` rend le résultat courant ;
  tout autre verbe est un appel dont le résultat est ignoré.
- Résultats intermédiaires multiples : les nommer dans une variable (syntaxe à définir).

```
3과 4를 더해요                 // appel, résultat ignoré
3과 4를 더해서 줘요            // rend 3 + 4
3과 4를 더해서 5를 곱해서 줘요  // rend (3 + 4) * 5
인사해요                       // appel sans argument
```

### Déclaration en phrase

Une fonction peut aussi se déclarer comme une phrase : les paramètres viennent **avant** le verbe,
reliés par les mêmes particules qu'à l'appel (`와/과` entre eux, `을/를` sur le dernier). Les deux
formes sont équivalentes.

```
정수를 주는 비교하다(정수 가, 정수 나) { … }     // forme C
정수를 주는 정수 가와 정수 나를 비교하다 { … }   // forme en phrase

7과 3을 비교해요                                 // l'appel, identique pour les deux
```

- La déclaration ressemble ainsi à l'appel : `정수 가와 정수 나를 비교하다` / `7과 3을 비교해요`.
- La particule suit la dernière syllabe du nom, pas du type : `가와`, `나를` (voyelle), `끝과`,
  `끝을` (consonne).
- Un seul paramètre : il porte `을/를` (`정수 가를 제곱하다`).
- Sans paramètre, la forme C reste la seule : `인사하다() { … }`.

### Formes générées

À la déclaration, le compilateur génère trois formes du verbe et les range dans une table
« forme → (fonction, usage) ». Pas d'analyse à l'envers : le parser cherche le mot dans la table.

| Forme | Usage | 하다 | 먹다 |
| --- | --- | --- | --- |
| `-아/어서` | enchaîner une étape | 해서 | 먹어서 |
| `-아/어요` | finir la chaîne | 해요 | 먹어요 |
| `-(으)ㄴ` | qualifier un nom (`더한 값`) | 한 | 먹은 |

### Surcharge (à faire)

Les fonctions sont des verbes, et le choix de verbes naturels est limité : `더하다` doit pouvoir
additionner des `정수` comme des `실수`. Un même verbe peut donc être déclaré **plusieurs fois**,
avec des paramètres différents.

```
정수를 주는 더하다(정수 가, 정수 나) { (가 + 나)를 줘요 }
실수를 주는 더하다(실수 가, 실수 나) { (가 + 나)를 줘요 }

3과 4를 더해서 줘요          // la version 정수
1.5와 2.5를 더해서 줘요      // la version 실수
```

- Les déclarations d'un même verbe doivent différer par le **nombre** ou les **types** de leurs
  paramètres. Le type de retour seul ne suffit pas : à l'appel, rien ne dit lequel est attendu.
- La version appelée est choisie après le parser, pendant la vérification des types, d'après les
  arguments de l'appel. Dans une chaîne, le résultat de l'étape précédente compte comme premier
  argument : `3과 4를 더해서 2.5를 곱해서` choisit le `곱하다` dont le premier paramètre est un
  `정수`.
- Aucune version ne correspond, ou plusieurs correspondent aussi bien : erreur.
- La table des formes donne le **verbe** (`더해서` → `더하다`), pas une fonction précise : toutes
  ses versions partagent les mêmes formes conjuguées.
- Chaque version a son propre symbole dans le fichier objet (le nom du verbe complété par les
  types des paramètres), pour que l'éditeur de liens les distingue.
- Les fonctions `외부` ne peuvent pas être surchargées : le C n'a qu'un symbole par nom.

### Exemples

Fonctions utilisées dans les exemples :

```
정수를 주는 더하다(정수 가, 정수 나) {
    (가 + 나)를 줘요
}

정수를 주는 빼다(정수 가, 정수 나) {
    (가 - 나)를 줘요
}

정수를 주는 제곱하다(정수 가) {      // 제곱 = carré
    (가 * 가)를 줘요
}

인사하다() {
    …
}
```

| Verbe | `-아/어서` | `-아/어요` | `-(으)ㄴ` |
| --- | --- | --- | --- |
| 더하다 | 더해서 | 더해요 | 더한 |
| 빼다 | 빼서 (ㅐ + 어 → ㅐ) | 빼요 | 뺀 |
| 제곱하다 | 제곱해서 | 제곱해요 | 제곱한 |
| 인사하다 | 인사해서 | 인사해요 | 인사한 |

**Enchaîner (`-아/어서`)** : chaque étape reçoit le résultat précédent comme premier argument.

```
10과 4를 빼서 줘요                       // 10 - 4 = 6
10과 4를 빼서 3을 더해서 줘요            // (10 - 4) + 3 = 9
10과 4를 빼서 3을 더해서 제곱해서 줘요   // ((10 - 4) + 3)² = 81
3과 4를 더해서 제곱해서 2를 빼서 줘요     // (3 + 4)² - 2 = 47
```

`제곱해서` n'a pas d'argument écrit : son seul paramètre est le résultat précédent.
Particules : 10 (십) et 3 (삼) finissent par une consonne → `과`, `을` ; 4 (사) et 2 (이) par une
voyelle → `와`, `를`.

**Finir (`-아/어요`)** : `줘요` rend le résultat ; un autre verbe appelle et ignore le résultat.

```
인사해요                  // appel sans argument
3과 4를 더해요            // calcule 7 et l'ignore
3과 4를 더해서 줘요       // rend 7
인사해서 3과 4를 더해요   // erreur : 인사하다 ne rend rien, rien à passer à 더하다
```

**Qualifier (`-(으)ㄴ`)** : proposition pour nommer un résultat intermédiaire, avec la forme de
variable `NOM 은/는 VALEUR이에요/예요` (`값` = valeur, finale ㅂ → `이에요`).

```
합은 3과 4를 더한 값이에요           // 합 = 3 + 4
차는 10과 4를 뺀 값이에요            // 차 = 10 - 4
합과 차를 더해서 줘요               // 7 + 6 = 13
```

### Verbes réguliers (première étape)

Tout se calcule depuis la dernière syllabe du radical (décomposée en initiale, voyelle, finale à
partir du code Unicode, puis recomposée).

**`-아/어`** : `아` si la dernière voyelle est `ㅏ` ou `ㅗ`, `어` sinon. Avec une finale (받침), pas
de contraction : 받다 → 받아요, 먹다 → 먹어요. Sans finale, la voyelle se contracte :

| Voyelle du radical | Résultat | Exemple |
| --- | --- | --- |
| ㅏ + 아 | ㅏ | 가다 → 가요 |
| ㅗ + 아 | ㅘ | 보다 → 봐요 |
| ㅓ + 어 | ㅓ | 서다 → 서요 |
| ㅜ + 어 | ㅝ | 주다 → 줘요 |
| ㅣ + 어 | ㅕ | 마시다 → 마셔요 |
| ㅐ + 어 | ㅐ | 보내다 → 보내요 |
| ㅔ + 어 | ㅔ | 세다 → 세요 |
| ㅚ + 어 | ㅙ | 되다 → 돼요 |
| ㅡ + 아/어 | ㅡ tombe, harmonie sur la syllabe précédente | 쓰다 → 써요, 바쁘다 → 바빠요 |

**`-(으)ㄴ`** : sans finale, `ㄴ` devient la finale (가다 → 간) ; avec une finale, ajouter `은`
(먹다 → 먹은) ; finale `ㄹ` : elle tombe et `ㄴ` la remplace (만들다 → 만든).

**하다** : cas à part, inclus dès la première étape : 해서, 해요, 한.

### Verbes irréguliers (plus tard)

L'irrégularité dépend du verbe, pas de son orthographe (입다 → 입어요 mais 돕다 → 도와요 ; 묻다
« enterrer » → 묻어요 mais 묻다 « demander » → 물어요). Il faudra donc une table intégrée des
verbes irréguliers courants, et une annotation optionnelle à la déclaration pour les cas ambigus.

| Classe | Exemple | -어요 | -어서 | -(으)ㄴ |
| --- | --- | --- | --- | --- |
| ㅂ | 돕다 | 도와요 | 도와서 | 도운 |
| ㄷ | 듣다 | 들어요 | 들어서 | 들은 |
| ㅅ | 짓다 | 지어요 | 지어서 | 지은 |
| 르 | 부르다 | 불러요 | 불러서 | 부른 |
| 우 | 푸다 | 퍼요 | 퍼서 | 푼 |

En attendant, un verbe dont la finale est `ㅂ`, `ㄷ`, `ㅅ` ou dont le radical finit par `르` peut
être irrégulier : le compilateur le traite comme régulier.

## Particules collées

Les particules s'écrivent collées, comme en coréen. Le lexer ne découpe pas : il produit un **mot
hangul** entier (`나를`), et c'est le **parser** qui le sépare, car il faut connaître les noms
déclarés (même principe que le « lexer hack » du C pour `typedef`).

- **Après un mot-clé** (`정수를`) : le mot-clé est connu, le reste est la particule.
- **Après un nombre** (`42를`) ou **`)`** (`(가 + 나)를`) : le lexer s'arrête naturellement.
- **Déclaration** (`합은 3과 4를 더한 값이에요`) : le nom n'existe pas encore ; on retire la
  particule attendue à la fin du mot (`은/는` ici), une seule fois. Pour une boucle qui déclare sa
  variable (`정수 칸을 0부터 10까지 세면서`), le type annonce la déclaration et c'est `을/를` qu'on
  retire.
- **Utilisation** (`나를`) : on cherche le plus long nom déclaré au début du mot, tel que le reste
  soit une particule valide. `나를` → `나` + `를`.
- **Conflit** : déclarer un nom égal à un autre nom + une particule dans la même portée est une
  erreur (`사` et `사과` : `사과` serait `사` + `과`). Un mot qui est exactement un nom déclaré n'a
  pas de particule.

Après une chaîne littérale (`"%ld\n"과`), `와` et `과` sont acceptés tous les deux : le 받침 ne
se déduit pas du texte.

Particules reconnues : `을/를`, `이/가`, `은/는`, `와/과`, `에`, `(으)로`, `의`, `부터`, `까지`,
et les formes de la copule `이면/면`, `이에요/예요`, `인`. Pas de `보다` (« que ») : il finit par `다`,
la marque des fonctions, et les comparaisons s'écrivent `<` et `>`.

**Au plus une particule par mot, toujours à la fin.** Contrairement au coréen, les particules ne se
cumulent pas : `결과에는` (에 + 는) ou `여기까지만` sont des erreurs. Les formes de la copule
(`이면`, `이에요`, `인`) comptent comme une seule particule. Le découpage compare donc **tout le
reste** du mot à la liste : `값이에요` → `값` + `이에요`, jamais `값` + `이` + `에요`.

Bonus : une fois la particule séparée, le compilateur connaît le mot qui la précède et peut vérifier
le choix selon le 받침 (`나을` → « 받침이 없으니 '를'을 쓰세요 »).

## Déclarations

| | Concept | Mot | Notes |
| --- | --- | --- | --- |
| ★ | fonction | `TYPE을/를 주는 VERBE다(TYPE nom, …) { … }` | « VERBE, qui donne un TYPE » ; sans retour : `VERBE다(…) { … }` |
| | variable | `NOM은/는 VALEUR이에요/예요` ou `NOM은/는 TYPE이에요/예요` | avec une valeur : type déduit (`0` → `정수`, `0.5` → `실수`) ; avec un type : valeur par défaut (zéro du type) ; `이에요` après une consonne, `예요` après une voyelle ; modifiable par défaut |
| | constante | `고정된 NOM은/는 VALEUR이에요/예요` | voir Modificateurs |
| ★ | fonction externe (libc) | `외부 TYPE을/를 주는 nom(TYPE nom, …)` | nom latin, sans corps ; `…` pour les variadiques : `외부 정수를 주는 printf(문자 주소 형식, …)` |
| | structure | 구조 | |
| | exporter / public | | |

```
개수는 0이에요              // 정수, vaut 0
평균은 0.5예요              // 실수, vaut 0.5
개수는 정수예요             // 정수, valeur par défaut 0
이름은 문자 주소예요        // 문자 주소 (char*), vaut 빈 주소
이름은 문자&예요            // pareil, avec l'abréviation &
점수는 정수 10개예요        // tableau de 10 정수, tous à 0
```

Le parser regarde ce qui précède `이에요/예요` : un mot-clé de type donne une déclaration sans
valeur, autre chose une valeur. Valeurs par défaut : `0`, `0.0`, `거짓`, `빈 주소`.
`고정된` avec un type seul (`고정된 개수는 정수예요`) est une erreur : la constante vaudrait zéro
pour toujours.

## Types

| | Concept | Mot | Notes |
| --- | --- | --- | --- |
| ★ | entier (64 bits) | `정수` | `정수를` |
| | entier 32 bits | `짧은 정수` | 짧다 (« être court ») → `짧은` ; pour l'interop C (`int`) |
| | entier 16 bits | | rarement utile, à ajouter si besoin |
| | entier 8 bits | `바이트` | non signé ; octets, chaînes C ; `바이트를` |
| | entier non signé | `부호 없는 정수` | 부호 = signe, 없다 → `없는` ; terme standard en informatique coréenne |
| | flottant (64 bits) | `실수` | « nombre réel » ; homonyme de 실수 « erreur » ; `실수를` |
| | flottant (32 bits) | `짧은 실수` | même modificateur que `짧은 정수` |
| | booléen | `논리` | « logique » ; `논리를` |
| | caractère | `문자` | `문자를` |
| | pointeur | `TYPE 주소` ou `TYPE&` | « adresse » : `정수 주소 가` = `정수& 가` = 가, adresse d'un entier ; `&` est une abréviation de `주소`, collée ou non (`정수&`, `정수 &`), répétable (`정수&&` = `정수 주소 주소`) |
| | tableau | `TYPE N개` | compteur 개 : `정수 10개 가` = 가, 10 entiers ; concept : 배열 |

## Valeurs

| | Concept | Mot | Notes |
| --- | --- | --- | --- |
| | vrai | `참` | finale ㅁ → `참을`, `참이에요` |
| | faux | `거짓` | « mensonge » ; finale ㅅ → `거짓을`, `거짓이에요` |
| | pointeur nul | `빈 주소` | « adresse vide » (비다 → `빈`), sur le modèle du type `TYPE 주소` ; `빈 주소를` |

## Contrôle de flux

| | Concept | Mot | Notes |
| --- | --- | --- | --- |
| ★ | retourner | `(EXPR)을/를 줘요`, ou `TERME을/를 줘요` | 주다 = donner ; 돌아오다 est intransitif (« revenir ») |
| | si (avant la condition) | `만약` | « si, supposons que » ; ouvre la condition, ce qui aide le parser |
| | si (après la condition) | `이면` / `면` | « si c'est » ; `이면` après une consonne, `면` après une voyelle (les deux acceptés au début) |
| | sinon | `아니면` | « si ce n'est pas le cas » |
| | sinon si | `아니면 만약 …이면` | composition de `아니면` et `만약`, comme `else if` |
| | tant que | `COND인 동안` | « pendant que c'est » ; 동안 = durée, pendant |
| | pour (boucle `for`) | `NOM을/를 A부터 B까지 세면서` ou `TYPE NOM을/를 A부터 B까지 세면서` | « en comptant NOM de A à B » ; 부터 = depuis, 까지 = jusqu'à, **B exclu** (`0부터 10까지` = 10 tours, de 0 à 9), comme `0..10` en Rust ; 세다 = compter, -면서 = en faisant ; sans type, NOM existe déjà et la boucle le modifie ; avec un type, la boucle déclare NOM, qui n'existe que dans la boucle (même règle que les paramètres : `TYPE NOM` déclare) |
| | sortir de la boucle (`break`) | `그만해요` | « arrête » (그만하다) |
| | continuer (`continue`) | `넘어가요` | « on passe à la suite » (넘어가다) |

```
만약 가 > 나면 {
    가를 줘요
} 아니면 만약 가 == 나면 {
    0을 줘요
} 아니면 {
    나를 줘요
}

가 < 10인 동안 {
    …
}

가를 1부터 10까지 세면서 {
    만약 가 == 5면 { 넘어가요 }
    만약 가 == 8이면 { 그만해요 }
    …
}

정수 칸을 0부터 10까지 세면서 {     // déclare 칸, comme for (int 칸 = 0; …)
    …
}
```

`만약` n'a pas d'équivalent pour `인 동안` : le parser lit d'abord une expression, puis regarde le
mot suivant. Une particule (`를`, `과`…) annonce une chaîne d'appels, `인 동안` une boucle.

## Opérateurs symboles (repris du C)

Tout opérateur arithmétique du C qui n'a pas de mot coréen garde son symbole, avec la précédence et
l'associativité du C. Seuls `&&`, `||` et `!` sont remplacés par des mots (voir la section
suivante). L'affectation (`=`, `+=`…) et `++` / `--` n'existent pas : on utilise `넣다`.

| Précédence (forte → faible) | Opérateurs | Associativité |
| --- | --- | --- |
| 1 | `-` `~` (unaires) | droite |
| 2 | `*` `/` `%` | gauche |
| 3 | `+` `-` | gauche |
| 4 | `<<` `>>` | gauche |
| 5 | `<` `<=` `>` `>=` | gauche |
| 6 | `==` `!=` | gauche |
| 7 | `&` | gauche |
| 8 | `^` | gauche |
| 9 | `\|` | gauche |
| 10 | `그리고` (`&&`) | gauche |
| 11 | `또는` (`\|\|`) | gauche |

`>>` sur un entier signé est arithmétique (`sar`), sur un `부호 없는` logique (`shr`).

Après un mot de type (`정수`, `문자`…), `&` n'est pas le ET bit à bit mais l'abréviation de
`주소` (voir Types). Le parser connaît les mots de type, il n'y a donc pas d'ambiguïté :
`문자& 형식` est un type, `가 & 3` un ET bit à bit.

## Opérateurs en mots

| | Concept | Mot ou symbole | Notes |
| --- | --- | --- | --- |
| | et logique | `그리고` | « et » : `가 > 0 그리고 나 > 0` |
| | ou logique | `또는` | « ou », celui des maths et de la logique : `가 < 0 또는 가 > 9` |
| | non logique | `COND이/가 아니면`, `COND이/가 아닌 동안` | « si ce n'est pas COND » : la négation est une variante de la fin de condition |
| | conversion de type (`as`) | `TERME을/를 TYPE(으)로 바꿔서` | verbe intégré 바꾸다 (« changer X en Y ») dans une chaîne ; `으로` après une consonne, `로` après une voyelle **ou ㄹ** (`서울로`, « vers Séoul ») |
| | taille d'un type (`sizeof`) | `TYPE의 크기` | « la taille de TYPE » : `정수의 크기` = 8 |

```
만약 가 > 0 그리고 나 > 0이면 { … }
만약 (가 > 0)이 아니면 { … }          // si ce n'est pas (가 > 0)
가를 실수로 바꿔서 2.5를 더해서 줘요     // (실수) 가 + 2.5
```

`아니면` sert déjà de « sinon », mais il n'y a pas de conflit : « sinon » vient juste après `}`, la
négation juste après une condition.

## Modificateurs

| | Concept | Mot | Notes |
| --- | --- | --- | --- |
| | modifiable (`mut`) | — | par défaut |
| | immuable | `고정된` | « fixé » (고정되다 → `고정된`) ; placé devant le nom, comme `짧은 정수` ; variante plus longue : `바뀌지 않는` (« qui ne change pas ») |

```
개수는 0이에요                    // modifiable
고정된 합은 3과 4를 더한 값이에요  // immuable
개수에 1을 넣어요                 // affectation : « mettre 1 dans 개수 »
개수와 1을 더해서 개수에 넣어요    // 개수 = 개수 + 1
개수에 개수와 1을 더해서 넣어요    // pareil, la destination en tête
합에 5를 넣어요                   // erreur : 합 est 고정된
```

L'affectation utilise le verbe intégré 넣다 (« mettre dans ») avec `에` (« dans ») : elle se place
naturellement en fin de chaîne. `넣어요` finit la phrase, `넣어서` continue la chaîne avec la
valeur affectée.

La destination `NOM에` peut aussi ouvrir la phrase, avant la chaîne : un `NOM에` en tête est un
argument du **dernier** verbe de la chaîne. Il n'y a pas d'ambiguïté, puisque seul ce verbe peut
le recevoir et qu'aucune autre étape n'attend de `에`.

## Noms prédéfinis

| | Concept | Nom | Notes |
| --- | --- | --- | --- |
| ★ | point d'entrée (`main`) | `main` | seule exception au langage tout coréen : la libc l'appelle sous ce nom |
| ★ | afficher | — | pas de fonction intégrée : `printf` de la libc, déclaré avec `외부` |
| ★ | rendre une valeur | `주다` | `줘요` : fin de chaîne qui rend le résultat (voir Contrôle de flux) |
| | affecter | `넣다` | `NOM에 VALEUR을/를 넣어요` ; en fin de chaîne : `… 개수에 넣어요` |
| | convertir | `바꾸다` | `TERME을/를 TYPE(으)로 바꿔서` (voir Opérateurs) |
| | compter (boucle) | `세다` | `[TYPE] NOM을/를 A부터 B까지 세면서` (voir Contrôle de flux) |

Ces verbes sont réservés : une fonction de l'utilisateur ne peut pas porter leur nom.

```
외부 정수를 주는 printf(문자 주소 형식, …)

"%ld\n"과 42를 printf해요                  // 42
3과 4를 더해서 결과에 넣어요
"%ld\n"과 결과를 printf해요                // 7
```

Le résultat d'une chaîne devient le **premier** argument de l'étape suivante, alors que pour
`printf` le premier argument est le format. On ne peut donc pas finir une chaîne par `printf해요` :
il faut passer par une variable.

## Mots ajoutés

| | Concept | Mot | Notes |
| --- | --- | --- | --- |
| | | | |
