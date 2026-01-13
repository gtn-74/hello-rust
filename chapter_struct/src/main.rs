#[derive(Debug)]
struct User {
    user_name: String,
    email: String,
    sign_in_count: u64,
    active: bool,
}
struct ParamsT {
    user_name: String,
    email: String,
}

fn main() {
    let user1 = User {
        email: String::from("someone@example.com"),
        user_name: String::from("someusername123"),
        active: true,
        sign_in_count: 1,
    };

    // let res = build_user_a(user1);

    let res = build_user_b(ParamsT {
        email: user1.email,
        user_name: user1.user_name,
    });
    println!("{:#?}", res);
    // fn build_userA(obj: User) -> User {
    //     let User {
    //         user_name, email, ..
    //     } = obj;
    //     User {
    //         user_name,
    //         email,
    //         sign_in_count: 12,
    //         active: false,
    //     }
    // }
    // fn build_user_a(
    //     User {
    //         user_name, email, ..
    //     }: User,
    // ) -> User {
    //     User {
    //         user_name,
    //         email,
    //         sign_in_count: 12,
    //         active: false,
    //     }
    // }

    fn build_user_b(
        ParamsT {
            user_name, email, ..
        }: ParamsT,
    ) -> User {
        User {
            user_name,
            email,
            sign_in_count: 12,
            active: false,
        }
    }

    // println!("{:#?}", user1);
    // println!("{}", user1.user_name);
}
