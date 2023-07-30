use tbf_parser::{parse::*, types::TbfHeader};

fn get_test<'a>() -> TbfHeader<'a> {
    let mut buffer: Vec<u8> = include_bytes!("./flashes/simple.dat").to_vec();
 
    let (ver, header_len, whole_len) = parse_tbf_header_lengths(&buffer[0..8].try_into().unwrap()).ok().unwrap();
    dbg!(ver, header_len, whole_len);
    
    let header = parse_tbf_header(&buffer[0..header_len as usize],2).unwrap();    
    return header;
}

#[test]
fn check_sum(){
    let header = get_test();
    dbg!(header);   
}