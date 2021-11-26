mod front_of_house{
    pub mod hosting{
        pub fn add_to_waitlist()->u8{ 2+2 }
        fn seat_at_table(){}
    }

    mod serving {
        fn take_order(){}

        fn server_order(){}

        fn take_payment(){}
    }
}

pub fn eat_at_restaurant(){
    //absolute path
    crate::front_of_house::hosting::add_to_waitlist();

    //relative path
    front_of_house::hosting::add_to_waitlist();
}


mod tests{
    use crate::front_of_house;

    #[test]
    fn it_works(){
        assert!(front_of_house::hosting::add_to_waitlist() == 4);
    }
}