---
title: A Korean programming language and its compiler
lang: en
---

[한국어](ko.md)

**Krc** is a programming language written in Korean (한국어 프로그래밍 언어), with a compiler in
Rust that produces x86-64 Linux ELF object files. Functions are verbs, and arguments carry their
particles. It started in the Compiler class of Chung-Ang University and now continues for fun.

```
두 정수 가와 나를 받아 나누고 정수를 반환 {
    나머지는 정수예요.
    (나 == 0)이 아닌 동안 {
        나머지에 (가 % 나)를 넣어요.
        가에 나를 넣어요.
        나에 나머지를 넣어요.
    }
    가를 반환해요.
}
```

This is the greatest common divisor, from
[`sample/10_gcd.kr`](https://github.com/Solype/Krc/blob/main/sample/10_gcd.kr).

## Try it

You need Linux on x86-64, and `gcc` for the link step.

```bash
curl -LO https://github.com/Solype/Krc/releases/download/v0.1.0/krc-v0.1.0-x86_64-linux.tar.gz
tar xzf krc-v0.1.0-x86_64-linux.tar.gz
cd krc-v0.1.0-x86_64-linux
```

Write this program in `hello.kr`:

```
외부 정수를 반환하는 printf(문자 주소 형식, …)

정수를 반환하는 main() {
    "안녕하세요!\n"을 printf해요.
    0을 반환해요.
}
```

Then compile and run it:

```bash
./compiler -x hello.kr
./a.out
```

## The language

- **Functions are verbs.** Calls chain like a sentence, without nesting:
  `3과 4를 더해서 제곱해서 반환해요`.
- **Arguments carry their particles**, as in `나머지에 (가 % 나)를 넣어요`.
- **Statements end in the polite 해요 style**, as in `42를 반환해요.`.
- **libc functions are called with `하다`**, like foreign words in Korean:
  `"%ld\n"과 가를 printf해요.`.

The full design is in [Language design](LANGUAGE_DESIGN.md).

## The compiler

The lexer, the LR(1) parser, the typed IR, the x86-64 encoder and the ELF writer were all written
for this project. Krc is a work in progress: what is left is tracked in the
[GitHub issues](https://github.com/Solype/Krc/issues).

- [Source code](https://github.com/Solype/Krc)
- [Releases](https://github.com/Solype/Krc/releases)
- [Encoder test report](TEST_REPORT.md)
- [Use of AI in this project](https://github.com/Solype/Krc/blob/main/USE_OF_AI.md)
