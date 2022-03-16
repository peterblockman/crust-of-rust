#[macro_export]
macro_rules! avec {
    () => {
        Vec::new()
    };
    // double curly braces to say this is a block
    // if we have multiple line and the last line is what we want to return
    // ($(element: expr),+ = 1 ore more expr
    // $(,)? adding an optional trailling comma
    ($($element: expr),+ $(,)?) => {{
        let mut vs = Vec::new();
        // repeat what inside the () same number of time as the patter
        // that $element in it. So we can pass as many element as we want
        $(vs.push($element);)*
        vs
    }};

    ($element: expr; $count: expr) => {{
        let mut vs = Vec::new();
        // define x here and clone above otherwise it does not work with nonliteral type
        // with Option only the fist take will success because there is only 1 interior value
        // we have to clone it everytime we push
        let x = $element;
        for _ in 0..$count{
            vs.push(x.clone());
        }
        vs
    }}


}

#[test]
fn empty_vec() {
    let x: Vec<u32> = avec![];
    assert!(x.is_empty());
}

#[test]
fn single() {
    let x: Vec<u32> = avec![42];
    assert!(!x.is_empty());
    assert_eq!(x.len(), 1);
    assert_eq!(x[0], 42);
}

#[test]
fn double() {
    let x: Vec<u32> = avec![42, 43];
    assert!(!x.is_empty());
}

#[test]
fn trailling() {
    let x: Vec<u32> = avec![1, 2, 3, 4,];
    assert!(!x.is_empty());
}

#[test]
fn clone_2() {
    let x: Vec<u32> = avec![42; 2];
    assert!(!x.is_empty());
    assert_eq!(x.len(), 2);
    assert_eq!(x[0], 42);
    assert_eq!(x[1], 42);
}

#[test]
fn clone_2_nonliterla() {
    let mut y = Some(42);
    let x: Vec<u32> = avec![y.take().unwrap(); 2];
    assert!(!x.is_empty());
    assert_eq!(x.len(), 2);
    assert_eq!(x[0], 42);
    assert_eq!(x[1], 42);
}
