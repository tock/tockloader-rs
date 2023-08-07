use tbf_parser::parse::*;

#[test]
fn simple_tbf() {
    let buffer: Vec<u8> = include_bytes!("./flashes/simple.dat").to_vec();

    let (ver, header_len, whole_len) = parse_tbf_header_lengths(&buffer[0..8].try_into().unwrap())
        .ok()
        .unwrap();
    assert_eq!(ver, 2);
    assert_eq!(header_len, 52);
    assert_eq!(whole_len, 8192);

    let header = parse_tbf_header(&buffer[0..header_len as usize], 2).unwrap();
    dbg!(&header);
    assert_eq!(header.enabled(), true);
    assert_eq!(header.get_minimum_app_ram_size(), 4848);
    assert_eq!(header.get_init_function_offset(), 41 + header_len as u32);
    assert_eq!(header.get_protected_size(), header_len as u32);
    assert_eq!(header.get_package_name().unwrap(), "_heart");
    assert_eq!(header.get_kernel_version().unwrap(), (2, 0));
}

#[test]
fn footer_sha256() {
    let buffer: Vec<u8> = include_bytes!("./flashes/footerSHA256.dat").to_vec();

    let (ver, header_len, whole_len) = parse_tbf_header_lengths(&buffer[0..8].try_into().unwrap())
        .ok()
        .unwrap();
    assert_eq!(ver, 2);
    assert_eq!(header_len, 76);
    assert_eq!(whole_len, 8192);

    let header = parse_tbf_header(&buffer[0..header_len as usize], 2).unwrap();
    dbg!(&header);
    assert_eq!(header.enabled(), true);
    assert_eq!(header.get_minimum_app_ram_size(), 4848);
    assert_eq!(header.get_init_function_offset(), 41 + header_len as u32);
    assert_eq!(header.get_protected_size(), header_len as u32);
    assert_eq!(header.get_package_name().unwrap(), "_heart");
    assert_eq!(header.get_kernel_version().unwrap(), (2, 0));
    let binary_offset = header.get_binary_end() as usize;
    assert_eq!(binary_offset, 5836);

    let (footer, footer_size) = parse_tbf_footer(&buffer[binary_offset..]).unwrap();
    dbg!(footer);
    assert_eq!(footer_size, 36);
    let correct_sha256 = [
        214u8, 17, 81, 32, 51, 178, 249, 35, 161, 33, 109, 184, 195, 46, 238, 158, 141, 54, 63, 94,
        60, 245, 50, 228, 239, 107, 231, 127, 220, 158, 77, 160,
    ];
    assert_eq!(footer.data(), correct_sha256);
}
