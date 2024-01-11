
trait X {
    fn x(&mut self);
}

struct XX;

impl X for XX {
    fn x(&mut self) {
        todo!()
    }
}

impl Drop for XX{
    fn drop(&mut self) {
        println!("Dropping HasDrop!");
    }
}

struct C<'a> {
    mut_ref: Option<&'a mut dyn X>,
}

struct D<'a,'b>{
    mut_ref: Option<&'a mut (dyn X + 'b)>
}

struct E<'a>{
    mut_ref: Option<&'a mut (dyn X + 'static)>
}
//这不可能
// struct F<'a>{
//     mut_ref: Option<&'static mut (dyn X + 'a)>
// }
struct G<'a> {
    mut_ref: Option<&'a mut (dyn X+'a)>,
}
#[test]
fn test() {
    let mut b: Option<Box<dyn X>> = Some(Box::new(XX));

    // let c = C {
    //     mut_ref: b.as_deref_mut(),
    // };

    // let d = D{
    //     mut_ref: b.as_deref_mut(),
    // };

    // let e = E{
    //     mut_ref: b.as_deref_mut(),
    // };
    let g = G{
        mut_ref: b.as_deref_mut()
    };

    // let c = match b.as_deref_mut() {
    //     None => C { mut_ref: None },
    //     Some(x) => C { mut_ref: Some(x) },
    // };
}
