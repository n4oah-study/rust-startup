# Rust의 트레잇(trait)
## 정의
* 트레잇은 여러 타입들이 공통적으로 갖는 동작에 대하여 추상화해줌
## 고아규칙
* trait을 구현하려면 trait혹은 구현체(struct) 둘 중 하나라도 구현하는 소스내에 존재해야 함
* 우리의 소스에서는 Vec(Struct)에 대한 Display(trait)를 구현할 수는 없다.
## 노트
* 트레잇은 interface와 비슷하지만 다른점이 있음