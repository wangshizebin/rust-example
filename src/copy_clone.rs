pub fn op() {
    // Because it implements the Copy trait,
    // it must implement the Clone trait
    // and all the fields must implement the Clone trait
    #[derive(Debug, Clone, Copy)]
    struct UserNext {
        id: i64,
    }
    let user = UserNext { id: 1 };
    let user1 = user;
    println!("{:?} {:?}", user.id, user1.id);

    // Because it implements the Clone trait,
    // the clone() method is automatically provided.
    // but it cannot implement the Copy trait,
    // because the String type cannot implement the Copy trait
    #[derive(Debug, Clone)]
    struct User {
        name: String,
    }

    let user = User {
        name: "wang".to_string(),
    };
    let user1 = user.clone();
    println!("{:?} {:?}", user.name, user1.name);

    // error： It doesn't implement the Copy trait
    // because the User struct contains a String field,
    // and String does not implement Copy.
    // #[derive(Debug, Clone, Copy)]
    // struct User {
    //     name: String,
    // }
    //

    // The Copy trait is implemented manually
    struct UserByHand {
        name: String,
    }
    impl Clone for UserByHand {
        fn clone(&self) -> Self {
            Self {
                name: self.name.clone(),
            }
        }
    }

    // The Debuf trait is implemented manually
    impl std::fmt::Debug for UserByHand {
        fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
            f.debug_struct("UserByHand")
                .field("name", &self.name)
                .finish()
        }
    }
    let user = UserByHand {
        name: "wang".to_string(),
    };
    let user1 = user.clone();
    println!("{:?} {:?}", user.name, user1);
}
