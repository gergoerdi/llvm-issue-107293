This repository demonstrates the following Rust compiler bug on the
AVR target: https://github.com/rust-lang/rust/issues/107293

The included Rust crate in `worduino-avr/` passes the feature flag
`score` to `worduino-engine/`. In `worduino-engine`, the only thing
changed by this feature flag is the presence or abscence of the
`score` field in `struct Player`; otherwise, this field is unused:

```
#[derive(Clone, Copy)]
struct Player {
    e: Entity,
    lives: u8,
    #[cfg(feature = "score")] score: u16,
    bullet: Option<Entity>,
}

impl Player {
    fn new() -> Player {
        Player {
            e: Self::start_pos(),
            lives: 2,
            #[cfg(feature = "score")] score: 0,
            bullet: None,
        }
    }
}
```

As we can see with the included AVR simulator, the observable
behaviour of the two programs differ: they end up sending different
data via SPI.
