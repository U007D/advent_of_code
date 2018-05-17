use super::*;
use captcha_solver::CaptchaSolver;
use Result;

#[derive(Clone, Default, Debug)]
struct Env {
    solver: CaptchaSolver,
    result: Option<Result<usize>>,
}

impl Env {
    fn new() -> Self {
        Self {
            solver: CaptchaSolver::new(),
            result: None,
        }
    }
}
#[test]
fn tests() {
    run(&given("an initialized CaptchaSolver", Env::new(), |ctx| {
        ctx.when("fed nothing", |ctx| {
            ctx.before(|env| {
                env.result = Some(env.solver.sum_when_matches_next(""));
            });
            let expected_result = Some(Ok(0));

            ctx.then("the result should be 0", move |env| {
                assert!(env.result == expected_result);
            });
        });

        ctx.when("fed a single digit: 1", |ctx| {
            ctx.before(|env| {
                env.result = Some(env.solver.sum_when_matches_next("1"));
            });
            let expected_result = Some(Ok(0));

            ctx.then("the result should be 0", move |env| {
                assert!(env.result == expected_result);
            });
        });

        ctx.when("fed a non-repeating sequence: 1212345", |ctx| {
            ctx.before(|env| {
                env.result = Some(env.solver.sum_when_matches_next("1212345"));
            });
            let expected_result = Some(Ok(0));

            ctx.then("the result should be 0", move |env| {
                assert!(env.result == expected_result);
            });
        });

        ctx.when("fed a repeating sequence: 11", |ctx| {
            ctx.before(|env| {
                env.result = Some(env.solver.sum_when_matches_next("11"));
            });
            let expected_result = Some(Ok(2));

            ctx.then("the result should be 2", move |env| {
                assert!(env.result == expected_result);
            });
        });

        ctx.when("fed the problem sequence 1122", |ctx| {
            ctx.before(|env| {
                env.result = Some(env.solver.sum_when_matches_next("1122"));
            });
            let expected_result = Some(Ok(3));

            ctx.then("the result should be 3", move |env| {
                assert!(env.result == expected_result);
            });
        });

        ctx.when("fed the problem sequence 1234", |ctx| {
            ctx.before(|env| {
                env.result = Some(env.solver.sum_when_matches_next("1234"));
            });
            let expected_result = Some(Ok(0));

            ctx.then("the result should be 0", move |env| {
                assert!(env.result == expected_result);
            });
        });
    }));
}
