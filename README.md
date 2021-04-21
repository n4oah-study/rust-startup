# rust bash명령어

컴파일

```sh
rustc 파일이름
```

실행

```sh
./파일이름
```

## 개발도구

- cargo: 내장된 의존성 관리 및 빌드 도구인 카고는 러스트 생태계를 통틀어 편리하면서도 일관된 컴파일 및 의존성 관리 기능을 제공함
- Rustfmt: 이 도구는 모든 개발자가 일관된 코딩 스타일을 유지할 수 있도록 돕는다.
- 러스트 언어 서버(The Rust Language Server): 통합개발환경을 위한 코드 완성 및 인라인 에러 메시지 등의 기능을 제공한다.

## cargo 프로젝트 생성

cargo 프로젝트 생성

```sh
cargo new guessing_game --bin // 바이너리
cargo new guessing_game_lib --lib // 라이브러리
```

## cargo build

cargo 프로젝트 빌드

```sh
cargo build
target/debug/ 생성
```

실행

```sh
./파일이름
```

cargo 프로젝트 빌드 후 바로 실행

```sh
cargo run
```

프로젝트가 정상적으로 build되는지 확인 (바이너리 파일을 생성하거나, 실행하지는 않음)

```sh
cargo check
```

프로젝트 release
target/release/ 생성

```sh
cargo build --release
```

## Rust 네이밍 컨밴션

- <https://doc.rust-lang.org/1.0.0/style/style/naming/README.html>

## 참고

- 라이브러리로 생성된 패키지는 cargo run을 할 수 없음(main.rs가 없기 때문)
  - cargo build 로 라이브러리를 만들어주어야 함.
- rust는 모든 라이브러리가 global로 설치됨 (~/.cargo/registry)
- RUST의 기본 인코딩 방식은 UTF-8임

Rust 버전: rustc 1.36.0 (a53f9df32 2019-07-03)

### 프로젝트별 readme.md에 있는 NOTE부분은 개인적인 생각임(뇌피셜)

## 참고문서

- 튜토리얼 위주
  - <https://rinthel.github.io/rust-lang-book-ko/ch00-00-introduction.html>
  - <https://doc.rust-lang.org/rust-by-example/primitives/tuples.html>
  - <https://riptutorial.com/ko/rust/topic/362/rust-%EC%8B%9C%EC%9E%91%ED%95%98%EA%B8%B0>
  - <https://wiki.archlinux.org/index.php/Rust>
- API
