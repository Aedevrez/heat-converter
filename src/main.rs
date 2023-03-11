use std::env;
use std::process;

use heat_converter::*;

fn main() {
    let args: Vec<String> = env::args().collect();

    let degree = input_checker(&args).unwrap_or_else( |err| {
        eprintln!("Problem while taking input: {err}");
        process::exit(1);
    });

    let degree: i64 = degree_parser(&degree);

    run(degree);
}

//1. Program CL argümanlarını alsın. Eğer yeterli CL argümanı yoksa 2. adıma geçsin, yeterli CL argümanı varsa 3. adıma geçsin, fazla argüman varsa hata versin. ++
//2. Eğer CL'den sayı alınmadıysa uyarı geçsin ve yine de sayıyı sorsun. ++
//3. Program argümanı parse'lasın. Eğer girilen sayı parse'lanamıyorsa paniklesin. Ayrıca bu adımda artık lib.rs'yi kullanmaya başlayabiliriz.
//3.1 Bu adımda notta bahsettiğimiz struct ve enum oluşturulabilir, Envvar'lar sorgulanabilir. Envvar alınıp alınmadığı programda struct oluşturulmadan önce teyit edilmeli.
//3.2 lib.rs'yi oluştur. ++
//3.3 Degree struct'ını oluştur. ++
//3.4 degree_parser ++ ve Degree::build fonksiyonlarını yaz. ++
//4. Parse'lanan sayının C -> F ve F -> C değerleri yazdırılsın. Eğer FC veya CF envvarları kullanılmışsa yalnızca ilgili işlemi bastırsın. Bu aşama artık run fonksiyonu içerisinde yapılsın. ++
//Not: Sayı değeri ve CF/FC envvarı kullanılıp kullanılmaması bir struct'ta birleştirilebilir. Envvar değerinin CF veya FC olması enum olabilir. ++
//5. Degree parser fonksiyonunda hata ayıklama daha kullanıcı dostu bir şekilde yapılabilir.
