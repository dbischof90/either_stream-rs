use either_stream::EitherStreamExt;
use futures::{future, stream, stream::Stream, *};

fn create_either_stream() -> stream::IterOk<std::vec::IntoIter<future::Either<i32, i32>>, ()> {
    let a: future::Either<i32, i32> = future::Either::A(1);
    let b: future::Either<i32, i32> = future::Either::B(2);
    let c: future::Either<i32, i32> = future::Either::A(3);
    let d: future::Either<i32, i32> = future::Either::B(4);
    stream::iter_ok::<_, ()>(vec![a, b, c, d])
}

#[test]
fn map_left_integers() {
    let stream = create_either_stream();
    let result = stream
        .map_left(|x| 2 * x)
        .and_then(|x| match x {
            future::Either::A(n) | future::Either::B(n) => Ok(n),
        })
        .collect()
        .wait()
        .unwrap();
    assert_eq!(result, vec![2, 2, 6, 4]);
}

#[test]
fn map_right_integers() {
    let stream = create_either_stream();
    let result = stream
        .map_right(|x| 2 * x)
        .and_then(|x| match x {
            future::Either::A(n) | future::Either::B(n) => Ok(n),
        })
        .collect()
        .wait()
        .unwrap();
    assert_eq!(result, vec![1, 4, 3, 8]);
}

#[test]
fn map_left_and_right_integers() {
    let stream = create_either_stream();
    let result = stream
        .map_right(|x| 2 * x)
        .map_left(|x| 2 * x)
        .and_then(|x| match x {
            future::Either::A(n) | future::Either::B(n) => Ok(n),
        })
        .collect()
        .wait()
        .unwrap();
    assert_eq!(result, vec![2, 4, 6, 8]);
}
