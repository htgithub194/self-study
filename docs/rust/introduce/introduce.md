---
marp: true
theme: uncover
class: invert
style: |
  @import './theme.css';
paginate: true
---

# Brieft introduction to Rust
**ThongDinh ft DatDinh**

---
<!-- _class: slide-base invert-->
## Contents

- Get to know Rust
  - History of Rust
  - Notable features
  - How Rust invovles in our world
- Get dive deeper to know Rust
  - Why Rust is memory safety
  - Why Rust is fast

---
<!-- _class: slide-base invert-->
## Take away 1:
- Pros:
    memory safe
    speed
- Cons:
    hard to learn
    cost time to compile


---
<!-- _class: slide-base invert-->
## Memory unsafety in C/C++
- **Dangling pointers**: accessing memory after freeing
- **Double free**: freeing the same memory twice.
- **Memory leaks**: never freeing memory.
- Data Races in Multithreading
- Buffer Overflows / Out-of-Bounds Access

---
<!-- _class: slide-base invert-->
## Garbage Collection in Java

- Automatically finds and removes unused objects in the heap.
- Pros:
  - Ease of Development
  - Platform Independence.
- Cons:
  - Unpredictable Pauses
  - CPU Overhead
  - Bloat: extra memory for GC

---
<!-- _class: slide-base invert-->
## How Rust archives memory safety

![bg height: 40%](images/rules.png)

---
<!-- _class: slide-base invert-->
## How Rust archives memory safety
- Ownership model
- Lifetimes
- Communicate threads for sharing data

---
<!-- _class: slide-base invert-->
## Ownership model
- Enforce safety at compile time by RULES
- reference and mutable-reference
- Value can only be edited from 1 point

---
<!-- _class: slide-base invert-->
## Ownership model

![bg height: 75%](images/ownership.excalidraw.svg)


---
<!-- _class: slide-base invert-->
## Lifetime

![bg height: 95%](images/lifetime.excalidraw.svg)


---
<!-- _class: slide-base invert-->
## Multithread
- “Do not communicate by sharing memory
instead, share memory by communicating.”

---
<!-- _class: slide-base invert-->
## Multithread

![bg height: 100%](images/threads.excalidraw.svg)


---
<!-- _class: slide-base invert-->
## Speed
-  Build to native machine code --> fast ~ C/C++
- No GC -> faster than Java, Go, Python
- Zero-cost abstraction (same as C++)
- Optimized LLVM Backend (compiler)
- Support concurrent programming

---
<!-- _class: slide-base invert-->
## Speed benchmark vs C

https://programming-language-benchmarks.vercel.app/c-vs-rust

---
<!-- _class: slide-base invert-->
## Speed benchmark vs GO

![bg height: 65%](images/rust-vs-go-http-performance-2025.svg)

- https://markaicode.com/rust-vs-go-performance-benchmarks-microservices-2025/?utm_source=chatgpt.com


---
<!-- _class: slide-base invert-->
## Q & A

Thank you !!!