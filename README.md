# Repline: The Prebaked Multiline Editor

Repline is an easy-to-use, fairly sensible multiline terminal editor meant
specifically for REPL-ifying existing Rust projects. It aims to provide a simple API
and fairly intuitive™ keybinds, which will probably work in some terminals.

Repline is a component of the Conlang project.

## Integrating Repline into your project

The easiest, most intuitive way to integrate Repline into your project is with
the `prebaked` interface. This interface provides user input to a passed-in
`FnMut` lambda as a string each time the user presses the enter key at the end
of the last line. 

```rust
use repline::prebaked::{read_and, Response};

fn main() -> Result<(), Box<dyn Error>> {
    // read_and takes three arguments: an ANSI terminal `color` string,
    // a `begin` prompt, which is shown on the first line,
    // and an `again` prompt, which is shown on subsequent lines.
    //
    // It returns a Result<(), repline::error::Error> on I/O failure.
    // `read_and` captures the Ctrl+C and Ctrl+D sequences.
    //
    // See its documentation for more info.
    read_and("", ".>", " >", |line| match line.trim() {
        // Returning Response::Continue will add a new line with the "again" prompt
        "" => Ok(Response::Continue),

        // Returning Response::Accept will add the input to history
        // and clear the input
        "accept" => Ok(Response::Accept),

        // Returning Response::Deny will just clear the input
        "deny" => Ok(Response::Deny),

        // Returning Response::Break will end the loop
        "exit" => Ok(Response::Break),

        // Returning an Err value will add a new line, (like Response::Continue,)
        // but show the formatted error value after the cursor.
        command => Err(format!("Unrecognized command: {command}"))?,
    })?;

    Ok(())
}
```

## Advanced Repline integration

More complicated projects may have more complicated needs. To service these, the
repline interface is exposed for direct consumption. This can be used if:
- You want to use a non-stdin source, such as a byte slice or file stream
  - See the [error](/examples/error.rs) example.
- You want to switch input sources after startup
  - See the [continue](/examples/continue.rs) example.
- You want to customize the prompt in reaction to user input
  - Read the docs for `repline::Repline`
