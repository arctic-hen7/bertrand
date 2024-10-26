# Bertrand

Building prototypes for backend systems isn't always easy, so Bertrand lets you define a simple HTML file like this one:

```html
<!DOCTYPE html>
<html lang="en">
<head>
    <meta charset="UTF-8">
    <meta name="viewport" content="width=device-width, initial-scale=1.0">
    <title>Bertrand Test</title>
</head>
<body>
    <div class="bertrand bertrand-init bstate:stateA">
        <p>This is state A!</p>
    </div>
    <div class="bertrand bstate:stateB">
        <p>This is state B!</p>
    </div>
    <div class="bertrand bstate:stateC">
        <p>This is state C!</p>
    </div>
</body>
</html>
```

We've got a few `div`s with the classes `bertrand` and `bstate:<some-state>`. When we run `bertrand example.html`, we'll have a local file server started (you can customise where with `--host` and `-p`/`--port`) which will, when the index is loaded in a browser, initially display *This is state A!*, because that was `bertrand-init`. However, we can send messages to it with a command like the following:

```
curl -X POST -d "stateB" http://localhost:8080/api/send
```

And that will be relayed by the server to all connected clients, who will switch to displaying the `bstate:stateB` `div`. That's pretty much all there is to it!

This lets you easily create state-based demos of backend systems by linking up your backend to Bertrand, and then displaying a browser-based representation of what's going on. In my personal experience, this resonates much more than a CLI demo!

*Note: you can provide either a single HTML file or a directory containing an `index.html` to Bertrand, and it will serve everything in there.*

## Installation

You'll need a [Rust toolchain](https://rustup.rs) to install Bertrand, and then you can run:

```
cargo install bertrand
```

## License

See [`LICENSE`](LICENSE).
