fn apply_fn<F>(f: &F, ch: char) where F: Fn(char) -> bool {
    assert!(f(ch));
}

fn apply_fn_mut<F>(f: &mut F, ch: char) where F: FnMut(char) -> bool {
    assert!(f(ch));
}

fn apply_fn_once<F>(f: F, ch: char) where F: FnOnce(char) -> bool {
    assert!(f(ch));
}


fn main() {
    let s1 = "read-only";
    let mut lookup = |ch| s1.find(ch).is_some();
    apply_fn(&lookup, 'r');
    apply_fn_mut(&mut lookup, 'o');
    apply_fn_once(lookup, 'y');
    assert_eq!(s1, "read-only");

    let mut s2 = "append".to_string();
    let mut modify = |ch| {
        s2.push(ch);
        true
    };
//    apply_fn(&modify, 'e'); // error[E0525]: expected a closure that implements the `Fn` trait, but this closure only implements `FnMut`
    apply_fn_mut(&mut modify, 'e');
    apply_fn_once(modify, 'd');
    assert_eq!(s2, "appended");

    let s3 = "be converted".to_string();
    let mut consume = |ch| {
        let bytes = s3.into_bytes();
        bytes.contains(&(ch as u8))
    };
//    apply_fn(&consume, 'b'); // error[E0525]: expected a closure that implements the `Fn` trait, but this closure only implements `FnOnce`
//    apply_fn_mut(&mut consume, 'c'); // error[E0525]: expected a closure that implements the `FnMut` trait, but this closure only implements `FnOnce`
    apply_fn_once(consume, 'd');
//    assert_eq!(s3, "error"); // error[E0382]: borrow of moved value: `s3`

//    let lookup = || assert!(s1.find('d').is_some());
//    let handle = std::thread::spawn(lookup);
    //error[E0373]: closure may outlive the current function, but it borrows `s1`, which is owned by the current function

    let lookup = move || assert!(s1.find('d').is_some());
    let handle = std::thread::spawn(lookup);
}
