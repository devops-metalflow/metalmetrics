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
            Arg::new("config-file")
                .long("config_file")
                .value_name("NAME")
                .about("config file (.yml)")
                .takes_value(true)
                .required(true),
        )
        .arg(
            Arg::new("inxi-file")
                .long("inxi_file")
                .value_name("NAME")
                .about("inxi file (/path/to/inxi)")
                .takes_value(true)
                .required(false),
        )
        .arg(
            Arg::new("listen-url")
                .long("listen_url")
                .value_name("URL")
                .about("listen url (host:port)")
                .takes_value(true)
                .required(false),
        )
        .arg(
            Arg::new("output-file")
                .long("output_file")
                .value_name("NAME")
                .about("output file (.json|.txt|.xlsx)")
                .takes_value(true)
                .required(false),
        )
        .get_matches();
}
