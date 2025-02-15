# demo01
Rust WebAssembly Demo
=======
demo01 🎨
========
Programming mini game for Demo in Rust & WebAssembly

[![screenshot](screen.png)](https://myurioka.github.io/demo01/)

[Play in browser](https://myurioka.github.io/demo01)

### How to play (Control)

  * Click Circle: Get number and start

### Requirement
  * Rust, Cargo
  * WASM

### How to Build & Run

  ```sh
  $ cd demo01
  $ pnpm build-wasm
  $ pnpm dev --open
  ```
  Browse http://localhost:5173

### Sequence Diagram

```mermaid
sequenceDiagram
    autonumber
    participant B as Browser
    participant H as heap
    participant R as Rust

    R->>H: static mut Box<Game>
    note over H: Game
    R->>H: Glosure::wrap(Box::new(|_time:f64|()))
    note over H: requestAnimationFrame(First)
    B->>H: callback
    R->>H: Glosure::wrap(Box::new(|_time:f64|()))
    loop callback GAME.on_animation_frame
    B->>H: callback
    note over H: requestAnimationFrame
    H->>H: Game.update()
    H->>H: Game.draw()
    end
    R->>H: Closure::wrap(Box::new(|MouseEvnet|)())
    note over H: add_event_listner_with_callback("mousedown")
    alt callback GAME.on_click
    B->>H: callback
    H->>H: Game.on_click
    H->>H: forget()
    end
```
<br>
<ol>
<li>Create Static mut Game Object</li>
<li>RefCell< T > and the Interior Mutablilly Pattern</li>
<li>reguestAnimationFrame(callback)</li>
</ol>

