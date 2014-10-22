# Outline

- Picture Worth 1000 Words
  - ![red_blue_knowing.jpg]
  - ![pacman_pie_chart.jpg]

- Why Learn a New Language?
  - Learning for the sake of knowledge
    - "Good for you" / "learn one language per year"
    - I like to learn
  - Where is the industry going?
    - Keeping ahead of the curve
    - [Red Queen's Race](https://en.wikipedia.org/wiki/Red_Queen%27s_race)
    - "Well, in our country," said Alice, still panting a little, "you'd generally get to somewhere else—if you run very fast for a long time, as we've been doing."
      "A slow sort of country!" said the Queen. "Now, here, you see, it takes all the running you can do, to keep in the same place. If you want to get somewhere else, you must run at least twice as fast as that!"
      - Lewis Carroll's Through the Looking-Glass
    - We all believe this is important, as we all learned Ruby.
  - Perspective
    - New ideas
      - What do other people think are strengths?
      - What do they gain?
    - Current tools stale/inadequate
      - Look back at what I use today, what are its strengths?
      - What are its weaknesses?
      - What do I sacrifice?
      - What do I take for granted?
  - New Opportunity
    - What doors does this open?

- Rust
  - Design Goal: "To design and implement a safe, concurrent, practical, static systems language."
  - Developed by Mozilla
  - Servo: browser engine.
    - Just passed the Acid 2 test, roughly CSS 2.1.
    - Concurrency, parallelism, safety, reliable
  - Features List
    - Feature by feature, compared to Ruby and others for perspective

- Variables
  - Good defaults. Immutability.
      ```let x = 12; // immutable!```


+ stdout
+ Types
+ Expressions
+ Functions
+ Structs & Enums
+ Matching
+ Closures
+ Tests
+ Traits
+ Option/Result
+ Cargo
+ Modules
+ Pointers
+ Lifetimes
+ Ownership
+ Strings
+ Slices
+ Iterators
+ Common Traits & Kinds
+ Tasks
  + Arc
~ JSON Serialization
- FFI & Unsafe
+ Documentation
+ Popular Projects on Github
+ Resources/Tutorials/Guides/play.rustlang.org
- Macros
  - They exist.
  - Better than C Macros.
  - I know nothing.
  - Black magic.

- rand::*
- Implementing PartialEq, PartialOrd on Point

+ note that Option is already defined
~ more match
  - exhaustive
+ typos:
  + contains
  + value of pi

1.0
- low level power & flexibility
- high level safety & convenience


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

