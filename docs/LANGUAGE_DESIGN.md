# Language design

Words to define for the source language. Fill in the "Word" column; leave it empty if the concept
does not exist in the language. ★ = needed for the first slice (`main` returning 42, then a call
to `printf`).

Decisions already made:

- Identifiers do not accept digits (hangul, latin and `_`).
- Speech level: 해요체 in function bodies (`예요`, `반환해요`, `넣어요`…). The sentence declaration
  reads like a line of a specification and ends with the noun `반환` (see
  [Sentence declaration](#sentence-declaration)).
- End of statement: the ending in `요` ends the statement, no `;`. A `.` is accepted right after
  the `요` (`42를 반환해요.`). Line breaks count as spaces.
- Comments: `// …` to the end of the line, and `/* … */`.
- libc functions: latin name + `하다`, like foreign words in Korean (컴파일하다, 클릭하다):
  `printf해요`, `printf해서`. The lexer separates latin from hangul on its own.
  Example: `"%ld\n"과 가를 printf해요`. Digits in latin names (`log10`, `atan2`) are still to be
  decided.
- Particles: always attached (`나를`, `정수를`, `42를`, `(가 + 나)를`). See
  [Attached particles](#attached-particles) for how they are split.
- Functions = verbs, chained calls without nesting: see [Functions and conjugation](#functions-and-conjugation).
- Return: an expression with an operator goes between parentheses (`(가 + 나)를 반환해요`),
  otherwise there is no telling whether `를` applies to `나` or to `가 + 나`. A simple term
  (number, name) does not need them: `42를 반환해요`, `가를 반환해요`. After `)`, the particle is
  attached.
- Plain arithmetic is always possible: wherever a value is expected (assignment, argument, return,
  declaration, condition), an expression with operators is accepted, without going through a
  function. Between parentheses if a particle follows: `개수에 (개수 + 1)을 넣어요` (`a = a + 1`),
  `결과에 (가 * 나)를 넣어요` (`a = b * c`).
- Negative number: `-5` is a simple term (`-5를 반환해요`). The `-` stays a separate token in the
  lexer (otherwise `가-5` would give `가` then `-5`); the parser attaches `-` to a number. `-가`
  stays an expression: `(-가)를 반환해요`.

Reference example (first slice):

```
정수를 반환하는 main() {
    42를 반환해요
}

정수를 반환하는 더하다(정수 가, 정수 나) {
    (가 + 나)를 반환해요
}

정수를 반환하는 곱하다(정수 가, 정수 나) {
    (가 * 나)를 반환해요
}

정수를 반환하는 계산하다() {
    3과 4를 더해서 5를 곱해서 반환해요       // (3 + 4) * 5
}

인사하다() {
    …
}
```

## Functions and conjugation

### Principle

- A function is a **verb**, declared in its dictionary form (`더하다`, `계산하다`, `먹다`).
  The only exception to the all-Korean language: `main`, which is never called in the code.
- **`다` marks functions**: every word ending in `다` is a verb, so a function. A variable or
  parameter name cannot end in `다` (`바다`, "the sea"). This lets the compiler recognize a verb
  without context, even when it is called before its declaration. A word in `다` not followed by
  `(` gives a warning: it is probably a badly chosen name.
- Calls **do not nest**: they are composed as a chain, from left to right.
  The result of the previous step becomes the **first argument** of the next step.
- Arguments: joined by `와/과`, the last one carries `을/를`. An argument is a simple term or an
  arithmetic expression between parentheses, never a call.
- A chain ends with a verb in the `-아/어요` form: `반환해요` returns the current result; any other
  verb is a call whose result is ignored.
- Several intermediate results: name them in a variable (syntax to be defined).

```
3과 4를 더해요                 // call, result ignored
3과 4를 더해서 반환해요            // returns 3 + 4
3과 4를 더해서 5를 곱해서 반환해요  // returns (3 + 4) * 5
인사해요                       // call without argument
```

### Sentence declaration

A function can also be declared as a sentence: "takes the integer 가 and the integer 나, compares
them and returns an integer". The parameters come **before** the verb, joined by the same
particles as in the call (`와/과` between them, `을/를` on the last one), then `받아` ("taking"),
the verb in the `-고` form, and the returned type followed by the noun `반환`. Both forms are
equivalent.

```
정수를 반환하는 비교하다(정수 가, 정수 나) { … }       // C form
정수 가와 정수 나를 받아 비교하고 정수를 반환 { … }   // sentence form
두 정수 가와 나를 받아 비교하고 정수를 반환 { … }     // short form

7과 3을 비교해요                                   // the call, the same for all three
```

- The declaration thus looks like the call: `정수 가와 정수 나를 받아 비교하고` / `7과 3을 비교해요`.
- The particle follows the last syllable of the name, not of the type: `가와`, `나를` (vowel),
  `끝과`, `끝을` (consonant).
- A single parameter carries `을/를` (`정수 가를 받아 제곱하고 정수를 반환`).
- **Short form**: a native Korean number before the type announces several parameters of that
  type; the names that follow have no type: `두 정수 가와 나를`, `세 소수 가와 나와 라를`. Groups
  can be mixed: `두 정수 가와 나와 소수 라를`. Recognized numbers: `두` (2) to `열` (10); `한` (1)
  is also the `-(으)ㄴ` form of `하다`, and a single parameter needs no counter. A number is only
  one before a type: `세` and `열` stay free as names. Announcing more or fewer names than follow
  is an error, as is a name without a type outside a counted group.
- The verb is in the `-고` form (`비교하고`, `먹고`): the stem followed by `고`, without ambiguity,
  from which the compiler finds the infinitive (`비교하다`) and generates the other forms.
- `반환` alone, without `하다`, is the style of specifications (개조식); in the body, the return
  keeps the 해요체: `반환해요`.
- Without parameters, the sentence starts at the verb: `인사하고 정수를 반환 { … }`. Without
  parameters or return, the infinitive followed by the block: `인사하다 { … }`.
- With parameters but no return type, the C form is the only one for now:
  `보여주다(정수 가) { … }`. The sentence form without return (`정수 가를 받아 보여준다`) is still to
  be done.

### Generated forms

At the declaration, the compiler generates the forms of the verb and stores them in a table
"form → (function, use)". No reverse analysis: the parser looks the word up in the table.
Only the `-고` form of a sentence declaration is brought back to the infinitive, by replacing `고`
with `다`.

| Form | Use | 하다 | 먹다 |
| --- | --- | --- | --- |
| `-아/어서` | chain a step | 해서 | 먹어서 |
| `-아/어요` | end the chain | 해요 | 먹어요 |
| `-(으)ㄴ` | qualify a noun (`더한 값`) | 한 | 먹은 |
| `-고` | verb of a sentence declaration | 하고 | 먹고 |

### Overloading (to do)

Functions are verbs, and the choice of natural verbs is limited: `더하다` must be able to add
`정수` as well as `소수`. The same verb can therefore be declared **several times**, with different
parameters.

```
정수를 반환하는 더하다(정수 가, 정수 나) { (가 + 나)를 반환해요 }
소수를 반환하는 더하다(소수 가, 소수 나) { (가 + 나)를 반환해요 }

3과 4를 더해서 반환해요          // the 정수 version
1.5와 2.5를 더해서 반환해요      // the 소수 version
```

- The declarations of the same verb must differ by the **number** or the **types** of their
  parameters. The return type alone is not enough: at the call, nothing says which one is
  expected.
- The called version is chosen after the parser, during type checking, from the arguments of the
  call. In a chain, the result of the previous step counts as the first argument:
  `3과 4를 더해서 2.5를 곱해서` picks the `곱하다` whose first parameter is a `정수`.
- No version matches, or several match equally well: error.
- The table of forms gives the **verb** (`더해서` → `더하다`), not a precise function: all its
  versions share the same conjugated forms.
- Each version has its own symbol in the object file (the name of the verb completed with the
  types of the parameters), so that the linker tells them apart.
- `외부` functions cannot be overloaded: C has only one symbol per name.

### Examples

Functions used in the examples:

```
정수를 반환하는 더하다(정수 가, 정수 나) {
    (가 + 나)를 반환해요
}

정수를 반환하는 빼다(정수 가, 정수 나) {
    (가 - 나)를 반환해요
}

정수를 반환하는 제곱하다(정수 가) {      // 제곱 = square
    (가 * 가)를 반환해요
}

인사하다() {
    …
}
```

| Verb | `-아/어서` | `-아/어요` | `-(으)ㄴ` |
| --- | --- | --- | --- |
| 더하다 | 더해서 | 더해요 | 더한 |
| 빼다 | 빼서 (ㅐ + 어 → ㅐ) | 빼요 | 뺀 |
| 제곱하다 | 제곱해서 | 제곱해요 | 제곱한 |
| 인사하다 | 인사해서 | 인사해요 | 인사한 |

**Chain (`-아/어서`)**: each step gets the previous result as its first argument.

```
10과 4를 빼서 반환해요                       // 10 - 4 = 6
10과 4를 빼서 3을 더해서 반환해요            // (10 - 4) + 3 = 9
10과 4를 빼서 3을 더해서 제곱해서 반환해요   // ((10 - 4) + 3)² = 81
3과 4를 더해서 제곱해서 2를 빼서 반환해요     // (3 + 4)² - 2 = 47
```

`제곱해서` has no written argument: its only parameter is the previous result.
Particles: 10 (십) and 3 (삼) end with a consonant → `과`, `을`; 4 (사) and 2 (이) with a vowel →
`와`, `를`.

**End (`-아/어요`)**: `반환해요` returns the result; another verb calls and ignores the result.

```
인사해요                  // call without argument
3과 4를 더해요            // computes 7 and ignores it
3과 4를 더해서 반환해요       // returns 7
인사해서 3과 4를 더해요   // error: 인사하다 returns nothing, nothing to pass to 더하다
```

**Qualify (`-(으)ㄴ`)**: proposal to name an intermediate result, with the variable form
`NAME 은/는 VALUE이에요/예요` (`값` = value, final ㅂ → `이에요`).

```
합은 3과 4를 더한 값이에요           // 합 = 3 + 4
차는 10과 4를 뺀 값이에요            // 차 = 10 - 4
합과 차를 더해서 반환해요               // 7 + 6 = 13
```

### Regular verbs (first step)

Everything is computed from the last syllable of the stem (split into initial, vowel and final
from the Unicode code point, then put back together).

**`-아/어`**: `아` if the last vowel is `ㅏ` or `ㅗ`, `어` otherwise. With a final (받침), no
contraction: 받다 → 받아요, 먹다 → 먹어요. Without a final, the vowel contracts:

| Vowel of the stem | Result | Example |
| --- | --- | --- |
| ㅏ + 아 | ㅏ | 가다 → 가요 |
| ㅗ + 아 | ㅘ | 보다 → 봐요 |
| ㅓ + 어 | ㅓ | 서다 → 서요 |
| ㅜ + 어 | ㅝ | 주다 → 줘요 |
| ㅣ + 어 | ㅕ | 마시다 → 마셔요 |
| ㅐ + 어 | ㅐ | 보내다 → 보내요 |
| ㅔ + 어 | ㅔ | 세다 → 세요 |
| ㅚ + 어 | ㅙ | 되다 → 돼요 |
| ㅡ + 아/어 | ㅡ drops, harmony on the previous syllable | 쓰다 → 써요, 바쁘다 → 바빠요 |

**`-(으)ㄴ`**: without a final, `ㄴ` becomes the final (가다 → 간); with a final, add `은`
(먹다 → 먹은); final `ㄹ`: it drops and `ㄴ` replaces it (만들다 → 만든).

**하다**: a special case, included from the first step: 해서, 해요, 한.

### Irregular verbs (later)

Irregularity depends on the verb, not on its spelling (입다 → 입어요 but 돕다 → 도와요; 묻다
"bury" → 묻어요 but 묻다 "ask" → 물어요). This will need a built-in table of the common irregular
verbs, and an optional annotation on the declaration for the ambiguous cases.

| Class | Example | -어요 | -어서 | -(으)ㄴ |
| --- | --- | --- | --- | --- |
| ㅂ | 돕다 | 도와요 | 도와서 | 도운 |
| ㄷ | 듣다 | 들어요 | 들어서 | 들은 |
| ㅅ | 짓다 | 지어요 | 지어서 | 지은 |
| 르 | 부르다 | 불러요 | 불러서 | 부른 |
| 우 | 푸다 | 퍼요 | 퍼서 | 푼 |

Meanwhile, a verb whose final is `ㅂ`, `ㄷ`, `ㅅ` or whose stem ends in `르` may be irregular: the
compiler treats it as regular.

## Attached particles

Particles are written attached, as in Korean. The lexer does not split: it produces a whole
**hangul word** (`나를`), and the **parser** separates it, because the declared names must be
known (same idea as the C "lexer hack" for `typedef`).

- **After a keyword** (`정수를`): the keyword is known, the rest is the particle.
- **After a number** (`42를`) or **`)`** (`(가 + 나)를`): the lexer stops there naturally.
- **Declaration** (`합은 3과 4를 더한 값이에요`): the name does not exist yet; the expected
  particle is removed from the end of the word (`은/는` here), only once. For a loop that declares
  its variable (`정수 칸을 0부터 10까지 세면서`), the type announces the declaration and `을/를` is
  removed.
- **Use** (`나를`): look for the longest declared name at the start of the word, such that the
  rest is a valid particle. `나를` → `나` + `를`.
- **Conflict**: declaring a name equal to another name + a particle in the same scope is an error
  (`사` and `사과`: `사과` would be `사` + `과`). A word that is exactly a declared name has no
  particle.

After a string literal (`"%ld\n"과`), `와` and `과` are both accepted: the 받침 cannot be deduced
from the text.

Recognized particles: `을/를`, `이/가`, `은/는`, `와/과`, `에`, `(으)로`, `의`, `부터`, `까지`, and
the forms of the copula `이면/면`, `이에요/예요`, `인`. No `보다` ("than"): it ends in `다`, the mark
of functions, and comparisons are written `<` and `>`.

**At most one particle per word, always at the end.** Unlike Korean, particles do not stack:
`결과에는` (에 + 는) or `여기까지만` are errors. The forms of the copula (`이면`, `이에요`, `인`) count
as a single particle. The split therefore compares **the whole rest** of the word with the list:
`값이에요` → `값` + `이에요`, never `값` + `이` + `에요`.

Bonus: once the particle is separated, the compiler knows the word before it and can check the
choice against the 받침 (`나을` → "받침이 없으니 '를'을 쓰세요").

## Declarations

| | Concept | Word | Notes |
| --- | --- | --- | --- |
| ★ | function | `TYPE을/를 반환하는 VERB다(TYPE name, …) { … }` | "VERB, which gives a TYPE"; without return: `VERB다(…) { … }` |
| | variable | `NAME은/는 VALUE이에요/예요` or `NAME은/는 TYPE이에요/예요` | with a value: deduced type (`0` → `정수`, `0.5` → `소수`); with a type: default value (zero of the type); `이에요` after a consonant, `예요` after a vowel; mutable by default |
| | constant | `고정된 NAME은/는 VALUE이에요/예요` | see Modifiers |
| ★ | external function (libc) | `외부 TYPE을/를 반환하는 name(TYPE name, …)` | latin name, no body; `…` for variadics: `외부 정수를 반환하는 printf(문자 주소 형식, …)` |
| | structure | 구조 | |
| | export / public | | |

```
개수는 0이에요              // 정수, holds 0
평균은 0.5예요              // 소수, holds 0.5
개수는 정수예요             // 정수, default value 0
이름은 문자 주소예요        // 문자 주소 (char*), holds 빈 주소
이름은 문자&예요            // the same, with the & abbreviation
점수는 정수 10개예요        // array of 10 정수, all 0
```

The parser looks at what comes before `이에요/예요`: a type keyword gives a declaration without a
value, anything else a value. Default values: `0`, `0.0`, `거짓`, `빈 주소`.
`고정된` with a type alone (`고정된 개수는 정수예요`) is an error: the constant would be zero
forever.

## Types

| | Concept | Word | Notes |
| --- | --- | --- | --- |
| ★ | integer (64 bits) | `정수` | `정수를` |
| | 32-bit integer | `짧은 정수` | 짧다 ("to be short") → `짧은`; for C interop (`int`) |
| | 16-bit integer | | rarely useful, to add if needed |
| | 8-bit integer | `바이트` | unsigned; bytes, C strings; `바이트를` |
| | unsigned integer | `부호 없는 정수` | 부호 = sign, 없다 → `없는`; the standard term in Korean computing |
| | float (64 bits) | `소수` | "decimal number" (小數); homonym of 소수 "prime number" (素數); `소수를` |
| | float (32 bits) | `짧은 소수` | same modifier as `짧은 정수` |
| | boolean | `논리` | "logic"; `논리를` |
| | character | `문자` | `문자를` |
| | pointer | `TYPE 주소` or `TYPE&` | "address": `정수 주소 가` = `정수& 가` = 가, address of an integer; `&` abbreviates `주소`, attached or not (`정수&`, `정수 &`), repeatable (`정수&&` = `정수 주소 주소`) |
| | array | `TYPE N개` | counter 개: `정수 10개 가` = 가, 10 integers; concept: 배열 |

## Values

| | Concept | Word | Notes |
| --- | --- | --- | --- |
| | true | `참` | final ㅁ → `참을`, `참이에요` |
| | false | `거짓` | "lie"; final ㅅ → `거짓을`, `거짓이에요` |
| | null pointer | `빈 주소` | "empty address" (비다 → `빈`), modeled on the type `TYPE 주소`; `빈 주소를` |

## Control flow

| | Concept | Word | Notes |
| --- | --- | --- | --- |
| ★ | return | `(EXPR)을/를 반환해요`, or `TERM을/를 반환해요` | 반환하다 = to return, the standard term in computing; the same root is used in declarations (`반환하는`, `반환`); 돌아오다 is intransitive ("to come back") |
| | if (before the condition) | `만약` | "if, suppose that"; opens the condition, which helps the parser |
| | if (after the condition) | `이면` / `면` | "if it is"; `이면` after a consonant, `면` after a vowel (both accepted at first) |
| | else | `아니면` | "if that is not the case" |
| | else if | `아니면 만약 …이면` | `아니면` and `만약` combined, like `else if` |
| | while | `COND인 동안` | "while it is"; 동안 = duration, during |
| | for (`for` loop) | `NAME을/를 A부터 B까지 세면서` or `TYPE NAME을/를 A부터 B까지 세면서` | "counting NAME from A to B"; 부터 = from, 까지 = up to, **B excluded** (`0부터 10까지` = 10 rounds, from 0 to 9), like `0..10` in Rust; 세다 = to count, -면서 = while doing; without a type, NAME already exists and the loop changes it; with a type, the loop declares NAME, which only exists in the loop (same rule as parameters: `TYPE NAME` declares) |
| | leave the loop (`break`) | `그만해요` | "stop" (그만하다) |
| | continue (`continue`) | `넘어가요` | "move on to the next" (넘어가다) |

```
만약 가 > 나면 {
    가를 반환해요
} 아니면 만약 가 == 나면 {
    0을 반환해요
} 아니면 {
    나를 반환해요
}

가 < 10인 동안 {
    …
}

가를 1부터 10까지 세면서 {
    만약 가 == 5면 { 넘어가요 }
    만약 가 == 8이면 { 그만해요 }
    …
}

정수 칸을 0부터 10까지 세면서 {     // declares 칸, like for (int 칸 = 0; …)
    …
}
```

`만약` has no equivalent for `인 동안`: the parser first reads an expression, then looks at the
next word. A particle (`를`, `과`…) announces a call chain, `인 동안` a loop.

## Symbol operators (taken from C)

Every arithmetic operator of C that has no Korean word keeps its symbol, with the precedence and
associativity of C. Only `&&`, `||` and `!` are replaced by words (see the next section).
Assignment (`=`, `+=`…) and `++` / `--` do not exist: use `넣다`.

| Precedence (high → low) | Operators | Associativity |
| --- | --- | --- |
| 1 | `-` `~` (unary) | right |
| 2 | `*` `/` `%` | left |
| 3 | `+` `-` | left |
| 4 | `<<` `>>` | left |
| 5 | `<` `<=` `>` `>=` | left |
| 6 | `==` `!=` | left |
| 7 | `&` | left |
| 8 | `^` | left |
| 9 | `\|` | left |
| 10 | `그리고` (`&&`) | left |
| 11 | `또는` (`\|\|`) | left |

`>>` on a signed integer is arithmetic (`sar`), on a `부호 없는` one logical (`shr`).

After a type word (`정수`, `문자`…), `&` is not the bitwise AND but the abbreviation of `주소` (see
Types). The parser knows the type words, so there is no ambiguity: `문자& 형식` is a type,
`가 & 3` a bitwise AND.

## Word operators

| | Concept | Word or symbol | Notes |
| --- | --- | --- | --- |
| | logical and | `그리고` | "and": `가 > 0 그리고 나 > 0` |
| | logical or | `또는` | "or", the one of maths and logic: `가 < 0 또는 가 > 9` |
| | logical not | `COND이/가 아니면`, `COND이/가 아닌 동안` | "if it is not COND": the negation is a variant of the end of the condition |
| | type conversion (`as`) | `TERM을/를 TYPE(으)로 바꿔서` | built-in verb 바꾸다 ("change X into Y") in a chain; `으로` after a consonant, `로` after a vowel **or ㄹ** (`서울로`, "to Seoul") |
| | size of a type (`sizeof`) | `TYPE의 크기` | "the size of TYPE": `정수의 크기` = 8 |

```
만약 가 > 0 그리고 나 > 0이면 { … }
만약 (가 > 0)이 아니면 { … }          // if it is not (가 > 0)
가를 소수로 바꿔서 2.5를 더해서 반환해요     // (소수) 가 + 2.5
```

`아니면` already means "else", but there is no conflict: "else" comes right after `}`, the
negation right after a condition.

## Modifiers

| | Concept | Word | Notes |
| --- | --- | --- | --- |
| | mutable (`mut`) | — | by default |
| | immutable | `고정된` | "fixed" (고정되다 → `고정된`); placed before the name, like `짧은 정수`; longer variant: `바뀌지 않는` ("that does not change") |

```
개수는 0이에요                    // mutable
고정된 합은 3과 4를 더한 값이에요  // immutable
개수에 1을 넣어요                 // assignment: "put 1 into 개수"
개수와 1을 더해서 개수에 넣어요    // 개수 = 개수 + 1
개수에 개수와 1을 더해서 넣어요    // the same, with the destination first
합에 5를 넣어요                   // error: 합 is 고정된
```

Assignment uses the built-in verb 넣다 ("put into") with `에` ("into"): it naturally goes at the
end of a chain. `넣어요` ends the sentence, `넣어서` continues the chain with the assigned value.

The destination `NAME에` can also open the sentence, before the chain: a leading `NAME에` is an
argument of the **last** verb of the chain. There is no ambiguity, since only that verb can take
it and no other step expects an `에`.

## Predefined names

| | Concept | Name | Notes |
| --- | --- | --- | --- |
| ★ | entry point (`main`) | `main` | the only exception to the all-Korean language: the libc calls it by this name |
| ★ | print | — | no built-in function: `printf` of the libc, declared with `외부` |
| ★ | return a value | `반환하다` | `반환해요`: end of a chain that returns the result (see Control flow); `반환하는` and `반환` in declarations |
| | assign | `넣다` | `NAME에 VALUE을/를 넣어요`; at the end of a chain: `… 개수에 넣어요` |
| | convert | `바꾸다` | `TERM을/를 TYPE(으)로 바꿔서` (see Word operators) |
| | count (loop) | `세다` | `[TYPE] NAME을/를 A부터 B까지 세면서` (see Control flow) |

These verbs are reserved: a user function cannot take their name.

```
외부 정수를 반환하는 printf(문자 주소 형식, …)

"%ld\n"과 42를 printf해요                  // 42
3과 4를 더해서 결과에 넣어요
"%ld\n"과 결과를 printf해요                // 7
```

The result of a chain becomes the **first** argument of the next step, while for `printf` the
first argument is the format. A chain therefore cannot end with `printf해요`: go through a
variable.

## Added words

| | Concept | Word | Notes |
| --- | --- | --- | --- |
| | | | |
