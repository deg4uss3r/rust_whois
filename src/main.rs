use std::net::{TcpStream, ToSocketAddrs};
use std::env::args;
use std::io::prelude::*;

//TODO split input for TLD (COM, NET) ask the server which to query next, and then query for the right response


fn who_is_domain(domain: &String) -> String {

    let mut b: &[u8] = domain.as_bytes();
    let new_return: &[u8] = "\r\n".as_bytes();
    let mut whois_final = String::new();
    let whois_top_level = "whois.iana.org:43";
    let mut stream = TcpStream::connect(&whois_top_level).unwrap();  //this returns what whois server to query based on tld (COM, ORG, EDU, etc)

    let test = stream.write(b);
    let nr = stream.write(new_return);
    let mut response = String::new();
    stream.read_to_string(&mut response).unwrap();

    let block_test = response.split("\n\n");
    let vec: Vec<&str> = block_test.collect();

    for s in vec{
        if s.len() >= 6{
            if &s[..5] == "refer"{
                whois_final = s.to_string();
            }
            else {
                continue;
            }
        } 
        else {
            continue;
        }
    }

    if whois_final.is_empty(){
        let error_msg = "There was an error with your request, please check your domain name";
        error_msg.to_string()
    }
    else {
        let refer_domain = &whois_final[14..];
        refer_domain.to_string()
    }


}

fn who_is_result(whois_dom: String, domain: &String){

    let mut c: &[u8] = &domain.as_bytes();
    let mut new_return: &[u8] = "\r\n".as_bytes();
    let server_details = whois_dom+":43";

    let mut stream = TcpStream::connect(&*server_details).unwrap();
    
    let test = stream.write(c);
    let nr = stream.write(new_return);
    let mut response = String::new();
    
    stream.read_to_string(&mut response).unwrap();
    println!("{}", response);

}

fn main() {

    let mut argus: Vec<_> = args().collect();

    if argus.len() == 2 {
        let org = who_is_domain(&argus[1]);
        who_is_result(org, &argus[1]);

    }
    else{

        println!("supply a domain name!");

    }

}
