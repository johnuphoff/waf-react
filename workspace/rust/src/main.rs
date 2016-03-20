/**
 * File        : main.rs
 * Description : Web server.
 * Copyright   : Copyright (c) 2016 Mirror Labs Inc. All rights reserved.
 * Maintainer  : Enzo Haussecker <enzo@mirror.co>
 * Stability   : Stable
 * Portability : Portable
 *
 * NOTICE: Permission is hereby granted, free of charge, to any person obtaining
 * a copy of this software and associated documentation files (the "Software"),
 * to deal in the Software without restriction, including without limitation the
 * rights to use, copy, modify, merge, publish, distribute, sublicense, and/or
 * sell copies of the Software, and to permit persons to whom the Software is
 * furnished to do so, subject to the following conditions:
 *
 * The above copyright notice and this permission notice shall be included in
 * all copies or substantial portions of the Software.
 *
 * THE SOFTWARE IS PROVIDED "AS IS", WITHOUT WARRANTY OF ANY KIND, EXPRESS OR
 * IMPLIED, INCLUDING BUT NOT LIMITED TO THE WARRANTIES OF MERCHANTABILITY,
 * FITNESS FOR A PARTICULAR PURPOSE AND NONINFRINGEMENT. IN NO EVENT SHALL THE
 * AUTHORS OR COPYRIGHT HOLDERS BE LIABLE FOR ANY CLAIM, DAMAGES OR OTHER
 * LIABILITY, WHETHER IN AN ACTION OF CONTRACT, TORT OR OTHERWISE, ARISING FROM,
 * OUT OF OR IN CONNECTION WITH THE SOFTWARE OR THE USE OR OTHER DEALINGS IN THE
 * SOFTWARE.
 *
 * This module implements a simple static file server.
 */

extern crate iron;
extern crate mount;
extern crate staticfile;

use iron::Iron;
use mount::Mount;
use staticfile::Static;
use std::path::Path;

fn main() {
   let mut mount = Mount::new();
   mount.mount("/", Static::new(Path::new("/tmp/assets")));
   Iron::new(mount).http("0.0.0.0:8008").unwrap();
}