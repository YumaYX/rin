use std::net::Ipv4Addr;

fn apply_mask(ip: Ipv4Addr, mask: u8) -> Ipv4Addr {
    if mask == 0 {
        return Ipv4Addr::new(0, 0, 0, 0);
    }
    let mask = !((1 << (32 - mask)) - 1);
    let ip_as_u32 = u32::from(ip);
    Ipv4Addr::from(ip_as_u32 & mask)
}

fn main() {
    let ip = Ipv4Addr::new(255, 255, 255, 255);

    for mask in 0..=32 {
        let masked_ip = apply_mask(ip, mask);
        println!("{}\t{}", mask, masked_ip);
    }
}
