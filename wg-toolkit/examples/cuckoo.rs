use wgtk::util::cuckoo::CuckooContext;


fn main() {

    let ctx = CuckooContext::new(943718, b"8352B49284AD04180");
    println!("solution: {:?}", ctx.work_bw());

}
