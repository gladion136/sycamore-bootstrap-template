
use sycamore::prelude::*;

fn main() {
    sycamore::render(|cx| view! { cx,
        // Header example
        ul(id="pillNav2", class="nav nav-pills nav-fill gap-2 p-1 small bg-primary rounded-5 shadow-sm", role="tabliist", style="--bs-nav-link-color: var(--bs-white); --bs-nav-pills-link-active-color: var(--bs-primary); --bs-nav-pills-link-active-bg: var(--bs-white);") {
            li(class="nav-item", role="presentation") {
                button(class="nav-link active rounded-5", id="home-tab2", data-bs-toggle="tab", type="button", role="tab", aria-selected="true") {
                    "Home"
                }
            }
            li(class="nav-item", role="presentation") {
                button(class="nav-link rounded-5", id="profile-tab2", data-bs-toggle="tab", type="button", role="tab", aria-selected="false") {
                    "Profile"
                }
            }
            li(class="nav-item", role="presentation") {
                button(class="nav-link rounded-5", id="contact-tab2", data-bs-toggle="tab", type="button", role="tab", aria-selected="false") {
                    "Contact"
                }
            }
        }

        // Head Banner example
        div(style = "display: flex; flex-direction: column; justify-content: center; align-items: center; height: 20vh;") {
            h1 { "Welcome to Sycamore!" }
            p {
                "This is just an example app. Try changing some code inside "
                code { "src/main.rs" }
                " and you'll be able to see the results here!"
            }
        }

        // Scrollspy example
        div(class="row") {
            div(class="col-4") {
                nav(id="navbar-example3", class="h-100 flex-column align-items-stretch pe-4 border-end", style="position: fixed;")  {
                    nav(class="nav nav-pills flex-column") {
                        a(class="nav-link", href="#item-1") {
                            "Item 1"
                        }
                        nav(class="nav nav-pills flex-column") {
                            a(class="nav-link ms-3 my-1", href="#item-1-1") {
                                "Item 1-1"
                            }
                            a(class="nav-link ms-3 my-1", href="#item-1-2") {
                                "Item 1-2"
                            }
                        }
                        a(class="nav-link", href="#item-2") {
                            "Item 2"
                        }
                        a(class="nav-link", href="#item-3") {
                            "Item 3"
                        }
                        nav(class="nav nav-pills flex-column") {
                            a(class="nav-link ms-3 my-1", href="#item-3-1") {
                                "Item 3-1"
                            }
                            a(class="nav-link ms-3 my-1", href="#item-3-2") {
                                "Item 3-2"
                            }
                        }
                    }
                }
            }
            div(class="col-8") {
                div(data-bs-spy="scroll", data-bs-target="#navbar-example3", data-bs-smooth-scroll="true", class="scrollspy-example-2", tabindex="0", style="height: 20vh;") {
                    div(id="item-1") {
                        h4 { "Item 1" }
                        p { "asdbiad iuU HDIUH ASHD PUADIU AS AI ASD BJABDJHSDB as d asd as das d asd as da sd as da asdas das asdbiad iuU HDIUH ASHD PUADIU AS AI ASD BJABDJHSDB as d asd as das d asd as da sd as da asdas das asdbiad iuU HDIUH ASHD PUADIU AS AI ASD BJABDJHSDB as d asd as das d asd as da sd as da asdas das asdbiad iuU HDIUH ASHD PUADIU AS AI ASD BJABDJHSDB as d asd as das d asd as da sd as da asdas das asdbiad iuU HDIUH ASHD PUADIU AS AI ASD BJABDJHSDB as d asd as das d asd as da sd as da asdas das asdbiad iuU HDIUH ASHD PUADIU AS AI ASD BJABDJHSDB as d asd as das d asd as da sd as da asdas das asdbiad iuU HDIUH ASHD PUADIU AS AI ASD BJABDJHSDB as d asd as das d asd as da sd as da asdas das asdbiad iuU HDIUH ASHD PUADIU AS AI ASD BJABDJHSDB as d asd as das d asd as da sd as da asdas das asdbiad iuU HDIUH ASHD PUADIU AS AI ASD BJABDJHSDB as d asd as das d asd as da sd as da asdas das asdbiad iuU HDIUH ASHD PUADIU AS AI ASD BJABDJHSDB as d asd as das d asd as da sd as da asdas das asdbiad iuU HDIUH ASHD PUADIU AS AI ASD BJABDJHSDB as d asd as das d asd as da sd as da asdas das asdbiad iuU HDIUH ASHD PUADIU AS AI ASD BJABDJHSDB as d asd as das d asd as da sd as da asdas das asdbiad iuU HDIUH ASHD PUADIU AS AI ASD BJABDJHSDB as d asd as das d asd as da sd as da asdas das asdbiad iuU HDIUH ASHD PUADIU AS AI ASD BJABDJHSDB as d asd as das d asd as da sd as da asdas das " }
                    }
                    div(id="item-1-1") {
                        h5 { "Item 1-1" }
                        p { "asdbiad iuU HDIUH ASHD PUADIU AS AI ASD BJABDJHSDB as d asd as das d asd as da sd as da asdas das asdbiad iuU HDIUH ASHD PUADIU AS AI ASD BJABDJHSDB as d asd as das d asd as da sd as da asdas das asdbiad iuU HDIUH ASHD PUADIU AS AI ASD BJABDJHSDB as d asd as das d asd as da sd as da asdas das asdbiad iuU HDIUH ASHD PUADIU AS AI ASD BJABDJHSDB as d asd as das d asd as da sd as da asdas das asdbiad iuU HDIUH ASHD PUADIU AS AI ASD BJABDJHSDB as d asd as das d asd as da sd as da asdas das asdbiad iuU HDIUH ASHD PUADIU AS AI ASD BJABDJHSDB as d asd as das d asd as da sd as da asdas das asdbiad iuU HDIUH ASHD PUADIU AS AI ASD BJABDJHSDB as d asd as das d asd as da sd as da asdas das asdbiad iuU HDIUH ASHD PUADIU AS AI ASD BJABDJHSDB as d asd as das d asd as da sd as da asdas das asdbiad iuU HDIUH ASHD PUADIU AS AI ASD BJABDJHSDB as d asd as das d asd as da sd as da asdas das asdbiad iuU HDIUH ASHD PUADIU AS AI ASD BJABDJHSDB as d asd as das d asd as da sd as da asdas das asdbiad iuU HDIUH ASHD PUADIU AS AI ASD BJABDJHSDB as d asd as das d asd as da sd as da asdas das asdbiad iuU HDIUH ASHD PUADIU AS AI ASD BJABDJHSDB as d asd as das d asd as da sd as da asdas das asdbiad iuU HDIUH ASHD PUADIU AS AI ASD BJABDJHSDB as d asd as das d asd as da sd as da asdas das asdbiad iuU HDIUH ASHD PUADIU AS AI ASD BJABDJHSDB as d asd as das d asd as da sd as da asdas das " }
                    }
                    div(id="item-1-2") {
                        h5 { "Item 1-2" }
                        p { "asdbiad iuU HDIUH ASHD PUADIU AS AI ASD BJABDJHSDB as d asd as das d asd as da sd as da asdas das asdbiad iuU HDIUH ASHD PUADIU AS AI ASD BJABDJHSDB as d asd as das d asd as da sd as da asdas das asdbiad iuU HDIUH ASHD PUADIU AS AI ASD BJABDJHSDB as d asd as das d asd as da sd as da asdas das asdbiad iuU HDIUH ASHD PUADIU AS AI ASD BJABDJHSDB as d asd as das d asd as da sd as da asdas das asdbiad iuU HDIUH ASHD PUADIU AS AI ASD BJABDJHSDB as d asd as das d asd as da sd as da asdas das asdbiad iuU HDIUH ASHD PUADIU AS AI ASD BJABDJHSDB as d asd as das d asd as da sd as da asdas das asdbiad iuU HDIUH ASHD PUADIU AS AI ASD BJABDJHSDB as d asd as das d asd as da sd as da asdas das asdbiad iuU HDIUH ASHD PUADIU AS AI ASD BJABDJHSDB as d asd as das d asd as da sd as da asdas das asdbiad iuU HDIUH ASHD PUADIU AS AI ASD BJABDJHSDB as d asd as das d asd as da sd as da asdas das asdbiad iuU HDIUH ASHD PUADIU AS AI ASD BJABDJHSDB as d asd as das d asd as da sd as da asdas das asdbiad iuU HDIUH ASHD PUADIU AS AI ASD BJABDJHSDB as d asd as das d asd as da sd as da asdas das asdbiad iuU HDIUH ASHD PUADIU AS AI ASD BJABDJHSDB as d asd as das d asd as da sd as da asdas das asdbiad iuU HDIUH ASHD PUADIU AS AI ASD BJABDJHSDB as d asd as das d asd as da sd as da asdas das asdbiad iuU HDIUH ASHD PUADIU AS AI ASD BJABDJHSDB as d asd as das d asd as da sd as da asdas das " }
                    }
                    div(id="item-2") {
                        h4 { "Item 2" }
                        p { "asdbiad iuU HDIUH ASHD PUADIU AS AI ASD BJABDJHSDB as d asd as das d asd as da sd as da asdas das asdbiad iuU HDIUH ASHD PUADIU AS AI ASD BJABDJHSDB as d asd as das d asd as da sd as da asdas das asdbiad iuU HDIUH ASHD PUADIU AS AI ASD BJABDJHSDB as d asd as das d asd as da sd as da asdas das asdbiad iuU HDIUH ASHD PUADIU AS AI ASD BJABDJHSDB as d asd as das d asd as da sd as da asdas das asdbiad iuU HDIUH ASHD PUADIU AS AI ASD BJABDJHSDB as d asd as das d asd as da sd as da asdas das asdbiad iuU HDIUH ASHD PUADIU AS AI ASD BJABDJHSDB as d asd as das d asd as da sd as da asdas das asdbiad iuU HDIUH ASHD PUADIU AS AI ASD BJABDJHSDB as d asd as das d asd as da sd as da asdas das asdbiad iuU HDIUH ASHD PUADIU AS AI ASD BJABDJHSDB as d asd as das d asd as da sd as da asdas das asdbiad iuU HDIUH ASHD PUADIU AS AI ASD BJABDJHSDB as d asd as das d asd as da sd as da asdas das asdbiad iuU HDIUH ASHD PUADIU AS AI ASD BJABDJHSDB as d asd as das d asd as da sd as da asdas das asdbiad iuU HDIUH ASHD PUADIU AS AI ASD BJABDJHSDB as d asd as das d asd as da sd as da asdas das asdbiad iuU HDIUH ASHD PUADIU AS AI ASD BJABDJHSDB as d asd as das d asd as da sd as da asdas das asdbiad iuU HDIUH ASHD PUADIU AS AI ASD BJABDJHSDB as d asd as das d asd as da sd as da asdas das asdbiad iuU HDIUH ASHD PUADIU AS AI ASD BJABDJHSDB as d asd as das d asd as da sd as da asdas das " }
                    }
                    div(id="item-3") {
                        h4 { "Item 3" }
                        p { "asdbiad iuU HDIUH ASHD PUADIU AS AI ASD BJABDJHSDB as d asd as das d asd as da sd as da asdas das asdbiad iuU HDIUH ASHD PUADIU AS AI ASD BJABDJHSDB as d asd as das d asd as da sd as da asdas das asdbiad iuU HDIUH ASHD PUADIU AS AI ASD BJABDJHSDB as d asd as das d asd as da sd as da asdas das asdbiad iuU HDIUH ASHD PUADIU AS AI ASD BJABDJHSDB as d asd as das d asd as da sd as da asdas das asdbiad iuU HDIUH ASHD PUADIU AS AI ASD BJABDJHSDB as d asd as das d asd as da sd as da asdas das asdbiad iuU HDIUH ASHD PUADIU AS AI ASD BJABDJHSDB as d asd as das d asd as da sd as da asdas das asdbiad iuU HDIUH ASHD PUADIU AS AI ASD BJABDJHSDB as d asd as das d asd as da sd as da asdas das asdbiad iuU HDIUH ASHD PUADIU AS AI ASD BJABDJHSDB as d asd as das d asd as da sd as da asdas das asdbiad iuU HDIUH ASHD PUADIU AS AI ASD BJABDJHSDB as d asd as das d asd as da sd as da asdas das asdbiad iuU HDIUH ASHD PUADIU AS AI ASD BJABDJHSDB as d asd as das d asd as da sd as da asdas das asdbiad iuU HDIUH ASHD PUADIU AS AI ASD BJABDJHSDB as d asd as das d asd as da sd as da asdas das asdbiad iuU HDIUH ASHD PUADIU AS AI ASD BJABDJHSDB as d asd as das d asd as da sd as da asdas das asdbiad iuU HDIUH ASHD PUADIU AS AI ASD BJABDJHSDB as d asd as das d asd as da sd as da asdas das asdbiad iuU HDIUH ASHD PUADIU AS AI ASD BJABDJHSDB as d asd as das d asd as da sd as da asdas das " }
                    }
                    div(id="item-3-1") {
                        h5 { "Item 3-1" }
                        p { "asdbiad iuU HDIUH ASHD PUADIU AS AI ASD BJABDJHSDB as d asd as das d asd as da sd as da asdas das asdbiad iuU HDIUH ASHD PUADIU AS AI ASD BJABDJHSDB as d asd as das d asd as da sd as da asdas das asdbiad iuU HDIUH ASHD PUADIU AS AI ASD BJABDJHSDB as d asd as das d asd as da sd as da asdas das asdbiad iuU HDIUH ASHD PUADIU AS AI ASD BJABDJHSDB as d asd as das d asd as da sd as da asdas das asdbiad iuU HDIUH ASHD PUADIU AS AI ASD BJABDJHSDB as d asd as das d asd as da sd as da asdas das asdbiad iuU HDIUH ASHD PUADIU AS AI ASD BJABDJHSDB as d asd as das d asd as da sd as da asdas das asdbiad iuU HDIUH ASHD PUADIU AS AI ASD BJABDJHSDB as d asd as das d asd as da sd as da asdas das asdbiad iuU HDIUH ASHD PUADIU AS AI ASD BJABDJHSDB as d asd as das d asd as da sd as da asdas das asdbiad iuU HDIUH ASHD PUADIU AS AI ASD BJABDJHSDB as d asd as das d asd as da sd as da asdas das asdbiad iuU HDIUH ASHD PUADIU AS AI ASD BJABDJHSDB as d asd as das d asd as da sd as da asdas das asdbiad iuU HDIUH ASHD PUADIU AS AI ASD BJABDJHSDB as d asd as das d asd as da sd as da asdas das asdbiad iuU HDIUH ASHD PUADIU AS AI ASD BJABDJHSDB as d asd as das d asd as da sd as da asdas das asdbiad iuU HDIUH ASHD PUADIU AS AI ASD BJABDJHSDB as d asd as das d asd as da sd as da asdas das asdbiad iuU HDIUH ASHD PUADIU AS AI ASD BJABDJHSDB as d asd as das d asd as da sd as da asdas das " }
                    }
                    div(id="item-3-2") {
                        h5 { "Item 3-2" }
                        p { "asdbiad iuU HDIUH ASHD PUADIU AS AI ASD BJABDJHSDB as d asd as das d asd as da sd as da asdas das asdbiad iuU HDIUH ASHD PUADIU AS AI ASD BJABDJHSDB as d asd as das d asd as da sd as da asdas das asdbiad iuU HDIUH ASHD PUADIU AS AI ASD BJABDJHSDB as d asd as das d asd as da sd as da asdas das asdbiad iuU HDIUH ASHD PUADIU AS AI ASD BJABDJHSDB as d asd as das d asd as da sd as da asdas das asdbiad iuU HDIUH ASHD PUADIU AS AI ASD BJABDJHSDB as d asd as das d asd as da sd as da asdas das asdbiad iuU HDIUH ASHD PUADIU AS AI ASD BJABDJHSDB as d asd as das d asd as da sd as da asdas das asdbiad iuU HDIUH ASHD PUADIU AS AI ASD BJABDJHSDB as d asd as das d asd as da sd as da asdas das asdbiad iuU HDIUH ASHD PUADIU AS AI ASD BJABDJHSDB as d asd as das d asd as da sd as da asdas das asdbiad iuU HDIUH ASHD PUADIU AS AI ASD BJABDJHSDB as d asd as das d asd as da sd as da asdas das asdbiad iuU HDIUH ASHD PUADIU AS AI ASD BJABDJHSDB as d asd as das d asd as da sd as da asdas das asdbiad iuU HDIUH ASHD PUADIU AS AI ASD BJABDJHSDB as d asd as das d asd as da sd as da asdas das asdbiad iuU HDIUH ASHD PUADIU AS AI ASD BJABDJHSDB as d asd as das d asd as da sd as da asdas das asdbiad iuU HDIUH ASHD PUADIU AS AI ASD BJABDJHSDB as d asd as das d asd as da sd as da asdas das asdbiad iuU HDIUH ASHD PUADIU AS AI ASD BJABDJHSDB as d asd as das d asd as da sd as da asdas das " }
                    }
                }

            }
        }
        script(src = "https://cdn.jsdelivr.net/npm/bootstrap@5.3.2/dist/js/bootstrap.bundle.min.js", crossorigin="anonymous") {}
    });
}
