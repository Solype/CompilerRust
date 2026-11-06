# Krc: 한국어 프로그래밍 언어와 컴파일러

[English](README.md) | **한국어** | [웹사이트](https://solype.github.io/Krc/ko)

[![CI](https://github.com/Solype/Krc/actions/workflows/ci.yml/badge.svg)](https://github.com/Solype/Krc/actions/workflows/ci.yml)
[![License: MIT](https://img.shields.io/badge/license-MIT-blue.svg)](LICENSE.md)

**Krc**는 한국어로 코드를 쓰는 프로그래밍 언어입니다. 컴파일러는 Rust로 작성했으며, x86-64 리눅스용
ELF 목적 파일을 만듭니다. 중앙대학교(CAU) 컴파일러 수업에서 시작한 프로젝트이고, 수업이 끝난 지금은
재미로 계속 개발하고 있습니다.

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

위 코드는 [`sample/10_gcd.kr`](sample/10_gcd.kr)에 있는 최대공약수 함수입니다.

## 언어의 특징

- **함수는 동사입니다.** 호출을 중첩하지 않고 문장처럼 이어서 씁니다:
  `3과 4를 더해서 제곱해서 반환해요`
- **인자에는 조사가 붙습니다.** 예: `나머지에 (가 % 나)를 넣어요`
- **문장은 해요체로 끝납니다.** 예: `42를 반환해요.`
- **libc 함수는 라틴 문자 이름에 `하다`를 붙여 호출합니다.** 예: `"%ld\n"과 가를 printf해요.`

자세한 설계는 [`docs/LANGUAGE_DESIGN.md`](docs/LANGUAGE_DESIGN.md)(영어)에 있습니다.

## 설치

x86-64 리눅스와, 링크를 위한 `gcc`가 필요합니다.

[릴리스 페이지](https://github.com/Solype/Krc/releases/latest)에서 바이너리를 받을 수 있습니다:

```bash
curl -LO https://github.com/Solype/Krc/releases/download/v0.1.0/krc-v0.1.0-x86_64-linux.tar.gz
tar xzf krc-v0.1.0-x86_64-linux.tar.gz
cd krc-v0.1.0-x86_64-linux
```

소스에서 직접 빌드할 수도 있습니다:

```bash
cargo build --release
```

이 경우 컴파일러는 `./target/release/compiler`에 만들어집니다.

## 사용법

`hello.kr` 파일을 만듭니다:

```
외부 정수를 반환하는 printf(문자 주소 형식, …)

정수를 반환하는 main() {
    "안녕하세요!\n"을 printf해요.
    0을 반환해요.
}
```

컴파일하고 실행합니다:

```bash
./compiler -x hello.kr
./a.out
```

- `file.kr`마다 같은 위치에 `file.o`가 만들어지고, AST와 IR이 출력됩니다.
- `-x`는 만들어진 `.o` 파일을 모두 `gcc`로 링크해 `a.out`을 만듭니다.
- `--tree`는 AST 앞에 파스 트리도 출력합니다.

예제 프로그램은 [`sample/`](sample)에 있습니다.

## 현재 상태

⚠️ 아직 개발 중인 프로젝트입니다.

어휘 분석기, LR(1) 파서, 타입이 있는 IR, x86-64 인코더, ELF 작성기를 모두 이 프로젝트를 위해 직접
구현했습니다. `sample/`의 모든 프로그램이 실행됩니다. 남은 작업은
[GitHub 이슈](https://github.com/Solype/Krc/issues)에서 관리합니다.

## 기여와 라이선스

기여 방법은 [`CONTRIBUTING.md`](CONTRIBUTING.md)(영어)를 참고해 주세요. 이 프로젝트의 일부는 AI
도구를 사용해 작성했으며, 자세한 내용은 [`USE_OF_AI.md`](USE_OF_AI.md)(영어)에 정리되어 있습니다.
라이선스는 MIT이며 [`LICENSE.md`](LICENSE.md)를 참고해 주세요.
