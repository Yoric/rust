use std;

fn main() {
    assert std::sys::rustrt::size_of::<fn#()>() ==
        std::sys::rustrt::size_of::<int>();
}