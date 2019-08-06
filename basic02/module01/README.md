# rust의 모듈
## 모듈 파일 시스템의 규칙
* 만일 foo라는 이름의 모듈이 서브모듈을 가지고 있지 않다면, foo.rs라는 이름의 파일 내에 foo에 대한 선언을 집어넣어야 합니다.
* 만일 foo가 서브모듈을 가지고 있다면, foo/mod.rs라는 이름의 파일에 foo에 대한 선언을 집어넣어야 합니다.
* 이 규칙들은 재귀적으로 적용되므로, foo라는 이름의 모듈이 bar라는 이름의 서브모듈을 갖고 있고 `bar는 서브모듈이 없다면, 여러분의 src 디렉토리 안에는 아래와 같은 파일들이 있어야 합니다:
> ├── foo\
> │   ├── bar.rs (contains the declarations in `foo::bar`)\
> │   └── mod.rs (contains the declarations in `foo`, including `mod bar`)