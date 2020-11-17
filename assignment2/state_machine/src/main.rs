
trait State {}
trait Terminal {}

trait Transition<T> /* TODO: Trait-Bounds */ {
	fn transition(self) -> T;
}

trait Terminate /* TODO: Trait-Bounds */ {
	fn terminate(self);
}

struct Start;
impl Start {
	fn new() -> Start {
		Start
	}
}
impl State for Start {}
impl Transition<Loop> for Start {
	fn transition(self) -> Loop {
		Loop
	}
}
struct Loop;
impl State for Loop {}
impl Transition<Loop> for Loop {
	fn transition(self) -> Loop {
		self
	}
}
impl Transition<End> for Loop {
	fn transition(self) -> End {
		End
	}
}
struct End;
impl State for End {}
impl Terminal for End{}
impl Terminate for End {
	fn terminate(self) {
	}
}
fn main() {
	let state = Start::new();
	// let state: End = state.transition(); // Compilezeitfehler
	let state: Loop = state.transition();
	// state.terminate(); // Compilezeitfehler
	let state: Loop = state.transition();
	// let state: Start = state.transition(); // Compilezeitfehler
	let state: End = state.transition();
	state.terminate();
}
