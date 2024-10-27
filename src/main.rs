use sha256::digest;

extern crate tokio;

mod args;

fn to_sha256(value: u32) -> String {
    let sha256_string = digest(value.to_string());
    sha256_string
}

fn find_zeroes(zeroes: u32, count: u32){
    let mut found_counter = count as i32;
    let mut index: u32 = 0;
    let zeroes_offset: usize = 64 - zeroes as usize;
    let str_compare = "0".repeat(zeroes as usize);
    loop{
        let index_sha256 = to_sha256(index);
        match &index_sha256[zeroes_offset..64]{
            check if check == str_compare => {
                println!("{:?}, {:?}", index, index_sha256);
                found_counter -= 1;
                if found_counter <= 0{
                    break;
                }
            },
            _ => {},
        }
        index += 1;
    }
}

fn main(){
    let args = args::get_args();
    // args.0 == "N" value
    // args.1 == "F" value
    find_zeroes(args.0, args.1);
}
