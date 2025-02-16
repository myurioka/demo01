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
    R->>H: Closure::wrap(Box::new(|_time:f64|()))
    H->>B: requestnimation()
    loop callback GAME.on_animation_frame
    B->>H: callback
    H->>H: Game.update()
    H->>H: Game.draw()
    H->>B: requestnimation()
    end
    R->>H: Closure::wrap(Box::new(|MouseEvnet|)())
    H->>B: add_event_listner_with_callback("mousedown")
    alt callback GAME.on_click
    H->>H: forget()
    B->>H: callback
    H->>H: Game.on_click
    end
```
<br>
<ol>
<li>Create Static mut Game Object👍</li>
<li>set interface function(closure) for requestAnimationFrame <br/> 👍RefCell< T > and the Interior Mutablilly Pattern<br/> After the first requestAnimationFrame call, the closure is disappointed</li>
<li>requestAnimationFrame(interface function for callback) in first</li>
<li>callback → Closure::wrap(Box::new(|_time:f64|())) 
<li>game update</li>
<li>game draw</li>
<li>requestAnimationFrame(interface function for callback) in loop</li>
<li>set interface function(closure) for MouseEvent at MouseDown</li>
<li>canvas.add_event_listener_with_callback("moudsedown")</li>
<li>forget() to keep interface function(closure)👍</li>
<li>MouseEvent(callback)</li>
<li>game set click position</li>
</ol>

