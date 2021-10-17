use std::cell::RefCell;

fn main() {
    let ref_cell = RefCell::new("hello".to_string());

    {
        let r = ref_cell.borrow();
        let count = r.len();
        assert_eq!(count, 5);
    }

    // will panic here, because ref_cell is already borrowed
    // let mut w = ref_cell.borrow_mut();
    // w.push_str(" world!");

    // solve way: move the above code into different block
    {
        let mut w = ref_cell.borrow_mut();
        w.push_str(" world!");
        println!("{}", w);
    }
}
