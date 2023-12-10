use std::io::Write;

fn main(){

    let mut file1 = std::fs::File::create("lager.txt").expect("create failed");
    file1.write_all("
                    33 Export
                    Desperados
                    Goldberg
                    Gulder
                    Heineken
                    Star".as_bytes()).expect("create failed");

    let mut file2 = std::fs::File::create("Stout.txt").expect("create failed");
    file2.write_all("
                    Legend
                    Turbo King
                    Williams".as_bytes()).expect("create failed");

    let mut file3 = std::fs::File::create("Non-Alcholic.txt").expect("create failed");
    file3.write_all("
                     Maltina
                     Amstel Malta 
                     Malta Gold
                     Fayrouz".as_bytes()).expect("create failed");

    println!("Files successfully created");
}
