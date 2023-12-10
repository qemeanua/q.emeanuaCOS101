use std::io::Write;

fn main() {

    let name_of_commisioner = vec!["Aigbogun Alamba Daudu     ", "Murtala AfeezBendu       ", "Okorocha Calistus Ogbona ", "Adewale Jimoh Akanbi     ", "Osazuwa Faith Etieye     "];
    let ministry = vec![ "Internal Affairs  ", " Justice           ", " Defense         ", " Power & Steel     ", " Petroleum       "];
    let geopolitical_zone = vec![" South West ", " North East  ", " South South ", " South West ", " South East "];

   

    let mut file = std::fs::File::create("cloud backup system dataset.txt").expect("failed");
    file.write_all("                CONVICTED MINISTERS     ".as_bytes()).expect("failed");
    file.write_all("\nNAME OF COMMISIONER        MINISTRY        GEOPOLTICAL ZONE".as_bytes()).expect("failed");
  
    for i in 0..name_of_commisioner.len(){
    file.write_all("\n".as_bytes()).expect("failed");

    file.write_all(  name_of_commisioner[i].as_bytes()).expect("failed");

    file.write_all(  ministry[i].as_bytes()).expect("failed");

    file.write_all(  geopolitical_zone[i].as_bytes()).expect("failed");

}
  println!("\nData set  of convicted minsters from different geopolitical zones created successfully");  
}

