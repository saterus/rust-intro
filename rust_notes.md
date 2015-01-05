# Outline

- differences between traits and interfaces
  - extendable after "class" definition
  - default implementation
  - trait bounds on functions change compilation rules
- memory intro
  - rust makes you the best c++ programmer that ever lived
  - riia
- rework memory slides
- missing deref in ownership slides


- mailing list
- basic website

- lifetimes
- ownership vs view vs loan

~ JSON Serialization
- FFI & Unsafe
+ Documentation
+ Popular Projects on Github
+ Resources/Tutorials/Guides/play.rustlang.org
- Macros
- rand::*
- Implementing PartialEq, PartialOrd on Point

- quickcheck
- stainless
- ffi
- peek at Arc

Memory Hierarchy
https://commons.wikimedia.org/wiki/File%3AComputerMemoryHierarchy.png
By User:Danlash at en.wikipedia.org [Public domain], via Wikimedia Commons

- amazing amount of smart people paid by mozilla to think about hard problems ruby ignores

Nitty Gritty Memory
- oh yeah, structs are groups of memory, without overhead.
  - Struct { x: i32 } => 4 bytes. that's it.
- when i declare stuff, which things are stack vs heap allocated?
  - args & locals => stack
  - box => heap
- what data is stored in a pointer?
- what happens when i create a reference pointer?
- what happens when i dereference a pointer?
- what data is actually stored in an Enum?
- what happens when i match on an Option?

  let y = &w;          // borrowed reference to w's memory
  let z = &x;          // borrowed reference to x's memory

  let a = w; // => ok! stack memory copied
  let b = x; // => error: cannot move out of `x` because it is borrowed

- Our Lens
  - Rails consultant
    - We're generalist experts
    - We're narrow experts
    - We're generalist Dunning Kruger victims
    - Our stack: Ruby + Javascript + HTML + CSS + SQL
    - I'm going to pick on Ruby throughout this talk
  - The Good Ruby
    - Developer happiness!
    - High level, concise, expressive, flexible language
    - Mature ecosystem of easily installed libaries
    - Great testing culture & tools
    - We're experts!
  - The Bad Ruby
    - About as slow as programming languages get...
    - Not easily improved w/ concurrency
      - GIL
    - Or with FFI
      - Trade C FFI for JVM FFI...
    - Little to no useful static analysis possible
      - Lots of "obvious" tests required
    - Difficult to improve some of these problems

- An Unfamiliar Lens
  - The Good C*
    - Capital F Fast
    - Great FFI
    - Mature ecosystem of libraries
    - Static analysis is genuinely useful
  - The Bad C*
    - Low level, verbose, 




# Unconsumed Notes:

- what presentation tool?
- modern C++?
- haskell with braces?
- bitrot

