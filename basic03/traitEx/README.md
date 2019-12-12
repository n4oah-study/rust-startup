# Rust의 트레잇(trait)
## 정의
* 트레잇은 여러 타입들이 공통적으로 갖는 동작에 대하여 추상화해줌
## 고아규칙
* trait을 구현하려면 trait혹은 구현체(struct) 둘 중 하나라도 구현하는 소스내에 존재해야 함
* 우리의 소스에서는 Vec(Struct)에 대한 Display(trait)를 구현할 수는 없다.
## 노트
* 트레잇은 interface와 비슷하지만 다른점이 있음
* 트레잇은 내가 정의한 struct(class)외에도 이미 있는 구조체에도 새로운 메소드를 정의할 수 있다. 그러나 interface는 일반적으로(java, C# ...) 내가 생성한 클래스에서만 implements를 할 수 있다.
