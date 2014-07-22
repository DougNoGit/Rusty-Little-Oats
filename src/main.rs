extern crate http;
extern crate iron;
extern crate staticfile;
extern crate time;

use time::now;

use std::io::net::ip::Ipv4Addr;

use iron::{Iron, Chain, Alloy, Request, Response, Server, Status, Continue, FromFn,};
use iron::mixin::Serve;

use staticfile::Static;

fn come_again(_req: &mut Request, res: &mut Response, _alloy: &mut Alloy) -> Status 
{
    let _ = res.serve(::http::status::Ok, "Wrong time, come back later.");
    Continue
}

fn main() 
{
    loop 
    {
    let mut server: Server = Iron::new(); 
    if time::now().tm_hour == 17
    {
        if time::now().tm_min <= 1
        {
            server.chain.link(Static::new(Path::new("./")));
        }
    }
    else 
    {
        server.chain.link(FromFn::new(come_again));
    }
    server.listen(Ipv4Addr(0, 0, 0, 0), 3000);
    }
}

