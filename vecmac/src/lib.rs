#[macro_export]
macro_rules! avec {
    // double curly braces to say this is a block
    // if we have multiple line and the last line is what we want to return
    // ($(element: expr),+ = 1 ore more expr
    // $(,)? adding an optional trailling comma
    // $($element: expr),* to say ) or more repeatation
    ($($element: expr),* $(,)?) => {{
        // check that count is const
        const C: usize =  $crate::count![@COUNT; $($element),*];
        // allow not pushing when empty
        #[allow(unused_mut)]
        let mut vs = Vec::with_capacity(C);
        // repeat what inside the () same number of time as the patter
        // that $element in it. So we can pass as many element as we want
        $(vs.push($element);)*
        vs
    }};

    ($($element:expr),*) => {
        // $($element),*: 0 or more
        $crate::avec![$($element),*]
    };


    ($element: expr; $count: expr) => {{
        let count = $count;
        // let mut vs = Vec::new(): calling new and doing pushes. Whenever we push we create a reallocation of the vector
        // because Vec have defeault capacity around 16, so after we push 16, it has to reallocate copy of all elements to an Vec that 2x larger than
        // the origial vector and so on. That is why we use with_capacity
        let mut vs = Vec::with_capacity(count);
        // define x here and clone above otherwise it does not work with nonliteral type
        // with Option only the fist take will success because there is only 1 interior value
        // we have to clone it everytime we push
        /*
            let x = $element;
            for _ in 0..count{
                // push
                vs.push(x.clone());
            }
        */

        // when we do push, it has to do pointer increment (bound check)

        // vs.extend(std::iter::repeat($element).take(count));

        // another option is to use resize. This does not do bound check. it is more effecient.
        // with resize we dont have to use Vec::with_capacity instead we can just use Vec::new()
        vs.resize(count, $element);

        vs
    }}
}

#[macro_export]
#[doc(hidden)]
macro_rules!  count{
    (@COUNT; $($element: expr),*) => {
        // if we use [()), *] it will throw error
        // because rust does not know what variable inside
        // we have to use SUBST take element and return ()
        // explanation for <[()]> https://stackoverflow.com/a/70407322/10104154
        <[()]>::len(&[$($crate::count![@SUBST; $element]), *])
    };
    (@SUBST; $_element:expr) => { () };
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
