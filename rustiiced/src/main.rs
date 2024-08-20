struct Counter {
    value: i64,
}

enum Message {
    Increment,
    Decrement,
}


impl Counter {
    fn update(&mut self, message: Message) {
	match message {
	    Message::Increment => {
		self.value +=1;
	    }
	    Message::Decrement => {
		self.value -= 1;
	    }
	}
    }
}


fn main() {

    //    println!("{}", counter.value);
    println!("Sucess!")
}

#[test]
fn counts_properly() {
    let mut counter = Counter { value: 0};
    counter.update(Message::Increment);
    counter.update(Message::Increment);
    counter.update(Message::Decrement);

    assert_eq!(counter.value, 1);
}
