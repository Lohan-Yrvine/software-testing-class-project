# SOS Dentes

Project of my software testing class with the purpose of creating and testing
an odontological office system.

## Building

```bash
cargo build # builds the entire workspace in debug mode
cargo run --bin pacient # builds and run the pacient binary
cargo run --bin receptionist # builds and run the receptionist binary
cargo run --bin dentist # builds and run the dentist binary
```

To build/run in release mode, just add `--release` in the commands before.

## Program's Workflow

The pacient gets tickets and waits its turn to be attended by the receptionist.

Once the pacient is attended, it provides some personal information to create
the service sheet and waits until the dentist calls them. The receptionist also
can schedule appointments and deals with the payment process at the end.

The dentist can call pacients to be attended and redirects them to the
receptionist after it.

