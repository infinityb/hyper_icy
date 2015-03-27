# hyper_icy

Header implementations for `icy-metaint` and `Icy-MetaData` for
[hyper](https://github.com/hyperium/hyper).

## Example

```rust
extern crate hyper;
extern crate hyper_icy;

use hyper::server::{Request, Response};
use hyper::net::Fresh;
use hyper_icy::{IcyMetaData, IcyMetaInt};

fn icy_handler(req: Request, mut res: Response<Fresh>) {
    if let Some(metadata) = req.headers.get::<IcyMetaData>() {
        if metadata.is_enabled() {
            res.headers_mut().set(IcyMetaInt(16000));
        }
    }

    // ...
}
```