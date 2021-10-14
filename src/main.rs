// Licensed under the Apache License, Version 2.0 (the "License");
// you may not use this file except in compliance with the License.
// You may obtain a copy of the License at
//
//     http://www.apache.org/licenses/LICENSE-2.0
//
// Unless required by applicable law or agreed to in writing, software
// distributed under the License is distributed on an "AS IS" BASIS,
// WITHOUT WARRANTIES OR CONDITIONS OF ANY KIND, either express or implied.
// See the License for the specific language governing permissions and
// limitations under the License.

use clap::{App, Arg};

fn main() {
    let app = App::new("metalmetrics-rs")
        .version(concat!(env!("CARGO_PKG_VERSION"), "-build-", env!("build")))
        .arg(
            Arg::new("attachments")
                .long("attachments")
                .value_name("ATTACHMENTS")
                .about("Attachments list (attach,...)")
                .takes_value(true)
                .required(false),
        )
        .arg(
            Arg::new("body")
                .long("body")
                .value_name("BODY")
                .about("Body content")
                .takes_value(true)
                .required(false),
        )
        .arg(
            Arg::new("config")
                .long("config")
                .value_name("CONFIG")
                .about("Config file (.yml)")
                .takes_value(true)
                .required(true),
        )
        .arg(
            Arg::new("header")
                .long("header")
                .value_name("HEADER")
                .about("Header content")
                .takes_value(true)
                .required(false),
        )
        .arg(
            Arg::new("recipients")
                .long("recipients")
                .value_name("RECIPIENTS")
                .about("Recipients list (alen@example.com,cc:bob@example.com,...)")
                .takes_value(true)
                .required(true),
        )
        .arg(
            Arg::new("title")
                .long("title")
                .value_name("TITLE")
                .about("Title content")
                .takes_value(true)
                .required(true),
        )
        .get_matches();

    if let Some(attachments) = app.value_of("attachments") {
        println!("{}", attachments);
    }
}
