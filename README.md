Provides select Rust bindings for CoreGraphics.  May be compared to [objrs_frameworks_core_graphics](https://crates.io/crates/objrs_frameworks_core_graphics)
and [core-graphics](https://crates.io/crates/core-graphics).

Part of the [objr expanded universe](https://github.com/drewcrawford/objr#objr-expanded-universe), distinctive features of coregraphicsr:

* Zero-cost abstractions.  Calling this library should perform identically to calling Foundation from Swift/ObjC applications.
    * Most of the magic happens in [objr](https://github.com/drewcrawford/objr)
      which provide cutting-edge high-performance primitives which are used here extensively.
* Safe APIs.  Where possible APIs are designed with safe abstractions to provide familiar guarantees to Rust developers
* Low-level.  These bindings assume familiarity with Foundation APIs and are not documented separately.
* Modular.  Foundation is a big library; individual features are gated with feature flags
* Free for noncommercial or "small commercial" use.

# Status
The following APIs are implemented
* CGPoint, CGFloat, CGSize, and CGRect
* CGColor (greyscale only currently)
* CGDirectDisplayID