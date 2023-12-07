# SOS Dentes

Project of my software testing class with the purpose of creating and testing
an odontological office system.

Disclaimer: this project was made in a rush, so it throws cloned Strings and
unwraps all over the place. And, actually, it's unfinished because I didn't
make the integration tests.

## Building

```bash
cargo build # builds the entire workspace in debug mode
cargo run --bin pacient # builds and run the pacient binary
cargo run --bin receptionist # builds and run the receptionist binary
cargo run --bin dentist # builds and run the dentist binary
```

## Program's Workflow

The pacient gets tickets and waits its turn to be attended by the receptionist.

Once the pacient is attended, it provides some personal information to create
the service sheet and waits until the dentist calls them. The receptionist also
can schedule appointments and deals with the payment process at the end.

The dentist can call pacients to be attended and redirects them to the
receptionist after it.
