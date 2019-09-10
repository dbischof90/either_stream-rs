use either_stream::EitherStreamExt;
use futures::{future, stream, stream::Stream, *};

fn create_either_stream(
) -> stream::IterOk<std::vec::IntoIter<future::Either<i32, i32>>, i32> {
    let a: future::Either<i32, i32> = future::Either::A(1);
    let b: future::Either<i32, i32> = future::Either::B(2);
    let c: future::Either<i32, i32> = future::Either::A(3);
    let d: future::Either<i32, i32> = future::Either::B(4);
    stream::iter_ok::<_, i32>(vec![a, b, c, d])
}

#[test]
fn and_then_left_integers() {
    let stream = create_either_stream();
    let result = stream
        .and_then_left(|x| Ok(2 * x))
        .map(|either| match either {
            future::Either::A(x) => x,
            future::Either::B(x) => x
        })
        .collect()
        .wait()
        .unwrap();
    assert_eq!(result, vec![2, 2, 6, 4]);
}


#[test]
fn and_then_left_shutdown_on_error() {
    let stream = create_either_stream();
    let result = stream
        .and_then_left(|x| Err(2 * x))
        .map(|either| match either {
            future::Either::A(x) => x,
            future::Either::B(x) => x
        })
        .collect()
        .wait();
    assert_eq!(result, Err(2));
}

#[test]
fn and_then_right_integers() {
    let stream = create_either_stream();
    let result = stream
        .and_then_right(|x| Ok(2 * x))
        .map(|either| match either {
            future::Either::A(x) => x,
            future::Either::B(x) => x
        })
        .collect()
        .wait()
        .unwrap();
    assert_eq!(result, vec![1, 4, 3, 8]);
}


#[test]
fn and_then_right_shutdown_on_error() {
    let stream = create_either_stream();
    let result = stream
        .and_then_right(|x| Err(2 * x))
        .map(|either| match either {
            future::Either::A(x) => x,
            future::Either::B(x) => x
        })
        .collect()
        .wait();
    assert_eq!(result, Err(4));
}


#[test]
fn and_then_left_right_joint_integers() {
    let stream = create_either_stream();
    let result = stream
        .and_then_left(|x| Ok(2 * x))
        .and_then_right(|x| Ok(2 * x))
        .map(|either| match either {
            future::Either::A(x) => x,
            future::Either::B(x) => x
        })
        .collect()
        .wait()
        .unwrap();
    assert_eq!(result, vec![2, 4, 6, 8]);
}
