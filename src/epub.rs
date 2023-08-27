use slint::SharedString;
use rfd;

pub fn gen(inpath:SharedString,outpath:SharedString)->i32{
    let m=rfd::MessageDialog::new().set_title("提示");
     m.show();
     return  1;
}