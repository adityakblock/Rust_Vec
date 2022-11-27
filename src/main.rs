use myvec::MyVec;
fn main() {
    let mut vec: MyVec<usize>= MyVec::<usize>::new();
    vec.push(1);
    vec.push(2);
    vec.push(3);
    vec.push(4);
    vec.push(5);
    assert_eq!(vec.capacity(), 8);
    assert_eq!(vec.len(), 5);
}