use tbf_parser::parse::*;

#[test]
fn check_sum(){
    let buffer: Vec<u8> = include_bytes!("./flashes/simple.dat").to_vec();
 
    let (ver, header_len, whole_len) = parse_tbf_header_lengths(&buffer[0..8].try_into().unwrap()).ok().unwrap();
    assert_eq!(ver, 2);
    assert_eq!(header_len, 52);
    assert_eq!(whole_len, 8192);
    
    let header = parse_tbf_header(&buffer[0..header_len as usize],2).unwrap();    
    dbg!(&header);
    assert_eq!(header.enabled(), true);
    assert_eq!(header.get_minimum_app_ram_size(), 4848);
    assert_eq!(header.get_init_function_offset(), 41 + header_len as u32);
    assert_eq!(header.get_protected_size(), header_len as u32);
    assert_eq!(header.get_package_name().unwrap(), "_heart");
    assert_eq!(header.get_kernel_version().unwrap(), (2,0));
}