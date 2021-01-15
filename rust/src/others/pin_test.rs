use std::marker::PhantomPinned;
use std::pin::Pin;
use std::ptr::NonNull;

// This is a self-referential struct because the slice field points to the data field.
// We cannot inform the compiler about that with a normal reference,
// as this pattern cannot be described with the usual borrowing rules.
// Instead we use a raw pointer, though one which is known not to be null,
// as we know it's pointing at the string.
struct Unmovable {
    data: String,
    slice: NonNull<String>,
    vec: Vec<usize>,
    _pin: PhantomPinned,
}

impl Unmovable {
    // To ensure the data doesn't move when the function returns,
    // we place it in the heap where it will stay for the lifetime of the object,
    // and the only way to access it would be through a pointer to it.
    fn new(data: String) -> Pin<Box<Self>> {
        let res = Unmovable {
            data,
            // we only create the pointer once the data is in place
            // otherwise it will have already moved before we even started
            slice: NonNull::dangling(),
            vec: vec![],
            _pin: PhantomPinned,
        };
        let mut boxed = Box::pin(res);

        let slice = NonNull::from(&boxed.data);
        // we know this is safe because modifying a field doesn't move the whole struct
        unsafe {
            let mut_ref: Pin<&mut Self> = Pin::as_mut(&mut boxed);
            Pin::get_unchecked_mut(mut_ref).slice = slice;
        }
        boxed
    }
}

pub fn solve() {
    let unmoved = Unmovable::new("hello".to_string());

    // The pointer should point to the correct location,
    // so long as the struct hasn't moved.
    // Meanwhile, we are free to move the pointer around.
    let mut still_unmoved = unmoved;
    assert_eq!(still_unmoved.slice, NonNull::from(&still_unmoved.data));

    let x = still_unmoved.as_mut();
    unsafe {
        Pin::get_unchecked_mut(x).vec.push(5);
    }
    println!("{:?}", still_unmoved.vec);

    // Since our type doesn't implement Unpin, this will fail to compile:
    // let mut new_unmoved = Unmovable::new("world".to_string());
    // std::mem::swap(&mut *still_unmoved, &mut *new_unmoved);
}
