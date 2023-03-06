use std::env;
use std::process;
use std::io;

fn main() {
    let args: Vec<String> = env::args().collect();

    let degree = input_checker(&args).unwrap_or_else( |err| {
        eprintln!("Problem while taking input: {err}");
        process::exit(1);
    });

    println!("{degree}");
}

fn input_checker(args: &[String]) -> Result<String, &'static str> {
    if args.len() < 2 {
        return input_reminder(); //Temporary
    } else if args.len() > 2 {
        return Err("Too many arguments!");
    } else {
        let degree = args[1].clone();
        Ok(degree)
    }
}

fn input_reminder() -> Result <String, &'static str> {
    println!("You didn't enter a value as a command line argument.
What is the number you want to convert?");
    let mut value = String::new();
    io::stdin()
        .read_line(&mut value)
        .expect("Could not read from Command Line.");
    Ok(value.trim().to_string())
}

//1. Program CL argümanlarını alsın. Eğer yeterli CL argümanı yoksa 2. adıma geçsin, yeterli CL argümanı varsa 3. adıma geçsin, fazla argüman varsa hata versin. ++
//2. Eğer CL'den sayı alınmadıysa uyarı geçsin ve yine de sayıyı sorsun. ++
//3. Program argümanı parse'lasın. Eğer girilen sayı parse'lanamıyorsa paniklesin. Ayrıca bu adımda artık lib.rs'yi kullanmaya başlayabiliriz.
//3.1 Bu adımda notta bahsettiğimiz struct ve enum oluşturulabilir, Envvar'lar sorgulanabilir. Envvar alınıp alınmadığı programda struct oluşturulmadan önce teyit edilmeli.
//4. Parse'lanan sayının C -> F ve F -> C değerleri yazdırılsın. Eğer FC veya CF envvarları kullanılmışsa yalnızca ilgili işlemi bastırsın.
//Not: Sayı değeri ve CF/FC envvarı kullanılıp kullanılmaması bir struct'ta birleştirilebilir. Envvar değerinin CF veya FC olması enum olabilir.
