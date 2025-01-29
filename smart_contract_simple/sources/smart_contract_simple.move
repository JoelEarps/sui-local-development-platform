module smart_contract_simple::hello_world {
    use std::string;
    use sui::object::{Self, UID};
    use sui::transfer;
    use sui::tx_context::{Self, TxContext};
    use sui::event;

    /// An object that contains an arbitrary string
    public struct HelloWorldObject has key, store {
        id: UID,
        /// A string contained in the object
        text: string::String
    }

    public struct TestEvent has copy, drop  {
        user: string::String,
        purpose: string::String,
    }

    public fun mint(ctx: &mut TxContext) {
        let object = HelloWorldObject {
            id: object::new(ctx),
            text: string::utf8(b"Hello World!")
        };
        transfer::public_transfer(object, tx_context::sender(ctx));

        event::emit(TestEvent {
            user: string::utf8(b"Joel"),
            purpose: string::utf8(b"Test Event to show this works")
        })
    }

    // Test for an emitted event?
    #[test]
    fun this_is_a_test() {
        assert!(true, 1)
    }
}

