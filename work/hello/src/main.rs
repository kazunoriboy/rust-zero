fn main() {
    fn hello() {
        struct Msg {
            msg1: &'static str,
            msg2: &'static str,
        }

        fn print_msg(msg: &Msg) {
            println!("{}{}", msg.msg1, msg.msg2);
        }

        let msg = Msg{msg1: "Hello, ", msg2: "world!"};
        print_msg(&msg);
    }
    hello();
}

