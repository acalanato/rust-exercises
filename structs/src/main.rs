fn main() {
    // first instance of struct user, only the whole can be mut
    let mut user1 = User {
	active: true,
	username: String::from("Morgana"),
	email: String::from("morgana@fofura.pt.br"),
	sign_in_count: 1,
    };
    // struct update syntax
    let user2 = User {
	active: user1.active,
	username: user1.username,
	email: String::from("morgana@lynda.pt.br"),
	sign_in_count: user1.sign_in_count,
    };
    // short version of update
    let user2 = User {// last one
	email: String::from("morgana@simplesmente.pt.br"),
	..user2
    };
    user1.email = String::from("morgana@gracinha.pt.br");
    let user3 = build_user("Merlin".to_string(), "merlin@lindo.demais".to_string());
    print!("
User:\t{0}
Email:\t{1}

User:\t{2}
Email:\t{3}
",
	   user2.username,
	   user2.email,
	   user3.username,
	   user3.email,
    );
    print!(r"
 ,_     _
 |\\_,-~/
 / _  _ |    ,--.
(  @  @ )   / ,-'
 \  _T_/-._( (
 /         `. \
|         _  \ |
 \ \ ,  /      |
  || |-_\__   /
 ((_/`(____,-'

");
    //struct tuples
    let _black = Color(0,0,0);
    let _origin = Point(0,0,0);

    let _subject = AlwaysEqual;
}
//Morgana e Merlin são meus gatos
//defining struct user
struct User {
    active: bool,
    username: String,
    email: String,
    sign_in_count: u64,
}

//using a function to abstract struct usage
//input must be in the same order
fn build_user(username: String, email: String) -> User {
    User {
	active: true,
	username,
	email,
	sign_in_count: 1,
    }
}

// struct tuple are just like named tuples
struct Color(i32,i32,i32);
struct Point(i32,i32,i32);

//Unit-like structs without fields
struct AlwaysEqual;
