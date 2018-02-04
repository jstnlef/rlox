use std::time::{Duration, SystemTime, UNIX_EPOCH};
use interpreter::{Interpreter, RuntimeResult};
use lox_object::{Callable, LoxObject};
use scanner::Literal;

pub struct Clock;

impl Callable for Clock {
    fn arity(&self) -> usize {
        0
    }

    fn call(&self, interpreter: &mut Interpreter, arguments: &[LoxObject]) -> RuntimeResult<LoxObject> {
        let dur: Duration = SystemTime::now().duration_since(UNIX_EPOCH).expect("time went backwards");

        let ms: f64 = dur.as_secs() as f64 * 1e3 + dur.subsec_nanos() as f64 / 1e6;

        Ok(Literal::Number(ms).to_lox_object())
    }
}
