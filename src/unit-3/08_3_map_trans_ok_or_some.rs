#[derive(Debug)]
struct Version {
    id: i32,
    published_by: Option<i32>
}

impl Version {
    pub fn published_by(&self) -> Option<User> {
        // map retorna Some or None segun sea el caso
        self.published_by.map(|pb| User::find(pb))
    }
}

#[derive(Debug)]
struct User {
    id: i32,
    login: String
}

impl User {
    fn find(id: i32) -> User {
        // La implementacion actual que busca al usuario en la base de datos
        // por el ID se removio por simplicidad
        User {
            id,
            login: String::from("some_username")
        }
    }
}

fn main() {
    let v1 = Version {
        id: 1,
        published_by: Some(3)
    };

    println!("vesion 1 published by user = {:?}", v1.published_by());

    let v2 = Version {
        id: 2,
        published_by: None
    };

    println!("vesion 2 published by user = {:?}", v2.published_by());
}