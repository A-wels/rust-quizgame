// function that gets the local ip address of the machine and generates a qr code based on that
use qrcode_generator::QrCodeEcc;
use local_ip_address::local_ip;


pub fn generate_qr(game_id: &str) -> String{
    let local_ip = local_ip().unwrap().to_string();
    let port = 8000;
    let url = format!("http://{}:{}/?gameID={}", local_ip,port, game_id);
    let result: String = qrcode_generator::to_svg_to_string(url, QrCodeEcc::High, 512, None::<&str>).unwrap();
    return result
}

