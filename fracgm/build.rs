// Copyright 2024 the FracGM authors. All rights reserved.
// Use of this source code is governed by a BSD-style
// license that can be found in the LICENSE file.

extern crate cc;
use std::env;

fn main() {
    if env::var("CARGO_FEATURE_MCIS").is_ok() {
        println!("cargo:rustc-link-lib=gomp");

        let mut builder = cc::Build::new();

        builder
            .cpp(true)
            .flag("-std=c++11")
            .include("./src/mcis/pmc/include")
            .include("./src/mcis/eigen")
            .file("./src/mcis/pmc/pmc_heu.cpp")
            .file("./src/mcis/pmc/pmc_maxclique.cpp")
            .file("./src/mcis/pmc/pmcx_maxclique.cpp")
            .file("./src/mcis/pmc/pmcx_maxclique_basic.cpp")
            .file("./src/mcis/pmc/pmc_cores.cpp")
            .file("./src/mcis/pmc/pmc_utils.cpp")
            .file("./src/mcis/pmc/pmc_graph.cpp")
            .file("./src/mcis/pmc/pmc_clique_utils.cpp")
            .file("./src/mcis/graph.cpp")
            .file("./src/mcis/inlier_selection.cpp");

        builder.flag("-fopenmp");
        builder.flag("-w"); // Disable pmc warning messages

        builder.compile("mcis");
    }
}
