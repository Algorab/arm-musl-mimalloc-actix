

    use actix::prelude::*;
    use std::time::{Duration};



    pub struct Checker {}
    
    pub mod messages {

        use actix::prelude::*;

        #[derive(Clone, Message)]
        #[rtype(result = "()")]
        pub struct Check;
    }

    impl Default for Checker {
        fn default() -> Self {
            Checker {}
        }
    }

    // Implement Interfaces for get the actor
    // out from the ArbiterRegistry
    impl Supervised for Checker {}

    impl ArbiterService for Checker {
        fn service_started(&mut self, ctx: &mut Context<Self>) {
            println!("Service Checker started");
            ctx.run_interval(Duration::from_millis(2000), |_act, ctx| {
                ctx.address().do_send(messages::Check);
            });
        }
    }

    impl Actor for Checker {
        type Context = Context<Self>;
        fn started(&mut self, _ctx: &mut Self::Context) {
            println!("Checker actor started.")
        }
    }

    impl Handler<messages::Check> for Checker {
        type Result = ();
        fn handle(&mut self, _msg: messages::Check, _ctx: &mut Context<Self>) {

            println!("Got Check from Interval");



        }
    }




