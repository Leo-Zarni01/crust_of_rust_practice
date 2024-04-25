#[macro_export]
macro_rules! avec {
    () => {
        Vec::new()
    };

    ($($element: expr),+ $(,)? ) => { 
        {
            let mut vs = Vec::new();
            $(vs.push($element);)* // repeat vs.push(element) as many times as indicated by the
                                   // asterisk
            vs
        }
    };

    ($element: expr; $count: expr) => {
        {
            let mut vs = Vec::new();
            let x = $element;
            for _ in 0..$count {
                vs.push(x.clone());
            }
            vs
        }
    };
}

#[test]
fn empty_vec() {
    let x: Vec<u32> = avec!();
    assert!(x.is_empty());
}

#[test]
fn single() {
    let x: Vec<u32> = avec!(42);
    assert!(!x.is_empty());
    assert_eq!(x.len(), 1);
    assert_eq!(x[0], 42);
}

#[test]
fn double() {
    let x: Vec<u32> = avec!(42, 43);
    assert!(!x.is_empty());
    assert_eq!(x.len(), 2);
    assert_eq!(x[0], 42);
    assert_eq!(x[1], 43);
}

#[test]
fn clone_2_nonliteral() {
    let mut y = Some(42);
    let x: Vec<u32> = avec!(y.take().unwrap(); 2); // push 42 2 times
    assert!(!x.is_empty());
    assert_eq!(x.len(), 2);
    assert_eq!(x[0], 42);
    assert_eq!(x[1], 42);
}
