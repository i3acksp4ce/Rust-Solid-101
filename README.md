# SOLID Principles ใน Rust 🦀

## 1. Single Responsibility Principle (SRP) - หลักการรับผิดชอบเดี่ยว

หลักการที่ว่าโค้ดหรือคลาสควรมีหน้าที่รับผิดชอบเพียงอย่างเดียวเท่านั้น

❌ แย่:

```rust
struct UserManager {
    users: Vec<User>,

    fn add_user(&mut self, user: User) {
        self.users.push(user);
    }

    fn save_to_database(&self) {
        // บันทึกลงฐานข้อมูล
    }

    fn generate_report(&self) -> String {
        // สร้างรายงาน
    }
}
```

✅ ดี:

```rust
struct UserRepository {
    users: Vec<User>,

    fn add_user(&mut self, user: User) {
        self.users.push(user);
    }
}

struct UserPersistence {
    fn save_users(&self, users: &[User]) {
        // บันทึกลงฐานข้อมูล
    }
}

struct ReportGenerator {
    fn generate_user_report(users: &[User]) -> String {
        // สร้างรายงาน
    }
}
```

## 2. Open/Closed Principle (OCP) - หลักการเปิดปิด

โค้ดควรเปิดให้ขยายเพิ่มเติมได้ แต่ปิดสำหรับการแก้ไข

❌ แย่:

```rust
enum Shape {
    Circle(f64),
    Square(f64),
}

fn calculate_area(shape: &Shape) -> f64 {
    match shape {
        Shape::Circle(r) => std::f64::consts::PI * r * r,
        Shape::Square(s) => s * s,
    }
}
```

✅ ดี:

```rust
trait Shape {
    fn area(&self) -> f64;
}

struct Circle {
    radius: f64,
}

impl Shape for Circle {
    fn area(&self) -> f64 {
        std::f64::consts::PI * self.radius * self.radius
    }
}

struct Square {
    side: f64,
}

impl Shape for Square {
    fn area(&self) -> f64 {
        self.side * self.side
    }
}
```

## 3. Liskov Substitution Principle (LSP) - หลักการแทนที่ของลิสคอฟ

ออบเจ็กต์ของคลาสลูกต้องสามารถแทนที่ออบเจ็กต์ของคลาสแม่ได้โดยไม่ทำให้โปรแกรมผิดพลาด

❌ แย่:

```rust
trait Bird {
    fn fly(&self);
}

struct Penguin;
impl Bird for Penguin {
    fn fly(&self) {
        panic!("เพนกวินบินไม่ได้!"); // ผิดหลัก LSP
    }
}
```

✅ ดี:

```rust
trait Animal {
    fn move_around(&self);
}

trait FlyingBird: Animal {
    fn fly(&self);
}

struct Sparrow;
impl Animal for Sparrow {
    fn move_around(&self) {
        // เคลื่อนที่ทั่วไป
    }
}
impl FlyingBird for Sparrow {
    fn fly(&self) {
        // บินได้
    }
}

struct Penguin;
impl Animal for Penguin {
    fn move_around(&self) {
        // เดินและว่ายน้ำ
    }
}
```

## 4. Interface Segregation Principle (ISP) - หลักการแยกอินเตอร์เฟซ

ไม่ควรบังคับให้คลาสต้อง implement methods ที่ไม่จำเป็นต้องใช้

❌ แย่:

```rust
trait Worker {
    fn work(&self);
    fn eat(&self);
    fn sleep(&self);
    fn code(&self);
}

struct Human;
impl Worker for Human {
    // ต้อง implement ทุก method
}

struct Robot;
impl Worker for Robot {
    fn sleep(&self) {
        // หุ่นยนต์ไม่จำเป็นต้องนอน แต่ถูกบังคับให้ implement
    }
    // ...
}
```

✅ ดี:

```rust
trait Workable {
    fn work(&self);
}

trait Sleepable {
    fn sleep(&self);
}

trait Eatable {
    fn eat(&self);
}

struct Human;
impl Workable for Human {}
impl Sleepable for Human {}
impl Eatable for Human {}

struct Robot;
impl Workable for Robot {}
```

## 5. Dependency Inversion Principle (DIP) - หลักการพึ่งพาแบบผกผัน

โมดูลที่มีระดับสูงไม่ควรพึ่งพาโมดูลที่มีระดับต่ำ ทั้งคู่ควรพึ่งพา abstractions

❌ แย่:

```rust
struct MySQLDatabase {
    fn save(&self, data: &str) {
        // บันทึกข้อมูลใน MySQL
    }
}

struct UserService {
    database: MySQLDatabase,

    fn save_user(&self, user: &User) {
        self.database.save(&user.to_string());
    }
}
```

✅ ดี:

```rust
trait Database {
    fn save(&self, data: &str);
}

struct MySQLDatabase;
impl Database for MySQLDatabase {
    fn save(&self, data: &str) {
        // บันทึกข้อมูลใน MySQL
    }
}

struct PostgresDatabase;
impl Database for PostgresDatabase {
    fn save(&self, data: &str) {
        // บันทึกข้อมูลใน Postgres
    }
}

struct UserService<T: Database> {
    database: T,

    fn save_user(&self, user: &User) {
        self.database.save(&user.to_string());
    }
}
```
