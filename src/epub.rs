use slint::SharedString;
use rfd;
use super::EpubSetting;



pub fn gen(s:EpubSetting)->i32{
    let m=
    rfd::MessageDialog::new()
    .set_title("提示")
    .set_description(s.inpath.as_str());
     m.show();
     return  1;
}