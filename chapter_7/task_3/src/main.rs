fn closure_test<F>(f: F)->bool where F: Fn()->bool{
    f()
}

fn main() {
    closure_test(|| true);
}
