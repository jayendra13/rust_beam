use crate::transforms::transform::PTransform;


// https://stackoverflow.com/questions/28136739/is-it-possible-to-control-the-size-of-an-array-using-the-type-parameter-of-a-gen
struct Create<T: Sized, const COUNT: usize> {
    elements: [T; COUNT],
};

impl Create {

}

impl PTransform for Create {

    fn exapand
}