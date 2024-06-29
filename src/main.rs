//pub mod dataloading;

fn main() {
    println!("Hello, world!");

    Test::new().unwrap().run().unwrap();
}

slint::slint! {
    import { TabWidget } from "std-widgets.slint";

    export component Recipe inherits Window {
        width: 200px;
        height: 200px;
        TabWidget {
            Tab {
                title: "First";
                Rectangle { 
                    background: orange;
                    width: 50px;
                }
            }
            Tab {
                title: "Second";
                Rectangle { background: pink; }
            }
        }
    }

    export component Test inherits Window {
        Rectangle {
            height: 200px;
            theTabWidget := TabWidget {
                width: parent.width;
                Tab {
                    title: "First";
                    Rectangle { 
                        background: orange;
                    }
                }
                Tab {
                    title: "Second";
                    Rectangle { background: pink; }
                }
            }
        }
    }
}