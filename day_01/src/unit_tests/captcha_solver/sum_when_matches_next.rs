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
            let expected_result = Some(Ok(1));

            ctx.then("the result should be 1", move |env| {
                assert!(env.result == expected_result);
            });
        });
    }));
}
