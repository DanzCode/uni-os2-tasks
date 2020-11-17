
trait State {}
trait Terminal {}

trait Transition<T> /* TODO: Trait-Bounds */ {
	fn transition(self) -> T;
}

trait Terminate /* TODO: Trait-Bounds */ {
	fn terminate(self);
}

struct Start;
struct Loop;
struct End;

fn main() {
	/* TODO: Zustandstransitionen definieren
	let state = Start::new();
	// let state: End = state.transition(); // Compilezeitfehler
	let state: Loop = state.transition();
	// state.terminate(); // Compilezeitfehler
	let state: Loop = state.transition();
	// let state: Start = state.transition(); // Compilezeitfehler
	let state: End = state.transition();
	state.terminate();
	*/
}
