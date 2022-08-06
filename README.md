## Explicitly notate the distortion between the `Display`ed with its intention effortlessly. 

---

# A thought experiment

Picture what you reading now as a vector floating in a Hilbert space.

If you close your eye, you should see a fixed line with an arrow head, alone and still in a void. 

Note that this vector represents what you are reading, more precisly, what I expressed, it is what I made observale, however, not my intention, cause intention simply can not be observed.

But you can place another vector in that same void, represents my intention. You can do that, you just don't know their dimension.

Hilbert space assures that, if you project one on another, there exists an unique distortion between two vectors.

The non-observable now can be pictured by indicating an angle. 

Combining the assumption that intention can both evlove and recurse, what a trajactory could be drawn?





# Example

the following example shows 2 things:

- how Expression crawls from an intention.

- how an expressed can dig to its intention.

```rust
use ang::Angle;
use melcome::expression::{Intention, Expression,Crawling,Digging, Expressing, Url};

fn main() {
    let itn = Intention::new();
    let mut expr = Expression::from_itn(itn);
    let url = Box::new(Url("a thought experiment#0000".to_string()));
    expr.express(url);
    let cta = Angle::Degrees(24.0);
    expr.judge(cta);
    println!("{expr}\n");
    let url2 = Box::new(Url("Example#0000".to_string()));
    let mut expr2 = Expression::from_obe(url2);
    expr2.join(&expr);
    println!("{expr2}\n")
}
```
any struct implemented `Observable` should be able to `express`.
```
Debug:
Expression { intention: Intention { id: None, origin: Some(Intention { id: Some(4b14c2a5-0a63-4456-b7ff-099740416141), origin: None }) }, distortion: [Some(Degrees(24.0))], expressed: [Url("a thought experiment#0000")] }

Debug:
Expression { intention: Intention { id: None, origin: Some(Intention { id: None, origin: Some(Intention { id: Some(4b14c2a5-0a63-4456-b7ff-099740416141), origin: None }) }) }, distortion: [None], expressed: [Url("Example#0000")] } 
```
