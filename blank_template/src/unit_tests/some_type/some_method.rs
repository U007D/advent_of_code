use super::*;

#[derive(Clone, Debug)]
struct Env {}

impl Env {
    fn new() -> Self {
        Self {}
    }
}

#[test]
fn tests() {
    run(&given("", Env::new(), |ctx| {
        ctx.before(|env| {});

        ctx.when("", |ctx| {
            ctx.before(|env| {});

            ctx.then("", move |env| {
            });
        });
    }));
}

